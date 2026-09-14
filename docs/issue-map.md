# MoonMesh Issue Map

MoonMesh uses milestone-sized issues so independent subsystems can move in parallel without collapsing architectural boundaries.

| Issue | Track | Depends on | Parallel work |
|---|---|---|---|
| #1 | Identity lifecycle | M0 | key storage, PeerId, recovery tests |
| #2 | Secure peer link | identity contract | session abstraction, integration harness |
| #3 | Two-node mesh + discovery | #1, #2 | endpoint model, discovery adapters, fixtures |
| #4 | NAT + relay | #2, #3 | probing, hole punching, relay fallback |
| #5 | DHT + gossip | #3 | DHT lookup, bounded topology gossip |
| #6 | Routing + self-healing | #4, #5 | topology snapshots, scoring, recovery |
| #7 | Multi-hop + VPN + hardening | #6 | gateways, OS interface, observability, release |
| #8 | CI quality gates | M0 | CI maintenance can proceed independently |

## Dependency spine

`Identity -> Secure Link -> Peer/Discovery -> Connectivity -> Distributed State -> Routing -> Multi-hop/VPN -> Production`

CI, security auditing, documentation and developer tooling sit across the spine and must stay independent of runtime networking logic.

## Parallelization rules

1. Work behind interfaces before implementations exist.
2. Do not move DHT or gossip state into packet forwarding.
3. Do not let routing depend on transport-specific endpoint details.
4. Do not add a mandatory centralized coordinator.
5. Do not invent cryptographic primitives or a bespoke VPN handshake.
6. Every new subsystem gets deterministic unit/integration tests before production claims.
