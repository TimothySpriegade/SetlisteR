use std::collections::HashMap;
use std::env;

pub struct SecretsManager;

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum KeyType {
    SetlistFmApiKey,
}

const KEYRING_SERVICE: &str = "SetlisteR";
const SETLIST_KEYRING_USER: &str = "setlist_fm_api_key";
const SETLIST_FM_API_KEY_ENV_VAR: &str = "SETLIST_FM_API_KEY";

impl SecretsManager {
    pub fn new() -> SecretsManager {
        SecretsManager
    }

    pub fn set_keys_from_args(&self, keys: HashMap<KeyType, String>) -> Result<(), ()> {
        for (key_type, key_value) in keys {
            match key_type {
                KeyType::SetlistFmApiKey => {
                    Self::set_keyring_secret(KEYRING_SERVICE, SETLIST_KEYRING_USER, key_value);
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

    fn set_keyring_secret(keyring_service: &str, keyring_user: &str, key: String) {
        match keyring::Entry::new(keyring_service, keyring_user) {
            Ok(entry) => match entry.set_password(&key) {
                Ok(_) => println!("Secret successfully configured for service {keyring_service}"),
                Err(e) => eprintln!("[keyring] set_password failed: {e}"),
            },
            Err(e) => eprintln!("[keyring] Entry::new failed: {e}"),
        }
    }

    fn get_keyring_secret(keyring_service: &str, keyring_user: &str) -> Option<String> {
        match keyring::Entry::new(keyring_service, keyring_user) {
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
