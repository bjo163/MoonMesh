# MoonMesh Issue Map

MoonMesh uses milestone-sized issues so independent subsystems can move in parallel without collapsing architectural boundaries.

| Issue | Track | Depends on | Parallel work |
|---|---|---|---|
| #1 | M1 Identity lifecycle | M0 | key storage, PeerId, recovery tests |
| #2 | M2 Secure peer link | #1 | session abstraction, integration harness |
| #3 | M3-M4 Two-node mesh + discovery | #1, #2 | endpoint model, discovery adapters, fixtures |
| #4 | M5-M6 NAT + relay | #2, #3 | probing, hole punching, relay fallback |
| #5 | M7-M8 DHT + gossip | #3 | DHT lookup, bounded topology gossip |
| #6 | M9-M11 Routing + self-healing | #4, #5 | topology, scoring, recovery |
| #7 | M12-M18 Integrated production track | #6 | multi-hop, gateways, VPN, hardening |
| #8 | M0 CI quality gates | M0 | CI maintenance |
| #10 | M9 topology snapshots + metrics | #3, #4 | topology model can start with fixtures |
| #11 | M10 deterministic adaptive routing | #10 | scoring/tie-breakers |
| #12 | M11 failure detection + self-healing | #11 | failure state machine |
| #13 | M14 portable VPN interface | #6 | OS adapter contract |
| #14 | M15 structured observability | #3+ | diagnostics and redaction |
| #15 | M16 security hardening + abuse tests | #2+ | fuzzing, limits, trust-boundary review |
| #16 | M17-M18 cross-platform release | #8 + runtime readiness | matrix, artifacts, checksums |

## Dependency spine

`Identity -> Secure Link -> Peer/Discovery -> Connectivity -> Distributed State -> Topology -> Routing -> Self-Healing -> Multi-hop/VPN -> Production`

CI, security auditing, documentation and developer tooling sit across the spine and must stay independent of runtime networking logic.

## Fast-track rule

Issues #1-#5 establish network primitives. #10-#16 can be developed in parallel wherever their public contracts are already stable. Integration issues must consume those contracts rather than reaching into implementation internals.

## Boundary rules

1. Work behind interfaces before implementations exist.
2. Discovery learns peers; gossip disseminates bounded state; routing chooses paths; tunnel moves encrypted traffic.
3. Routing must not depend on transport-specific endpoint details.
4. No mandatory centralized coordinator or packet-forwarding server.
5. No custom cryptographic primitive or bespoke VPN handshake.
6. Privileged OS networking stays outside portable protocol/routing crates.
7. Every externally reachable parser/state machine gets deterministic tests before production claims.
