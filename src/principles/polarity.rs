// ============================================================================
// IV. POLARITY — The Principle of Duality
// "Everything is dual; everything has poles; everything has its pair of opposites;
//  like and unlike are the same; opposites are identical in nature, but different
//  in degree; extremes meet."
//
// Axiom: Every type has a dual. Opposites are not booleans — they are extremes
// of a continuous spectrum. The KEY insight: opposites are identical in nature,
// different only in degree. Heat and cold are both temperature. Love and hate are
// both passion. Courage and fear are both arousal.
//
// Connection to quantum computing:
// - A qubit fits this shape: |0⟩ and |1⟩ are the poles
// - Superposition is a position BETWEEN the poles
// - Measurement is forced polarization — collapsing to one pole
// - The Bloch sphere exhibits this shape: a spectral space between poles
// - Quantum gates TRANSMUTE — they move the qubit along the spectrum
//
// This principle replaces boolean logic with spectral logic.
// ============================================================================

use std::f64::consts::PI;
use std::fmt;

/// A value that exists on a polar spectrum.
/// NOT boolean. A continuous range between two extremes.
pub trait Polarity: fmt::Debug {
    /// Where on the spectrum [0.0 = full negative pole, 1.0 = full positive pole]
    fn pole_position(&self) -> f64;

    /// Move toward the opposite pole by a given degree [-1.0 to 1.0]
    /// Positive degree = toward positive pole
    /// Negative degree = toward negative pole
    fn transmute(&self, degree: f64) -> Self;

    /// The point where the nature flips (default: 0.5)
    fn transmutation_point(&self) -> f64 {
        0.5
    }

    /// Which pole dominates?
    fn polarity_sign(&self) -> PolaritySign {
        let pos = self.pole_position();
        let tp = self.transmutation_point();
        if (pos - tp).abs() < 1e-10 {
            PolaritySign::Neutral
        } else if pos > tp {
            PolaritySign::Positive
        } else {
            PolaritySign::Negative
        }
    }

    /// How strongly polarized? 0.0 = neutral, 1.0 = fully committed to a pole
    fn polarization(&self) -> f64 {
        (self.pole_position() - self.transmutation_point()).abs() * 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PolaritySign {
    Positive,
    Negative,
    Neutral, // The transmutation point — neither and both
}

// ============================================================================
// SPECTRUM VALUE — The fundamental polar type
// ============================================================================

/// A value on a named polar spectrum.
/// This is the hermetic replacement for `bool`.
///
/// Where `bool` says "true or false", `SpectralValue` says
/// "0.73 on the spectrum between darkness and light,
///  named 'illumination', with transmutation at 0.5"
///
/// In quantum terms: this is a qubit's state on the Bloch sphere
/// projected onto one axis.
#[derive(Clone)]
pub struct SpectralValue {
    /// Position on the spectrum [0.0, 1.0]
    position: f64,
    /// Name of the negative pole
    pub negative_pole: String,
    /// Name of the positive pole
    pub positive_pole: String,
    /// The nature of this spectrum (what are we measuring?)
    pub nature: String,
    /// Where transmutation occurs
    transmutation: f64,
}

impl SpectralValue {
    /// Create a new spectral value
    pub fn new(
        nature: &str,
        negative: &str,
        positive: &str,
        position: f64,
    ) -> Self {
        Self {
            position: position.clamp(0.0, 1.0),
            negative_pole: negative.to_string(),
            positive_pole: positive.to_string(),
            nature: nature.to_string(),
            transmutation: 0.5,
        }
    }

    /// Create at the exact transmutation point — the state of pure potential
    pub fn neutral(nature: &str, negative: &str, positive: &str) -> Self {
        Self::new(nature, negative, positive, 0.5)
    }

    /// Create at full positive pole
    pub fn full_positive(nature: &str, negative: &str, positive: &str) -> Self {
        Self::new(nature, negative, positive, 1.0)
    }

    /// Create at full negative pole
    pub fn full_negative(nature: &str, negative: &str, positive: &str) -> Self {
        Self::new(nature, negative, positive, 0.0)
    }

    /// Get raw position
    pub fn position(&self) -> f64 {
        self.position
    }
}

impl Polarity for SpectralValue {
    fn pole_position(&self) -> f64 {
        self.position
    }

    fn transmute(&self, degree: f64) -> Self {
        let degree = degree.clamp(-1.0, 1.0);
        let new_pos = (self.position + degree).clamp(0.0, 1.0);
        Self {
            position: new_pos,
            ..self.clone()
        }
    }

    fn transmutation_point(&self) -> f64 {
        self.transmutation
    }
}

impl fmt::Debug for SpectralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}[{:.2}] ({} ←→ {})",
            self.nature, self.position, self.negative_pole, self.positive_pole
        )
    }
}

