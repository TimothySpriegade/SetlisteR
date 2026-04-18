use crate::data::models::setlistfm_response::SetlistResponse;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

pub struct SetlistFmClient {
    api_key: String,
    next_allowed_request: Arc<Mutex<Instant>>,
}

impl SetlistFmClient {
    pub fn new(api_key: String) -> Self {
        SetlistFmClient {
            api_key,
            next_allowed_request: Arc::new(Mutex::new(Instant::now())),
        }
    }

    pub async fn get_setlist_by_artist(
        &self,
        artist: &str,
        page_count: u16,
    ) -> Vec<Result<SetlistResponse, String>> {
        let client = reqwest::Client::new();
        let mut page = 1;
        let mut all_setlists: Vec<Result<SetlistResponse, String>> = Vec::new();

        while page <= page_count {
            let request_uri = format!(
                "https://api.setlist.fm/rest/1.0/search/setlists?artistName={}&p={}",
                artist, page
            );

            self.wait_for_slot().await;

            let response = client
                .get(&request_uri)
                .header("x-api-key", &self.api_key)
                .header("Accept", "application/json")
                .send()
                .await;

            match response {
                Ok(res) => {
                    let parsed = res
                        .json::<SetlistResponse>()
                        .await
                        .map_err(|err| format!("Error parsing JSON: {}", err));
                    all_setlists.push(parsed);
                }
                Err(err) => {
                    all_setlists.push(Err(format!("Error sending request: {}", err)));
                }
            }

            page += 1;
        }

        all_setlists
    }

    async fn wait_for_slot(&self) {
        let mut guard = self.next_allowed_request.lock().await;
        let now = Instant::now();

        if *guard > now {
            tokio::time::sleep_until((*guard).into()).await;
        }

        *guard = Instant::now() + Duration::from_millis(500); // 2 req/sec api ratelimit from setlistfm
    }
}
