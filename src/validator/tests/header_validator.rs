use crate::validator::header_validator::HeaderValidator;
use crate::validator::tests::args_mother::ArgsMotherObject;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_no_headers() {
        // Arrange
        let args = ArgsMotherObject::default().build();

        // Act
        let result = HeaderValidator::validate(&args);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }

    #[test]
    fn test_validate_with_headers() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_youtube_headers("  my_youtube_headers_data  ")
            .build();

        // Act
        let result = HeaderValidator::validate(&args);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("my_youtube_headers_data".to_string()));
    }

    #[test]
    fn test_validate_stdin_headers() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_youtube_headers("-")
            .build();
        let input_data = b"  raw_pasted_headers_from_stdin  ";

        // Act
        let result = HeaderValidator::validate_internal(&args, &input_data[..]);

        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Some("raw_pasted_headers_from_stdin".to_string())
        );
    }

    #[test]
    fn test_validate_stdin_headers_error() {
        // Arrange
        let args = ArgsMotherObject::default()
            .with_youtube_headers("-")
            .build();

        struct FailingReader;
        impl std::io::Read for FailingReader {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Simulated read failure",
                ))
            }
        }

        // Act
        let result = HeaderValidator::validate_internal(&args, FailingReader);

        // Assert
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Failed to read headers from stdin");
    }
}
