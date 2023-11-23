use std::fmt;

#[derive(Debug)]
pub struct SRUNClientError {
    pub message: String,
}

impl fmt::Display for SRUNClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SRUNClientError: {}", self.message)
    }
}

impl std::error::Error for SRUNClientError {}
