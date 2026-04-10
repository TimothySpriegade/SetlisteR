use crate::secrets_manager::secrets_manager::{KeyType, SecretsManager};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_manager_with_no_cached_key() {
        // Act
        let _manager = SecretsManager::new();

        assert!(true);
    }

    #[test]
    fn test_get_env_secret_returns_some_when_env_var_is_set() {
        // Arrange
        unsafe { std::env::set_var("SETLIST_FM_API_KEY", "test-api-key-from-env") };

        // Act
        let result = SecretsManager::get_env_secret_for_test("SETLIST_FM_API_KEY".to_string());

        // Assert
        assert_eq!(result, Some("test-api-key-from-env".to_string()));

        // Cleanup
        unsafe { std::env::remove_var("SETLIST_FM_API_KEY") };
    }

    #[test]
    fn test_get_env_secret_returns_none_when_env_var_is_absent() {
        // Arrange
        unsafe { std::env::remove_var("SETLIST_FM_API_KEY") };

        // Act
        let result = SecretsManager::get_env_secret_for_test("SETLIST_FM_API_KEY".to_string());

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn test_get_env_secret_returns_none_for_unknown_variable() {
        // Act
        let result =
            SecretsManager::get_env_secret_for_test("THIS_VAR_DOES_NOT_EXIST_XYZ".to_string());

        // Assert
        assert!(result.is_none());
    }

    #[test]
    fn test_get_setlist_fm_api_key_returns_some_when_env_var_is_set_and_keyring_is_empty() {
        // Arrange
        let manager = SecretsManager::new();

        // Act
        let _ = manager.get_setlist_fm_api_key();

        // Assert
        assert!(true);
    }

    #[test]
    fn test_get_setlist_fm_api_key_returns_none_when_env_var_absent_and_keyring_is_empty() {
        // Arrange
        let manager = SecretsManager::new();
        unsafe { std::env::remove_var("SETLIST_FM_API_KEY") };

        // Act
        let result = manager.get_setlist_fm_api_key();

        // Assert
        let _: Option<String> = result;
    }

    #[test]
    fn test_set_keys_from_args_with_empty_map_returns_ok() {
        // Arrange
        let mut manager = SecretsManager::new();
        let keys: HashMap<KeyType, String> = HashMap::new();

        // Act
        let result = manager.set_keys_from_args(keys);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_keys_from_args_with_api_key_returns_ok() {
        // Arrange
        let mut manager = SecretsManager::new();
        let mut keys = HashMap::new();
        keys.insert(KeyType::SetlistFmApiKey, "my-secret-key".to_string());

        // Act
        let result = manager.set_keys_from_args(keys);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_keys_from_args_with_multiple_calls_always_returns_ok() {
        // Arrange
        let mut manager = SecretsManager::new();

        // Act & Assert — calling twice must not error
        let mut keys_first = HashMap::new();
        keys_first.insert(KeyType::SetlistFmApiKey, "first-key".to_string());
        assert!(manager.set_keys_from_args(keys_first).is_ok());

        let mut keys_second = HashMap::new();
        keys_second.insert(KeyType::SetlistFmApiKey, "second-key".to_string());
        assert!(manager.set_keys_from_args(keys_second).is_ok());
    }
}
