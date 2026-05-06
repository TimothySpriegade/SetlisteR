use crate::secrets_manager::secrets_manager::SecretsManager;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_returns_manager() {
        // Act & Assert
        let _ = SecretsManager::new();
    }

    #[test]
    #[serial_test::serial]
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
    #[serial_test::serial]
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
    fn test_set_keys_from_args_with_empty_map_returns_ok() {
        // Arrange
        let manager = SecretsManager::new();

        // Act
        let result = manager.set_keys_from_args(HashMap::new());

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_set_and_get_keyring_secret() {
        // Arrange
        unsafe { std::env::remove_var("SETLIST_FM_API_KEY") };
        let manager = SecretsManager::new();
        let mut keys = HashMap::new();
        keys.insert(
            crate::secrets_manager::secrets_manager::KeyType::SetlistFmApiKey,
            "test-api-key-from-keyring".to_string(),
        );

        // Act
        let set_result = manager.set_keys_from_args(keys);
        let get_result = manager.get_setlist_fm_api_key();

        // Assert
        assert!(set_result.is_ok());
        assert_eq!(get_result, Some("test-api-key-from-keyring".to_string()));
    }
}
