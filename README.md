# MoonMesh

**Decentralized Self-Healing Overlay Network for Rust.**

MoonMesh is a P2P-first encrypted networking substrate. VPN is one capability exposed by the network, not the architectural boundary.

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
| encrypted tunnels         |
| packet forwarding         |
+-------------+-------------+
              |
+-------------v-------------+
| Control Plane             |
| identity                   |
| peer management            |
| discovery                  |
| DHT + gossip              |
| connectivity / NAT        |
| topology + routing        |
| policy                    |
+---------------------------+
```

### Design rules

1. **P2P-first, server-optional.** No mandatory central VPN controller.
2. **Identity-centric.** Cryptographic peer identity is independent of changing network endpoints.
3. **Direct-first connectivity.** Prefer direct paths; use NAT traversal and relay only as required.
4. **Separation of planes.** Discovery/control must not become packet forwarding by accident.
5. **No custom cryptography.** Reuse established cryptographic/tunnel protocols and libraries.
6. **Self-healing by design.** Topology and path selection must react to peer and link failure.
7. **Small stable contracts.** Crates own one responsibility and communicate through explicit types/protocols.

## Workspace

- `moonmesh-core` — shared primitives, identifiers, errors, configuration contracts
- `moonmesh-identity` — keys and peer identity lifecycle
- `moonmesh-protocol` — wire/control message definitions and versioning
- `moonmesh-peer` — peer state, lifecycle, capabilities, endpoint knowledge
- `moonmesh-discovery` — local/bootstrap/discovery interfaces
- `moonmesh-routing` — topology model, path metrics, route selection
- `moonmesh-connectivity` — endpoint negotiation, NAT traversal orchestration
- `moonmesh-tunnel` — encrypted data-plane abstraction
- `moonmesh-relay` — optional relay role and fallback transport
- `moonmesh-policy` — route/access/role policy
- `moonmesh-node` — composition/runtime for a MoonMesh node
- `moonmesh-cli` — operator-facing command line

Not every crate is implemented in v0.1. The workspace is the architectural boundary; functionality lands incrementally.

## Roadmap

`M0 Architecture → M1 Identity → M2 Secure Peer Link → M3 Two-Node Mesh → M4 Discovery → M5 NAT Traversal → M6 Relay → M7 DHT → M8 Gossip → M9 Topology → M10 Adaptive Routing → M11 Self-Healing → M12 Multi-hop → M13 Exit/Gateway → M14 OS VPN Interface → M15 Observability → M16 Security Hardening → M17 Cross-platform → M18 Production`

See `docs/architecture.md` and `docs/roadmap.md`.

## Status

**v0.1 architecture foundation.** The repository is intentionally being built from stable boundaries upward. Production claims are not made until end-to-end interoperability, security, failure recovery, and cross-platform tests exist.

## License

MIT
