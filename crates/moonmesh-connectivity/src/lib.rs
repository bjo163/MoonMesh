use moonmesh_core::PeerId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConnectivityState {
    Unknown,
    Probing,
    Direct,
    Traversed,
    Relayed,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PathKind {
    Direct,
    NatTraversal,
    Relay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LinkCandidate {
    pub peer: PeerId,
    pub path: PathKind,
    pub endpoint: Option<String>,
}

pub trait ConnectivityManager: Send + Sync {
    fn state(&self, peer: &PeerId) -> ConnectivityState;
    fn candidates(&self, peer: &PeerId) -> Vec<LinkCandidate>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direct_path_is_distinct_from_relay() {
        assert_ne!(PathKind::Direct, PathKind::Relay);
        assert_ne!(ConnectivityState::Direct, ConnectivityState::Relayed);
    }
}
