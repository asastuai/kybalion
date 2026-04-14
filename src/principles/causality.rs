// ============================================================================
// VI. CAUSE AND EFFECT — The Principle of Causality
// "Every cause has its effect; every effect has its cause;
//  everything happens according to Law; Chance is but a name for Law
//  not recognized."
//
// Axiom: Every state transition has a traceable causal chain.
// Non-determinism is an illusion of incomplete observation.
// The master moves from being an effect to being a cause.
//
// In cryptography:
// - The blockchain IS a causal chain — each block caused by the previous
// - Merkle trees are causal DAGs — every leaf is traceable to the root
// - Digital signatures prove causation — "this entity caused this message"
// - Zero-knowledge proofs prove causation without revealing the chain
//
// In quantum computing:
// - Entanglement IS causality across distance — measuring one causes the other
// - Decoherence is causal chain collapse — environmental causes destroy superposition
// - Quantum teleportation requires a classical causal channel alongside the quantum one
// ============================================================================

use super::mentalism::Plane;
use std::fmt;

/// Unique identifier for a causal event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CausalId(u64);

impl CausalId {
    fn from_content(content: &[u8]) -> Self {
        // Simple hash for ID — not cryptographic, just unique
        let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis
        for &byte in content {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(0x100000001b3); // FNV prime
        }
        CausalId(hash)
    }
}

impl fmt::Display for CausalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016x}", self.0)
    }
}

/// A value with full causal provenance — it knows WHERE it came from,
/// WHY it exists, and WHAT it has caused.
#[derive(Debug, Clone)]
pub struct Causal<T: fmt::Debug + Clone> {
    pub value: T,
    pub id: CausalId,
    pub origin: Origin,
    pub plane: Plane,
    pub depth: u32, // how many causes deep in the chain
}

#[derive(Debug, Clone)]
pub enum Origin {
    /// First Cause — the Unmoved Mover. No prior cause.
    /// In computation: user input, system initialization, external event.
    Prima {
        intent: String,
    },
    /// Derived from one or more prior causes.
    Derived {
        description: String,
        parents: Vec<CausalId>,
    },
}

impl<T: fmt::Debug + Clone> Causal<T> {
    /// Originate a new causal chain — the First Cause.
    /// "In the beginning was the Word, and the Word was with God."
    pub fn originate(value: T, intent: &str) -> Self {
        let id = CausalId::from_content(intent.as_bytes());
        Self {
            value,
            id,
            origin: Origin::Prima {
                intent: intent.to_string(),
            },
            plane: Plane::Mental,
            depth: 0,
        }
    }

    /// Propagate: cause produces effect. The chain extends.
    /// The new value is causally linked to this one.
    pub fn propagate<U: fmt::Debug + Clone>(
        &self,
        f: impl FnOnce(&T) -> U,
        description: &str,
    ) -> Causal<U> {
        let new_value = f(&self.value);
        let id_source = format!("{}:{}", self.id, description);
        Causal {
            value: new_value,
            id: CausalId::from_content(id_source.as_bytes()),
            origin: Origin::Derived {
                description: description.to_string(),
                parents: vec![self.id],
            },
            plane: self.plane,
            depth: self.depth + 1,
        }
    }

    /// Merge: multiple causes produce one effect.
    /// This is the causal junction — a confluence of chains.
    pub fn merge<U: fmt::Debug + Clone>(
        causes: &[&Causal<T>],
        f: impl FnOnce(&[&T]) -> U,
        description: &str,
    ) -> Causal<U> {
        let values: Vec<&T> = causes.iter().map(|c| &c.value).collect();
        let new_value = f(&values);

        let parent_ids: Vec<CausalId> = causes.iter().map(|c| c.id).collect();
        let id_source = format!(
            "merge:{}:{}",
            parent_ids.iter().map(|id| format!("{}", id)).collect::<Vec<_>>().join("+"),
            description
        );
        let max_depth = causes.iter().map(|c| c.depth).max().unwrap_or(0);

        Causal {
            value: new_value,
            id: CausalId::from_content(id_source.as_bytes()),
            origin: Origin::Derived {
                description: description.to_string(),
                parents: parent_ids,
            },
            plane: Plane::Mental,
            depth: max_depth + 1,
        }
    }

    /// Transcend: rise above the current causal plane.
    /// The master becomes cause rather than effect by moving to a higher plane.
    pub fn transcend(mut self, target: Plane) -> Self {
        self.plane = target;
        self
    }

    /// Is this a First Cause (no parents)?
    pub fn is_prima(&self) -> bool {
        matches!(self.origin, Origin::Prima { .. })
    }

    /// Get parent IDs (empty for Prima)
    pub fn parent_ids(&self) -> Vec<CausalId> {
        match &self.origin {
            Origin::Prima { .. } => vec![],
            Origin::Derived { parents, .. } => parents.clone(),
        }
    }
}

impl<T: fmt::Debug + Clone> fmt::Display for Causal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin_str = match &self.origin {
            Origin::Prima { intent } => format!("Prima(\"{}\")", intent),
            Origin::Derived { description, parents } => {
                format!(
                    "Derived(\"{}\", from {} cause{})",
                    description,
                    parents.len(),
                    if parents.len() == 1 { "" } else { "s" }
                )
            }
        };
        write!(
            f,
            "[{}] depth={} plane={:?} origin={}",
            self.id, self.depth, self.plane, origin_str
        )
    }
}

