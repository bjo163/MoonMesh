use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PeerId(String);

impl PeerId {
    pub fn new(value: impl Into<String>) -> Result<Self, CoreError> {
        let value = value.into();
        if value.is_empty() || value.len() > 128 || !value.is_ascii() || value.chars().any(char::is_whitespace) {
            return Err(CoreError::InvalidPeerId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<PeerId> for String {
    fn from(value: PeerId) -> Self {
        value.0
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("invalid peer id")]
    InvalidPeerId,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peer_id_accepts_stable_ascii_identifier() {
        let id = PeerId::new("peer_123").expect("valid id");
        assert_eq!(id.as_str(), "peer_123");
        assert_eq!(id.to_string(), "peer_123");
    }

    #[test]
    fn peer_id_rejects_empty_whitespace_and_non_ascii() {
        for value in ["", "peer id", "café"] {
            assert_eq!(PeerId::new(value), Err(CoreError::InvalidPeerId));
        }
    }
}
