pub struct HeaderValidator;

impl HeaderValidator {
    pub fn validate(args: &crate::Args) -> Result<Option<String>, String> {
        Self::validate_internal(args, std::io::stdin())
    }

    pub(crate) fn validate_internal<R: std::io::Read>(
        args: &crate::Args,
        mut reader: R,
    ) -> Result<Option<String>, String> {
        if let Some(headers) = &args.youtube_headers {
            if headers == "-" {
                println!(
                    "Reading YouTube headers from stdin. Paste your headers and press Ctrl+D (or Ctrl+Z on Windows) when done:"
                );
                let mut buffer = String::new();
                if reader.read_to_string(&mut buffer).is_ok() {
                    Ok(Some(buffer.trim().to_string()))
                } else {
                    Err("Failed to read headers from stdin".to_string())
                }
            } else {
                Ok(Some(headers.trim().to_string()))
            }
        } else {
            Ok(None)
        }
    }
}