// ============================================================================
// CAUSAL CHAIN — The ledger of causation
// ============================================================================

/// A CausalChain records the complete history of causation.
/// This is the blockchain before blockchain — an immutable sequence of
/// causal events, each linked to its predecessors.
#[derive(Debug)]
pub struct CausalChain {
    events: Vec<CausalEvent>,
}

#[derive(Debug, Clone)]
pub struct CausalEvent {
    pub id: CausalId,
    pub description: String,
    pub parents: Vec<CausalId>,
    pub depth: u32,
    pub plane: Plane,
}

impl CausalChain {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Record a causal event
    pub fn record<T: fmt::Debug + Clone>(&mut self, causal: &Causal<T>) {
        let (description, parents) = match &causal.origin {
            Origin::Prima { intent } => (intent.clone(), vec![]),
            Origin::Derived {
                description,
                parents,
            } => (description.clone(), parents.clone()),
        };

        self.events.push(CausalEvent {
            id: causal.id,
            description,
            parents,
            depth: causal.depth,
            plane: causal.plane,
        });
    }

    /// Trace the full lineage of an event back to its First Cause
    pub fn trace(&self, id: CausalId) -> Vec<&CausalEvent> {
        let mut lineage = Vec::new();
        let mut current = Some(id);

        while let Some(cid) = current {
            if let Some(event) = self.events.iter().find(|e| e.id == cid) {
                lineage.push(event);
                current = event.parents.first().copied();
            } else {
                break;
            }
        }

        lineage
    }

    /// How many events in the chain?
    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Maximum causal depth
    pub fn max_depth(&self) -> u32 {
        self.events.iter().map(|e| e.depth).max().unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for CausalChain {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CausalChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Causal Chain ({} events, max depth {}):", self.len(), self.max_depth())?;
        for event in &self.events {
            let indent = "  ".repeat(event.depth as usize + 1);
            let parent_str = if event.parents.is_empty() {
                "PRIMA".to_string()
            } else {
                event
                    .parents
                    .iter()
                    .map(|p| format!("{:.8}", format!("{}", p)))
                    .collect::<Vec<_>>()
                    .join(" + ")
            };
            writeln!(
                f,
                "{}[{}] {} ← {}",
                indent,
                &format!("{}", event.id)[..8],
                event.description,
                parent_str
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn originate_is_prima() {
        let cause: Causal<i32> = Causal::originate(42, "the answer");
        assert!(cause.is_prima());
        assert_eq!(cause.depth, 0);
        assert!(cause.parent_ids().is_empty());
    }

    #[test]
    fn propagation_extends_chain() {
        let first = Causal::originate(10, "seed");
        let second = first.propagate(|x| x * 2, "doubled");
        let third = second.propagate(|x| x + 1, "incremented");

        assert_eq!(second.value, 20);
        assert_eq!(third.value, 21);
        assert_eq!(second.depth, 1);
        assert_eq!(third.depth, 2);
        assert!(!second.is_prima());
    }

    #[test]
    fn ids_are_deterministic() {
        let a = Causal::originate(42, "the answer");
        let b = Causal::originate(42, "the answer");
        assert_eq!(a.id, b.id); // same intent = same ID
    }

    #[test]
    fn different_intents_different_ids() {
        let a = Causal::originate(42, "the answer");
        let b = Causal::originate(42, "another answer");
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn merge_multiple_causes() {
        let a = Causal::originate(3, "three");
        let b = Causal::originate(4, "four");

        let merged = Causal::merge(
            &[&a, &b],
            |values| values.iter().map(|&&v| v).sum::<i32>(),
            "sum of causes",
        );

        assert_eq!(merged.value, 7);
        assert_eq!(merged.depth, 1);
        assert_eq!(merged.parent_ids().len(), 2);
    }

    #[test]
    fn transcend_changes_plane() {
        let mundane = Causal::originate(42, "earthly concern");
        assert_eq!(mundane.plane, Plane::Mental);

        let transcended = mundane.transcend(Plane::Spiritual);
        assert_eq!(transcended.plane, Plane::Spiritual);
    }

    #[test]
    fn causal_chain_traces_lineage() {
        let mut chain = CausalChain::new();

        let genesis = Causal::originate(1, "genesis");
        chain.record(&genesis);

        let second = genesis.propagate(|x| x + 1, "increment");
        chain.record(&second);

        let third = second.propagate(|x| x * 3, "triple");
        chain.record(&third);

        // Trace from third back to genesis
        let lineage = chain.trace(third.id);
        assert_eq!(lineage.len(), 3);
        assert_eq!(lineage[0].depth, 2); // most recent first
        assert_eq!(lineage[2].depth, 0); // genesis last
    }

    #[test]
    fn chain_records_all_events() {
        let mut chain = CausalChain::new();

        let a = Causal::originate(1, "alpha");
        let b = Causal::originate(2, "beta");
        chain.record(&a);
        chain.record(&b);

        let c = a.propagate(|x| x + 10, "from alpha");
        chain.record(&c);

        assert_eq!(chain.len(), 3);
        assert_eq!(chain.max_depth(), 1);
    }
}
