use moonmesh_core::PeerId;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PeerStatus {
    Discovered,
    Connecting,
    Connected,
    Unreachable,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Endpoint {
    pub address: String,
    pub source: EndpointSource,
    pub expires_at_unix_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EndpointSource {
    Manual,
    LocalDiscovery,
    Bootstrap,
    Observed,
    Relay,
}

impl Endpoint {
    pub fn is_expired_at(&self, now_unix_ms: u64) -> bool {
        self.expires_at_unix_ms <= now_unix_ms
    }

    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis() as u64;
        self.is_expired_at(now)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerRecord {
    pub id: PeerId,
    pub status: PeerStatus,
    pub endpoints: Vec<Endpoint>,
    pub last_seen_unix_ms: Option<u64>,
}

impl PeerRecord {
    pub fn new(id: PeerId) -> Self {
        Self {
            id,
            status: PeerStatus::Discovered,
            endpoints: Vec::new(),
            last_seen_unix_ms: None,
        }
    }

    pub fn live_endpoints_at(&self, now_unix_ms: u64) -> impl Iterator<Item = &Endpoint> {
        self.endpoints
            .iter()
            .filter(move |endpoint| !endpoint.is_expired_at(now_unix_ms))
    }
}
