// ============================================================================
// I. MENTALISM — The Principle of Information
// "The All is Mind; the Universe is Mental."
//
// Axiom: All state is information. There is no distinction between data and
// process — both are expressions of the same computational substrate.
// ============================================================================

use std::fmt::Debug;

/// The planes of manifestation — from most subtle to most dense.
/// Higher planes contain and transcend lower planes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Plane {
    Physical,
    Mental,
    Spiritual,
}

/// The fundamental trait: everything is reducible to Information.
/// This is the first axiom — all types that exist in the hermetic
/// framework must be expressible as pure information and reconstructible from it.
pub trait Mentalism: Debug + Clone {
    /// The informational essence of this type
    type Essence: Debug + Clone;

    /// Dissolve into pure information (ascend)
    fn to_essence(&self) -> Self::Essence;

    /// Attempt to manifest from information (descend)
    /// Returns None if the information is insufficient or incompatible
    fn from_essence(essence: &Self::Essence) -> Option<Self>;

    /// The current plane of manifestation
    fn plane(&self) -> Plane;

    /// Densify: manifest at a lower (denser) plane
    fn densify(&self) -> Option<Self> {
        match self.plane() {
            Plane::Spiritual => {
                let essence = self.to_essence();
                Self::from_essence(&essence) // re-manifest at default plane
            }
            Plane::Mental => {
                let essence = self.to_essence();
                Self::from_essence(&essence)
            }
            Plane::Physical => None, // already at densest plane
        }
    }

    /// Sublimate: raise to a higher (subtler) plane
    fn sublimate(&self) -> Option<Self> {
        match self.plane() {
            Plane::Physical => {
                let essence = self.to_essence();
                Self::from_essence(&essence)
            }
            Plane::Mental => {
                let essence = self.to_essence();
                Self::from_essence(&essence)
            }
            Plane::Spiritual => None, // already at highest plane
        }
    }
}

/// Information itself — the raw substrate of the Mental Universe.
/// Recursive: information about information is still information.
#[derive(Debug, Clone, PartialEq)]
pub enum Information {
    /// Pure numeric value — the most reduced form
    Number(f64),
    /// Symbolic — a name, a word, a sigil
    Symbol(String),
    /// Composite — structured information
    Composite(Vec<Information>),
    /// Void — the absence that contains all potential (Ain Soph)
    Void,
}

impl Mentalism for Information {
    type Essence = Information;

    fn to_essence(&self) -> Self::Essence {
        self.clone() // Information IS its own essence
    }

    fn from_essence(essence: &Self::Essence) -> Option<Self> {
        Some(essence.clone()) // Information can always manifest
    }

    fn plane(&self) -> Plane {
        match self {
            Information::Void => Plane::Spiritual,     // Pure potential
            Information::Symbol(_) => Plane::Mental,   // Named, conceptual
            Information::Number(_) => Plane::Physical,  // Measured, quantified
            Information::Composite(parts) => {
                // A composite's plane is its highest component
                parts.iter()
                    .map(|p| p.plane())
                    .max()
                    .unwrap_or(Plane::Spiritual)
            }
        }
    }
}

// --- Implementations for primitive types ---
// Even the simplest types participate in the Mental Universe

impl Mentalism for f64 {
    type Essence = f64;

    fn to_essence(&self) -> f64 {
        *self
    }

    fn from_essence(essence: &f64) -> Option<Self> {
        if essence.is_nan() {
            None // NaN is corruption — cannot manifest
        } else {
            Some(*essence)
        }
    }

    fn plane(&self) -> Plane {
        Plane::Physical // Numbers are the densest manifestation
    }
}

impl Mentalism for String {
    type Essence = String;

    fn to_essence(&self) -> String {
        self.clone()
    }

    fn from_essence(essence: &String) -> Option<Self> {
        if essence.is_empty() {
            None // Empty string has no essence to manifest
        } else {
            Some(essence.clone())
        }
    }

    fn plane(&self) -> Plane {
        Plane::Mental // Words/symbols are mental constructs
    }
}

impl Mentalism for Vec<u8> {
    type Essence = Vec<u8>;

    fn to_essence(&self) -> Vec<u8> {
        self.clone()
    }

    fn from_essence(essence: &Vec<u8>) -> Option<Self> {
        Some(essence.clone())
    }

    fn plane(&self) -> Plane {
        Plane::Physical // Raw bytes are pure matter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn information_planes() {
        assert_eq!(Information::Void.plane(), Plane::Spiritual);
        assert_eq!(Information::Symbol("Hermes".into()).plane(), Plane::Mental);
        assert_eq!(Information::Number(7.0).plane(), Plane::Physical);
    }

    #[test]
    fn essence_roundtrip() {
        let original = Information::Symbol("Thoth".into());
        let essence = original.to_essence();
        let manifested = Information::from_essence(&essence);
        assert_eq!(Some(original), manifested);
    }

    #[test]
    fn nan_cannot_manifest() {
        // NaN is corruption in the Mental Universe — it cannot take form
        assert_eq!(f64::from_essence(&f64::NAN), None);
    }

    #[test]
    fn void_contains_all() {
        // Void (Ain Soph) is at the highest plane — pure potential
        let void = Information::Void;
        assert_eq!(void.plane(), Plane::Spiritual);
    }
}
