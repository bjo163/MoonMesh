use moonmesh_core::PeerId;

#[derive(Clone, Debug)]
pub enum ControlMessage {
    Ping { peer: PeerId },
    PeerAnnounce { peer: PeerId },
}
