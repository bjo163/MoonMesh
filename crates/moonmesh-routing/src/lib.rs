use moonmesh_core::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LinkMetrics {
    pub latency_ms: u32,
    pub loss_ppm: u32,
    pub bandwidth_kbps: u64,
    pub hop_count: u16,
    pub availability_ppm: u32,
    pub policy_cost: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinkObservation {
    pub source: PeerId,
    pub target: PeerId,
    pub metrics: LinkMetrics,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TopologySnapshot {
    links: BTreeMap<(String, String), LinkMetrics>,
}

impl TopologySnapshot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_link(mut self, observation: LinkObservation) -> Self {
        self.insert_link(observation);
        self
    }

    pub fn insert_link(&mut self, observation: LinkObservation) {
        self.links.insert(
            (observation.source.to_string(), observation.target.to_string()),
            observation.metrics,
        );
    }

    pub fn metric(&self, source: &PeerId, target: &PeerId) -> Option<&LinkMetrics> {
        self.links
            .get(&(source.to_string(), target.to_string()))
    }

    pub fn links(&self) -> impl Iterator<Item = (&(String, String), &LinkMetrics)> {
        self.links.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn peer(value: &str) -> PeerId {
        PeerId::new(value).expect("test peer id")
    }

    #[test]
    fn snapshot_replaces_same_direction_deterministically() {
        let a = peer("a");
        let b = peer("b");
        let mut snapshot = TopologySnapshot::new();
        snapshot.insert_link(LinkObservation {
            source: a.clone(),
            target: b.clone(),
            metrics: LinkMetrics {
                latency_ms: 10,
                ..Default::default()
            },
        });
        snapshot.insert_link(LinkObservation {
            source: a.clone(),
            target: b.clone(),
            metrics: LinkMetrics {
                latency_ms: 5,
                ..Default::default()
            },
        });
        assert_eq!(snapshot.metric(&a, &b).unwrap().latency_ms, 5);
        assert_eq!(snapshot.links().count(), 1);
    }
}