impl fmt::Display for SpectralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self.polarity_sign() {
            PolaritySign::Positive => &self.positive_pole,
            PolaritySign::Negative => &self.negative_pole,
            PolaritySign::Neutral => "balanced",
        };
        write!(
            f,
            "{}: {:.0}% {} ({})",
            self.nature,
            self.polarization() * 100.0,
            label,
            match self.polarity_sign() {
                PolaritySign::Positive => "+",
                PolaritySign::Negative => "-",
                PolaritySign::Neutral => "=",
            }
        )
    }
}

// ============================================================================
// QUBIT — The quantum polar type
// ============================================================================

/// A Qubit represented as a hermetic polar entity.
///
/// In standard quantum computing: |ψ⟩ = α|0⟩ + β|1⟩
/// In hermetic terms: a point on the spectrum between two poles,
/// with both amplitude AND phase (position on the Bloch sphere).
///
/// |0⟩ = negative pole (position 0.0)
/// |1⟩ = positive pole (position 1.0)
/// Superposition = anywhere between
///
/// The Bloch sphere is a 3D polar spectrum: θ (polar angle) and φ (azimuthal)
/// We represent this as two spectral dimensions.
#[derive(Debug, Clone)]
pub struct Qubit {
    /// θ: angle from |0⟩ pole [0, π] → mapped to [0.0, 1.0]
    pub theta: f64,
    /// φ: phase angle [0, 2π) → the "hidden" dimension
    pub phi: f64,
}

impl Qubit {
    /// |0⟩ state — the negative pole
    pub fn zero() -> Self {
        Self {
            theta: 0.0,
            phi: 0.0,
        }
    }

    /// |1⟩ state — the positive pole
    pub fn one() -> Self {
        Self {
            theta: 1.0,
            phi: 0.0,
        }
    }

    /// |+⟩ state — equal superposition (the transmutation point)
    pub fn plus() -> Self {
        Self {
            theta: 0.5,
            phi: 0.0,
        }
    }

    /// |-⟩ state — equal superposition with phase flip
    pub fn minus() -> Self {
        Self {
            theta: 0.5,
            phi: PI,
        }
    }

    /// Create from Bloch sphere angles
    pub fn from_bloch(theta: f64, phi: f64) -> Self {
        Self {
            theta: (theta / PI).clamp(0.0, 1.0),
            phi: phi % (2.0 * PI),
        }
    }

    /// Probability of measuring |0⟩ (negative pole)
    pub fn prob_zero(&self) -> f64 {
        let half_theta = self.theta * PI / 2.0;
        half_theta.cos().powi(2)
    }

    /// Probability of measuring |1⟩ (positive pole)
    pub fn prob_one(&self) -> f64 {
        let half_theta = self.theta * PI / 2.0;
        half_theta.sin().powi(2)
    }

    /// Apply a Hadamard gate — the ultimate transmutation.
    /// Takes a pure state and puts it in superposition (and vice versa).
    /// This echoes the alchemical solve et coagula pattern in quantum computing.
    pub fn hadamard(&self) -> Self {
        // Simplified Hadamard on the Bloch sphere
        if (self.theta - 0.0).abs() < 1e-10 {
            // |0⟩ → |+⟩
            Self::plus()
        } else if (self.theta - 1.0).abs() < 1e-10 {
            // |1⟩ → |-⟩
            Self::minus()
        } else if (self.theta - 0.5).abs() < 1e-10 && self.phi.abs() < 1e-10 {
            // |+⟩ → |0⟩
            Self::zero()
        } else if (self.theta - 0.5).abs() < 1e-10 && (self.phi - PI).abs() < 1e-10 {
            // |-⟩ → |1⟩
            Self::one()
        } else {
            // General case: rotate on the Bloch sphere
            // H = rotation of π around the (X+Z)/√2 axis
            Self {
                theta: 1.0 - self.theta,
                phi: PI - self.phi,
            }
        }
    }

