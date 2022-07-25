use std::error::Error;
use std::env::current_dir;

/// Prints working directory.
pub async fn handle() -> Result<String, Box<dyn Error>> {
    match current_dir() {
        Ok(b) => Ok(String::from(b.to_str().unwrap())),
        Err(e) => Ok(e.to_string())
    }
}