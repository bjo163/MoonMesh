## Summary

Describe the change and why it belongs in MoonMesh.

## Boundary check

- [ ] I changed only the subsystem owned by this PR.
- [ ] Control-plane and data-plane responsibilities remain separate.
- [ ] No mandatory central controller was introduced.
- [ ] No custom cryptographic primitive was introduced.

## Verification

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo check --workspace --all-targets`
- [ ] `cargo test --workspace --all-targets`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`

## Networking impact

State whether this changes identity, protocol, peer state, discovery, connectivity, routing, tunnel, relay, or policy behavior.

## Security impact

State any changes to trust boundaries, key material, packet handling, authentication, or privilege requirements.

## Release impact

State whether this is safe for the current development line. Do not mark production-ready without end-to-end evidence.