    /// Phase gate — rotate the phase without changing the pole position.
    /// This is changing the vibration without changing the polarity.
    /// The "hidden variable" that distinguishes |+⟩ from |-⟩.
    pub fn phase_shift(&self, angle: f64) -> Self {
        Self {
            theta: self.theta,
            phi: (self.phi + angle) % (2.0 * PI),
        }
    }

    /// Measure — force the qubit to collapse to a pole.
    /// This is an irreversible act: the superposition dies.
    /// Uses a deterministic threshold for reproducibility.
    ///
    /// In hermetic terms: the unmanifest becomes manifest.
    /// The price of manifestation is the loss of potential.
    pub fn measure(&self, threshold: f64) -> MeasurementResult {
        let p1 = self.prob_one();
        if threshold < p1 {
            MeasurementResult {
                collapsed: Qubit::one(),
                value: true,
                probability: p1,
                potential_lost: 1.0 - p1, // what we lost by manifesting
            }
        } else {
            MeasurementResult {
                collapsed: Qubit::zero(),
                value: false,
                probability: 1.0 - p1,
                potential_lost: p1,
            }
        }
    }
}

impl Polarity for Qubit {
    fn pole_position(&self) -> f64 {
        self.theta
    }

    fn transmute(&self, degree: f64) -> Self {
        Self {
            theta: (self.theta + degree).clamp(0.0, 1.0),
            phi: self.phi,
        }
    }
}

impl fmt::Display for Qubit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let p0 = self.prob_zero();
        let p1 = self.prob_one();
        write!(
            f,
            "|ψ⟩ = {:.2}|0⟩ + {:.2}|1⟩  (P(0)={:.1}%, P(1)={:.1}%)",
            p0.sqrt(),
            p1.sqrt(),
            p0 * 100.0,
            p1 * 100.0
        )
    }
}

/// The result of a measurement — manifestation from potential
#[derive(Debug)]
pub struct MeasurementResult {
    /// The collapsed state (now at a pole)
    pub collapsed: Qubit,
    /// The measured value (true = |1⟩, false = |0⟩)
    pub value: bool,
    /// The probability of this outcome
    pub probability: f64,
    /// How much potential was lost in the collapse
    pub potential_lost: f64,
}

impl fmt::Display for MeasurementResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Measured: |{}⟩ (p={:.1}%, potential lost: {:.1}%)",
            if self.value { "1" } else { "0" },
            self.probability * 100.0,
            self.potential_lost * 100.0
        )
    }
}

// ============================================================================
// POLAR OPERATIONS — the algebra of duality
// ============================================================================

/// Reconcile two polar values — the alchemical coniunctio.
/// The marriage of opposites produces a synthesis.
pub fn reconcile<P: Polarity>(a: &P, b: &P) -> f64 {
    (a.pole_position() + b.pole_position()) / 2.0
}

/// Polarize a neutral value — split unity into complementary opposites.
/// From the One comes the Two.
pub fn polarize(neutral: &SpectralValue, degree: f64) -> (SpectralValue, SpectralValue) {
    let degree = degree.clamp(0.0, 0.5);
    let positive = neutral.transmute(degree);
    let negative = neutral.transmute(-degree);
    (positive, negative)
}

