use std::time::Duration;


pub struct SetlistFmClient {
    api_key: String,
}

impl SetlistFmClient {
    pub fn new(api_key: String) -> Self {
        SetlistFmClient { api_key }
    }

    pub async fn get_setlist_by_artist(
        &self,
        artist: &str,
        page_count: u16,
    ) -> Vec<Result<serde_json::Value, String>> {
        let client = reqwest::Client::new();
        let mut page = 1;
        let mut all_setlists: Vec<Result<serde_json::Value, String>> = Vec::new();

        while page <= page_count {
            let request_uri = format!(
                "https://api.setlist.fm/rest/1.0/search/setlists?artistName={}&p={}",
                artist, page
            );

            let response = client
                .get(&request_uri)
                .header("x-api-key", &self.api_key)
                .header("Accept", "application/json")
                .send()
                .await;

            match response {
                Ok(res) => {
                    let json = res
                        .json::<serde_json::Value>()
                        .await
                        .map_err(|err| format!("Error parsing JSON: {}", err));
                    all_setlists.push(json);
                }
                Err(err) => {
                    all_setlists.push(Err(format!("Error sending request: {}", err)));
                }
            }

            // Sleep for 1 second after every 2 pages to avoid hitting rate limits of 2 requests per second, also the reason why no multithreading is used here
            if page % 2 == 0 {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            page += 1;
        }

        all_setlists
    }
}


