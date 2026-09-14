use moonmesh_core::PeerId;

#[derive(Clone, Debug)]
pub struct Identity {
    pub peer_id: PeerId,
}

impl Identity {
    pub fn placeholder() -> Self {
        Self {
            peer_id: PeerId("uninitialized".into()),
        }
    }
}
