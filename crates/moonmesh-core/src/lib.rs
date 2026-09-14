use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub String);

#[derive(Debug, thiserror::Error)]
pub enum CoreError {
    #[error("invalid peer id")]
    InvalidPeerId,
}
