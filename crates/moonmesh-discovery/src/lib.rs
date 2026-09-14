use moonmesh_core::PeerId;
use moonmesh_peer::Endpoint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveryResult {
    pub peer: PeerId,
    pub endpoints: Vec<Endpoint>,
}

pub trait Discovery: Send + Sync {
    fn discover(&self) -> Vec<DiscoveryResult>;
}

#[derive(Clone, Debug, Default)]
pub struct BootstrapDiscovery {
    peers: Vec<DiscoveryResult>,
}

impl BootstrapDiscovery {
    pub fn new(peers: Vec<DiscoveryResult>) -> Self {
        Self { peers }
    }
}

impl Discovery for BootstrapDiscovery {
    fn discover(&self) -> Vec<DiscoveryResult> {
        self.peers.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bootstrap_discovery_is_deterministic() {
        let peer = PeerId::new("peer-a").expect("valid peer id");
        let discovery = BootstrapDiscovery::new(vec![DiscoveryResult {
            peer: peer.clone(),
            endpoints: Vec::new(),
        }]);
        let result = discovery.discover();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].peer, peer);
    }
}
