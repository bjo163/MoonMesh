# MoonMesh Fast-Track Roadmap

## M0 — Architecture Foundation
Status: started

- repository identity and scope
- control/data plane boundary
- crate ownership contract
- initial Rust workspace
- buildable core/identity/protocol/peer contracts

## M1 — Identity

- real key generation and persistence
- stable PeerId derivation
- key rotation/recovery policy
- secure serialization

## M2 — Secure Peer Link

- transport abstraction
- authenticated handshake using established protocols/libraries
- replay/session handling
- integration tests for two local nodes

## M3 — Two-Node Mesh

- node runtime
- peer lifecycle state machine
- direct encrypted packet exchange
- deterministic local test harness

## M4 — Discovery

- bootstrap peers
- local discovery abstraction
- endpoint candidate model
- expiry/provenance rules

## M5 — NAT Traversal

- endpoint probing
- STUN/ICE-style orchestration where appropriate
- hole-punch lifecycle
- reconnect/backoff

## M6 — Relay

- optional relay role
- direct-first policy
- relay admission/capacity model
- relay failover tests

## M7 — DHT

- distributed peer lookup
- bounded cache
- bootstrap/recovery behavior
- partition tests

## M8 — Gossip

- versioned topology events
- bounded fanout
- duplicate suppression
- stale-event protection

## M9 — Topology

- link-quality model
- topology snapshots
- change propagation
- route candidate generation

## M10 — Adaptive Routing

- metric-driven path selection
- route recomputation
- policy constraints
- deterministic selector tests

## M11 — Self-Healing

- failure detection
- path invalidation
- automatic reroute
- convergence tests

## M12 — Multi-Hop

- intermediate forwarding
- hop limits
- loop prevention
- path authorization

## M13 — Exit / Gateway

- Internet exit role
- subnet gateway role
- route advertisements
- policy-controlled exposure

## M14 — VPN Interface

- platform abstraction
- Linux first
- Windows/macOS follow-up
- DNS and route management

## M15 — Observability

- structured logs
- peer/link/route diagnostics
- metrics
- support bundle

## M16 — Security Hardening

- threat model review
- fuzzing/property tests
- dependency audit
- key/material handling review

## M17 — Cross-Platform

- CI matrix
- platform-specific networking tests
- packaging

## M18 — Production

- interoperability validation
- upgrade/compatibility policy
- release automation
- documented operational model
