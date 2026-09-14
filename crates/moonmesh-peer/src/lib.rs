use moonmesh_core::PeerId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PeerStatus {
    Discovered,
    Connecting,
    Connected,
    Unreachable,
}

#[derive(Clone, Debug)]
pub struct PeerRecord {
    pub id: PeerId,
    pub status: PeerStatus,
}
