use ed25519_dalek::{SigningKey, VerifyingKey};
use getrandom::{rand_core::UnwrapErr, SysRng};
use moonmesh_core::PeerId;
use std::fmt;

pub struct Identity {
    signing_key: SigningKey,
    peer_id: PeerId,
}

impl Identity {
    pub fn generate() -> Self {
        let mut rng = UnwrapErr(SysRng);
        let signing_key = SigningKey::generate(&mut rng);
        Self::from_signing_key(signing_key)
    }

    pub fn from_secret_bytes(bytes: [u8; 32]) -> Self {
        Self::from_signing_key(SigningKey::from_bytes(&bytes))
    }

    pub fn secret_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn peer_id(&self) -> &PeerId {
        &self.peer_id
    }

    fn from_signing_key(signing_key: SigningKey) -> Self {
        let peer_id = peer_id_from_public_key(&signing_key.verifying_key());
        Self {
            signing_key,
            peer_id,
        }
    }
}

impl fmt::Debug for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identity")
            .field("peer_id", &self.peer_id)
            .finish_non_exhaustive()
    }
}

fn peer_id_from_public_key(key: &VerifyingKey) -> PeerId {
    let mut encoded = String::with_capacity(16 + 64);
    encoded.push_str("ed25519:");
    for byte in key.to_bytes() {
        use fmt::Write as _;
        let _ = write!(&mut encoded, "{byte:02x}");
    }
    PeerId::new(encoded).expect("generated peer id is valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_identities_are_unique() {
        let a = Identity::generate();
        let b = Identity::generate();
        assert_ne!(a.peer_id(), b.peer_id());
    }

    #[test]
    fn restoring_secret_preserves_peer_id() {
        let identity = Identity::generate();
        let secret = identity.secret_bytes();
        let restored = Identity::from_secret_bytes(secret);
        assert_eq!(identity.peer_id(), restored.peer_id());
        assert_eq!(identity.verifying_key(), restored.verifying_key());
    }

    #[test]
    fn debug_does_not_expose_secret_material() {
        let identity = Identity::generate();
        let debug = format!("{identity:?}");
        assert!(!debug.contains("signing_key"));
        assert!(!debug.contains(&format!("{:02x?}", identity.secret_bytes())));
    }
}
