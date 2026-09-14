# MoonMesh Architecture Contract v0.1

## 1. Scope

MoonMesh is a decentralized encrypted overlay-network engine. It may expose a host VPN interface, but the core system is the overlay fabric rather than a conventional VPN server.

## 2. Planes

### Control plane
Responsible for identity, peer lifecycle, endpoint knowledge, discovery, topology, connectivity negotiation, routing state, and policy.

### Data plane
Responsible for encrypted packet transport and forwarding over established paths.

A control-plane component MUST NOT silently become a traffic proxy. A data-plane component MUST NOT invent peer-discovery semantics.

## 3. Crate ownership

| Crate | Owns | Must not own |
|---|---|---|
| core | shared IDs, errors, primitives, config contracts | network behavior |
| identity | cryptographic identity lifecycle | routing/discovery |
| protocol | versioned message schemas | peer state machine |
| peer | peer records and lifecycle | global topology |
| discovery | finding peers and candidates | final path selection |
| routing | topology, metrics, routes | cryptographic identity |
| connectivity | making peer links viable | global application policy |
| tunnel | encrypted data-path abstraction | peer discovery |
| relay | optional relay behavior | authoritative topology |
| policy | roles, route/access policy | tunnel crypto |
| node | runtime composition/orchestration | domain ownership of lower-level state |
| cli | operator UX | protocol semantics |

## 4. Core objects

The first stable domain objects should be:

- `PeerId`: stable cryptographic identity reference.
- `PeerRecord`: identity + capabilities + known endpoints + liveness metadata.
- `Endpoint`: candidate transport endpoint with provenance and expiry.
- `Link`: observed/negotiated connectivity between two peers.
- `Route`: path from source to destination with explicit metrics and policy.
- `NodeRole`: endpoint/router/relay/exit/gateway capability set.
- `TopologyEvent`: versioned observation describing peer/link/topology change.

## 5. Connectivity policy

Preferred order:

1. direct path;
2. NAT traversal / hole punching;
3. alternate known endpoint;
4. relay fallback.

A relay is not required for normal operation and must be modeled as a route capability rather than the central architecture.

## 6. Discovery policy

Discovery is layered:

- local discovery for nearby candidates where available;
- explicit/bootstrap peers for initial network entry;
- DHT for distributed lookup at scale;
- gossip for dissemination of selected topology/state events.

Gossip is not the source of truth for all routing decisions. Routing consumes normalized peer/link observations.

## 7. Routing policy

Routing must be metric-driven and replaceable. Metrics can include latency, loss, bandwidth estimate, hop count, availability, trust/policy constraints, and operational cost.

The initial route selector should be deterministic for the same topology snapshot and policy inputs.

## 8. Security boundary

MoonMesh will not design novel ciphers or cryptographic primitives. Tunnel and identity implementations must rely on reviewed, established primitives/protocol implementations.

Keys are never logged in plaintext. Peer identity and endpoint metadata have explicit sensitivity classifications.

## 9. Failure model

The system assumes:

- peers disappear unexpectedly;
- endpoints change;
- links flap;
- NAT mappings expire;
- the network partitions;
- stale discovery data exists;
- relays may become unavailable.

Self-healing behavior must be driven by bounded timers, explicit state transitions, and observable events rather than hidden background magic.

## 10. Compatibility rule

Protocol versions, serialized messages, route semantics, and peer identity encodings must have explicit compatibility tests before they are treated as stable.
