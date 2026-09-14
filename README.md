# MoonMesh

**Decentralized Self-Healing Overlay Network for Rust.**

MoonMesh is a P2P-first encrypted networking substrate. A VPN interface is one capability exposed by the network, not the architectural boundary.

## Vision

Turn ordinary devices into a self-organizing encrypted network that can discover peers, establish direct connectivity when possible, select viable paths, recover from failures, and operate without a mandatory central controller.

## Architecture

```text
Application / OS
       |
   moon0 / API
       |
+---------------------------+
| Data Plane                |
| encrypted tunnel          |
| packet forwarding         |
+-------------+-------------+
              |
+-------------v-------------+
| Control Plane             |
| identity                   |
| peer management            |
| discovery                  |
| DHT + bounded gossip      |
| connectivity / NAT        |
| topology + routing        |
| policy                    |
+---------------------------+
```

### Design rules

1. **P2P-first, server-optional.** No mandatory central VPN controller or packet-forwarding service.
2. **Identity-centric.** Cryptographic peer identity is independent of changing network endpoints.
3. **Direct-first connectivity.** Prefer direct paths; use NAT traversal and relay only when required.
4. **Separation of planes.** Discovery/control must never silently become packet forwarding.
5. **No custom cryptography.** Reuse established cryptographic/tunnel protocols and reviewed implementations.
6. **Self-healing by design.** Topology and route selection react to peer/link failure.
7. **Small stable contracts.** Each crate owns one responsibility and communicates through explicit types/protocols.
8. **Privileged code stays isolated.** OS VPN/TUN/TAP integration is kept away from portable protocol and routing logic.

## Workspace

The repository starts with the four buildable foundation crates; additional architectural crates land when their contracts have an implementation need.

- `moonmesh-core` — identifiers, errors and shared primitives
- `moonmesh-identity` — cryptographic node identity lifecycle
- `moonmesh-protocol` — versioned wire/control messages
- `moonmesh-peer` — peer state and capabilities

Planned boundaries are documented for discovery, connectivity, routing, tunnel, relay, policy, node runtime and CLI. See `docs/architecture.md`.

## Fast-track roadmap

```text
M0 Architecture / CI
   |
M1 Identity
   |
M2 Secure Peer Link
   |
M3-M4 Two-node Mesh + Discovery
   |
M5-M6 NAT + Relay
   |
M7-M8 DHT + Bounded Gossip
   |
M9-M11 Topology + Adaptive Routing + Self-Healing
   |
M12-M18 Multi-hop + Gateway + VPN + Security + Release
```

See `docs/roadmap.md` and `docs/issue-map.md` for the dependency spine and parallel work tracks. GitHub Issues are the source of executable work.

## Automation

Every pull request and push to `main` runs:

```text
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

A scheduled security workflow runs `cargo audit`. Dependabot tracks Cargo and GitHub Actions updates. Version tags create **draft** GitHub Releases only after workspace checks pass.

These gates are intentionally conservative while MoonMesh is pre-release.

## Development

```text
cargo fmt --all
cargo check --workspace --all-targets
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
```

## Security

MoonMesh handles cryptographic identities and packet traffic. Treat all network input as hostile, never commit secrets, and follow `SECURITY.md` for vulnerability reporting.

## Status

**Pre-release — architecture foundation.** Production readiness is not claimed until end-to-end interoperability, security, NAT traversal, failure recovery and cross-platform tests pass.

## License

MIT
