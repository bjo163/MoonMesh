use moonmesh_core::PeerId;
use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u16 = 1;
pub const DEFAULT_CONTROL_TTL: u8 = 8;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ControlMessage {
    Ping { peer: PeerId },
    PeerAnnounce { peer: PeerId },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MessageHeader {
    pub version: u16,
    pub ttl: u8,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            version: PROTOCOL_VERSION,
            ttl: DEFAULT_CONTROL_TTL,
        }
    }

    pub fn decrement_ttl(&mut self) -> bool {
        if self.ttl == 0 {
            return false;
        }
        self.ttl -= 1;
        self.ttl > 0
    }
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_defaults_are_bounded() {
        let header = MessageHeader::new();
        assert_eq!(header.version, PROTOCOL_VERSION);
        assert_eq!(header.ttl, DEFAULT_CONTROL_TTL);
    }

    #[test]
    fn ttl_expires_deterministically() {
        let mut header = MessageHeader { version: 1, ttl: 1 };
        assert!(!header.decrement_ttl());
        assert_eq!(header.ttl, 0);
        assert!(!header.decrement_ttl());
    }
}