/// Invert — flip to the opposite pole position.
/// "Extremes meet" — the deepest negative becomes the highest positive.
pub fn invert<P: Polarity>(value: &P) -> P {
    let current = value.pole_position();
    let inverted = 1.0 - current;
    value.transmute(inverted - current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spectrum_between_poles() {
        let temp = SpectralValue::new("Temperature", "Cold", "Hot", 0.75);
        assert_eq!(temp.polarity_sign(), PolaritySign::Positive);
        assert!((temp.polarization() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn transmutation_point() {
        let neutral = SpectralValue::neutral("Emotion", "Fear", "Courage");
        assert_eq!(neutral.polarity_sign(), PolaritySign::Neutral);
        assert!((neutral.polarization() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn transmute_along_spectrum() {
        let cold = SpectralValue::full_negative("Temperature", "Cold", "Hot");
        assert_eq!(cold.polarity_sign(), PolaritySign::Negative);

        // Transmute toward hot
        let warm = cold.transmute(0.7);
        assert_eq!(warm.polarity_sign(), PolaritySign::Positive);
    }

    #[test]
    fn opposites_are_same_nature() {
        // The KEY hermetic insight: opposites share the same nature, different degree
        let cold = SpectralValue::full_negative("Temperature", "Cold", "Hot");
        let hot = SpectralValue::full_positive("Temperature", "Cold", "Hot");

        // Different positions, same nature
        assert_eq!(cold.nature, hot.nature);
        // They reconcile to the middle
        let middle = reconcile(&cold, &hot);
        assert!((middle - 0.5).abs() < 1e-10);
    }

    #[test]
    fn polarize_and_reconcile() {
        let unity = SpectralValue::neutral("Force", "Yin", "Yang");
        let (yang, yin) = polarize(&unity, 0.5);

        assert_eq!(yang.polarity_sign(), PolaritySign::Positive);
        assert_eq!(yin.polarity_sign(), PolaritySign::Negative);

        // Reconciliation restores unity
        let restored = reconcile(&yang, &yin);
        assert!((restored - 0.5).abs() < 1e-10);
    }

    #[test]
    fn inversion() {
        let courage = SpectralValue::new("Emotion", "Fear", "Courage", 0.8);
        let fear = invert(&courage);
        assert!((fear.pole_position() - 0.2).abs() < 1e-10);
    }

    // --- Qubit tests: where Polarity meets Quantum ---

    #[test]
    fn qubit_zero_is_negative_pole() {
        let q = Qubit::zero();
        assert_eq!(q.polarity_sign(), PolaritySign::Negative);
        assert!((q.prob_zero() - 1.0).abs() < 1e-10);
        assert!((q.prob_one() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn qubit_one_is_positive_pole() {
        let q = Qubit::one();
        assert_eq!(q.polarity_sign(), PolaritySign::Positive);
        assert!((q.prob_one() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn qubit_plus_is_superposition() {
        let q = Qubit::plus();
        assert_eq!(q.polarity_sign(), PolaritySign::Neutral);
        // Equal probability of both poles
        assert!((q.prob_zero() - 0.5).abs() < 1e-10);
        assert!((q.prob_one() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn hadamard_is_solve_et_coagula() {
        // Hadamard on |0⟩ → |+⟩ (dissolve into superposition)
        let dissolved = Qubit::zero().hadamard();
        assert_eq!(dissolved.polarity_sign(), PolaritySign::Neutral);

        // Hadamard on |+⟩ → |0⟩ (coagulate back to certainty)
        let coagulated = dissolved.hadamard();
        assert_eq!(coagulated.polarity_sign(), PolaritySign::Negative); // back to |0⟩
    }

    #[test]
    fn measurement_collapses_potential() {
        let superposition = Qubit::plus();
        let result = superposition.measure(0.3);

        // After measurement, qubit is at a pole
        assert!(
            result.collapsed.polarity_sign() == PolaritySign::Positive
                || result.collapsed.polarity_sign() == PolaritySign::Negative
        );

        // Some potential was lost in the collapse
        assert!(result.potential_lost > 0.0);
    }

    #[test]
    fn phase_preserves_polarity() {
        // Phase shift changes the "hidden" dimension without changing pole position
        let q = Qubit::plus();
        let shifted = q.phase_shift(PI / 4.0);

        // Same pole position (same probabilities)
        assert!((q.prob_one() - shifted.prob_one()).abs() < 1e-10);
        // Different phase (different "vibration")
        assert!((q.phi - shifted.phi).abs() > 0.1);
    }

    #[test]
    fn qubit_probabilities_sum_to_one() {
        // Born rule: probabilities must sum to 1 (conservation of information)
        let states = vec![
            Qubit::zero(),
            Qubit::one(),
            Qubit::plus(),
            Qubit::minus(),
            Qubit::from_bloch(PI / 3.0, PI / 4.0),
            Qubit::from_bloch(PI / 6.0, PI),
        ];

        for q in &states {
            let sum = q.prob_zero() + q.prob_one();
            assert!(
                (sum - 1.0).abs() < 1e-10,
                "Probabilities don't sum to 1 for {:?}: {}",
                q,
                sum
            );
        }
    }

    #[test]
    fn qubit_is_polarity() {
        // A qubit satisfies the Polarity trait — it IS a polar entity
        let q = Qubit::from_bloch(PI / 3.0, 0.0);
        let pos = q.pole_position();
        assert!(pos > 0.0 && pos < 1.0); // In superposition = between poles

        // Transmuting a qubit moves it along the polar spectrum
        let transmuted = q.transmute(0.2);
        assert!(transmuted.pole_position() > q.pole_position());
    }
}
