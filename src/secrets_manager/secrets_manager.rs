use std::collections::HashMap;
use std::env;
use std::sync::Once;

static INIT: Once = Once::new();

pub struct SecretsManager;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum KeyType {
    SetlistFmApiKey,
    YouTubeHeaders,
}

const KEYRING_SERVICE: &str = "SetlisteR";
const SETLIST_KEYRING_USER: &str = "setlist_fm_api_key";
const SETLIST_FM_API_KEY_ENV_VAR: &str = "SETLIST_FM_API_KEY";
const YOUTUBE_HEADERS_KEYRING_USER: &str = "youtube_headers";
const YOUTUBE_HEADERS_ENV_VAR: &str = "YOUTUBE_HEADERS";
const YOUTUBE_OAUTH_JSON_KEYRING_USER: &str = "youtube_oauth_json";
const YOUTUBE_OAUTH_JSON_ENV_VAR: &str = "YOUTUBE_OAUTH_JSON";

impl SecretsManager {
    pub fn new() -> SecretsManager {
        INIT.call_once(|| {
            #[cfg(test)]
            {
                if let Ok(store) = keyring_core::mock::Store::new() {
                    keyring_core::set_default_store(store);
                }
            }
            #[cfg(not(test))]
            {
                #[cfg(target_os = "macos")]
                {
                    if let Ok(store) = apple_native_keyring_store::Store::new() {
                        keyring_core::set_default_store(store);
                    }
                }
                #[cfg(target_os = "windows")]
                {
                    if let Ok(store) = windows_native_keyring_store::Store::new() {
                        keyring_core::set_default_store(store);
                    }
                }
                #[cfg(target_os = "linux")]
                {
                    if let Ok(store) = dbus_secret_service_keyring_store::Store::new() {
                        keyring_core::set_default_store(store);
                    }
                }
            }
        });
        SecretsManager
    }

    pub fn set_keys_from_args(&self, keys: HashMap<KeyType, String>) -> Result<(), ()> {
        for (key_type, key_value) in keys {
            match key_type {
                KeyType::SetlistFmApiKey => {
                    Self::set_keyring_secret(KEYRING_SERVICE, SETLIST_KEYRING_USER, key_value);
                }
                KeyType::YouTubeHeaders => {
                    Self::set_keyring_secret(
                        KEYRING_SERVICE,
                        YOUTUBE_HEADERS_KEYRING_USER,
                        key_value,
                    );
                    let _ =
                        keyring_core::Entry::new(KEYRING_SERVICE, YOUTUBE_OAUTH_JSON_KEYRING_USER)
                            .and_then(|entry| entry.delete_credential());
                    let _ = keyring_core::Entry::new(KEYRING_SERVICE, "youtube_client_id")
                        .and_then(|entry| entry.delete_credential());
                    let _ = keyring_core::Entry::new(KEYRING_SERVICE, "youtube_client_secret")
                        .and_then(|entry| entry.delete_credential());
                }
            }
        }
        Ok(())
    }

    pub fn get_setlist_fm_api_key(&self) -> Option<String> {
        match Self::get_keyring_secret(KEYRING_SERVICE, SETLIST_KEYRING_USER) {
            Some(secret) => Some(secret),
            None => Self::get_env_secret(SETLIST_FM_API_KEY_ENV_VAR.to_string()),
        }
    }

    pub fn get_youtube_headers(&self) -> Option<String> {
        match Self::get_keyring_secret(KEYRING_SERVICE, YOUTUBE_HEADERS_KEYRING_USER) {
            Some(secret) => Some(secret),
            None => Self::get_env_secret(YOUTUBE_HEADERS_ENV_VAR.to_string()),
        }
    }

    pub fn get_youtube_oauth_json(&self) -> Option<String> {
        match Self::get_keyring_secret(KEYRING_SERVICE, YOUTUBE_OAUTH_JSON_KEYRING_USER) {
            Some(secret) => Some(secret),
            None => Self::get_env_secret(YOUTUBE_OAUTH_JSON_ENV_VAR.to_string()),
        }
    }

    pub fn set_youtube_oauth_json(&self, oauth_json: String) {
        Self::set_keyring_secret(KEYRING_SERVICE, YOUTUBE_OAUTH_JSON_KEYRING_USER, oauth_json);
    }

    fn set_keyring_secret(keyring_service: &str, keyring_user: &str, key: String) {
        match keyring_core::Entry::new(keyring_service, keyring_user) {
            Ok(entry) => match entry.set_password(&key) {
                Ok(_) => println!("Secret successfully configured for service {keyring_service}"),
                Err(e) => eprintln!("[keyring] set_password failed: {e}"),
            },
            Err(e) => eprintln!("[keyring] Entry::new failed: {e}"),
        }
    }

    fn get_keyring_secret(keyring_service: &str, keyring_user: &str) -> Option<String> {
        match keyring_core::Entry::new(keyring_service, keyring_user) {
            Ok(entry) => match entry.get_password() {
                Ok(key) => Some(key),
                Err(e) => {
                    eprintln!("[keyring] get_password failed: {e}");
                    None
                }
            },
            Err(e) => {
                eprintln!("[keyring] Entry::new failed: {e}");
                None
            }
        }
    }

    fn get_env_secret(env_name: String) -> Option<String> {
        env::var(env_name).ok()
    }

    #[cfg(test)]
    pub(crate) fn get_env_secret_for_test(env_name: String) -> Option<String> {
        Self::get_env_secret(env_name)
    }
}
