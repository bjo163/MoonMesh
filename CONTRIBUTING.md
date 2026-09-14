# Contributing to MoonMesh

MoonMesh is a networking substrate. Changes must preserve explicit ownership between identity, protocol, peer state, discovery, connectivity, routing, tunnel, relay and policy layers.

## Fast path

1. Pick an existing issue or create a narrowly scoped one.
2. Keep one subsystem owner per change.
3. Add deterministic tests before changing behavior.
4. Run:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

5. Open a pull request with the affected subsystem and security impact stated.

## Architecture rules

- P2P-first; central services are optional helpers, never mandatory for packet forwarding.
- Peer identity is cryptographic and independent of endpoint addresses.
- Discovery learns peers; routing chooses paths; tunnel transports encrypted packets.
- Gossip disseminates bounded state; it is not an authoritative forwarding protocol.
- Established cryptographic protocols/libraries are preferred over new primitives.
- Privileged OS networking code must be isolated from portable core crates.

## Commit and PR scope

Prefer small, independently reviewable commits. Do not combine unrelated networking layers just because they share a milestone.

## Release discipline

A release tag may create a draft release automatically, but production readiness requires passing end-to-end interoperability, failure-recovery and security gates.
