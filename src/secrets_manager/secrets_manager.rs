use std::collections::HashMap;
use std::env;

pub struct SecretsManager {
    setlist_fm_api_key: Option<String>,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum KeyType {
    SetlistFmApiKey,
}

const KEYRING_SERVICE: &str = "SetlisteR";
const SETLIST_KEYRING_USER: &str = "setlist_fm_api_key";

const SETLIST_FM_API_KEY_ENV_VAR: &str = "SETLIST_FM_API_KEY";

impl SecretsManager {
    pub fn new() -> SecretsManager {
        SecretsManager {
            setlist_fm_api_key: None,
        }
    }

    pub fn set_keys_from_args(&mut self, keys: HashMap<KeyType, String>) -> Result<(), ()> {
        for (key_type, key_value) in keys {
            match key_type {
                KeyType::SetlistFmApiKey => {
                    Self::set_keyring_secret(
                        KEYRING_SERVICE,
                        SETLIST_KEYRING_USER,
                        key_value.clone(),
                    );
                    self.setlist_fm_api_key = Some(key_value);
                }
            }
        }
        Ok(())
    }

    pub fn get_setlist_fm_api_key(&self) -> Option<String> {
        Self::get_secret(
            KEYRING_SERVICE,
            SETLIST_KEYRING_USER,
            SETLIST_FM_API_KEY_ENV_VAR.to_string(),
        )
    }

    fn set_keyring_secret(keyring_service: &str, keyring_user: &str, key: String) {
        let entry = keyring::Entry::new(keyring_service, keyring_user)
            .expect("Failed to create keyring entry");
        entry
            .set_password(&key)
            .expect("Failed to set keyring entry");

        println!("Secret successfully configured for service {keyring_service}");
    }

    fn get_secret(
        keyring_service: &str,
        keyring_user: &str,
        fallback_env_variable: String,
    ) -> Option<String> {
        match Self::get_keyring_secret(&keyring_service, &keyring_user) {
            Some(secret) => Some(secret),
            None => match Self::get_env_secret(fallback_env_variable) {
                Some(env_secret) => Some(env_secret),
                None => None,
            },
        }
    }

    fn get_keyring_secret(keyring_service: &str, keyring_user: &str) -> Option<String> {
        if let Ok(entry) = keyring::Entry::new(keyring_service, keyring_user) {
            if let Ok(key) = entry.get_password() {
                return Some(key);
            }
        }
        None
    }

    #[cfg_attr(test, allow(dead_code))]
    fn get_env_secret(env_name: String) -> Option<String> {
        env::var(env_name).ok()
    }

    #[cfg(test)]
    pub(crate) fn get_env_secret_for_test(env_name: String) -> Option<String> {
        Self::get_env_secret(env_name)
    }
}
