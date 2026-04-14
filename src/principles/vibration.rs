// ============================================================================
// III. VIBRATION — The Principle of Frequency
// "Nothing rests; everything moves; everything vibrates."
//
// Axiom: All information has a frequency signature. Static data is an illusion.
// Everything is a waveform sampled at a point in time.
//
// Connection to quantum computing:
// - A qubit in superposition IS a vibrating state
// - The Quantum Fourier Transform decomposes states into frequency components
// - Shor's algorithm works because it finds the PERIOD (rhythm) of a function
//   by analyzing its VIBRATION (frequency spectrum)
// - Grover's algorithm amplifies the AMPLITUDE of the target state's vibration
//
// This principle makes frequency a first-class computational concept.
// ============================================================================

use std::f64::consts::PI;
use std::fmt::Debug;

/// Every piece of information vibrates at a frequency.
/// The frequency determines how it manifests and what it resonates with.
pub trait Vibration: Debug {
    /// The fundamental frequency of this information
    fn frequency(&self) -> f64;

    /// The harmonic overtones — secondary frequencies that color the vibration
    fn harmonics(&self) -> Vec<f64> {
        let f = self.frequency();
        (2..=7).map(|n| f * n as f64).collect()
    }

    /// The amplitude — intensity of manifestation at this frequency
    fn amplitude(&self) -> f64 {
        1.0 // default: unit amplitude
    }

    /// The phase — where in the cycle this vibration currently is [0, 2π)
    fn phase(&self) -> f64 {
        0.0 // default: zero phase
    }

    /// Sample this vibration at a point in time
    fn sample(&self, t: f64) -> f64 {
        self.amplitude() * (2.0 * PI * self.frequency() * t + self.phase()).sin()
    }

    /// Can this vibration resonate with another?
    /// Resonance occurs when frequencies form a simple ratio (p/q where p,q are small integers).
    /// Octave = 2:1, Fifth = 3:2, Fourth = 4:3, etc.
    fn resonates_with<V: Vibration>(&self, other: &V) -> bool {
        let f1 = self.frequency();
        let f2 = other.frequency();
        if f1 == 0.0 || f2 == 0.0 {
            return false;
        }
        let ratio = if f1 > f2 { f1 / f2 } else { f2 / f1 };

        // Check simple integer ratios up to 8:1 (8 octaves)
        // A ratio is "simple" if p/q ≈ ratio where p and q are small
        for q in 1..=8u32 {
            for p in q..=(q * 8) {
                let target = p as f64 / q as f64;
                if (ratio - target).abs() < 0.02 {
                    return true;
                }
            }
        }
        false
    }
}

// ============================================================================
// WAVEFORM — A concrete vibrating entity
// ============================================================================

/// A Waveform is the fundamental vibrating object.
/// It represents information expressed as oscillation.
#[derive(Debug, Clone)]
pub struct Waveform {
    pub freq: f64,
    pub amp: f64,
    pub phi: f64, // phase
}

impl Waveform {
    pub fn new(freq: f64, amp: f64, phi: f64) -> Self {
        Self { freq, amp, phi }
    }

    /// Pure tone at a given frequency
    pub fn pure(freq: f64) -> Self {
        Self::new(freq, 1.0, 0.0)
    }

    /// The "vibration" of silence — zero frequency, pure potential
    pub fn stillness() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Vibration for Waveform {
    fn frequency(&self) -> f64 {
        self.freq
    }
    fn amplitude(&self) -> f64 {
        self.amp
    }
    fn phase(&self) -> f64 {
        self.phi
    }
}

// ============================================================================
// SPECTRUM — The frequency decomposition of any data
// ============================================================================

/// A Spectrum is the vibrational fingerprint of information.
/// Just as white light contains all colors, all data contains frequencies.
///
/// This is the Discrete Fourier Transform reframed:
/// every byte sequence has a frequency signature — its vibrational essence.
#[derive(Debug, Clone)]
pub struct Spectrum {
    /// Frequency components: (frequency, amplitude, phase)
    pub components: Vec<Waveform>,
}

impl Spectrum {
    /// Decompose raw bytes into their vibrational spectrum.
    /// This is the hermetic DFT — revealing the hidden frequencies in matter.
    ///
    /// "To understand the vibration is to understand the thing."
    pub fn from_bytes(data: &[u8]) -> Self {
        let n = data.len();
        if n == 0 {
            return Self { components: vec![] };
        }

        let mut components = Vec::new();
        let samples: Vec<f64> = data.iter().map(|&b| b as f64).collect();

        // Full DFT — we need ALL frequency bins for perfect reconstruction
        for k in 0..n {
            let mut real = 0.0;
            let mut imag = 0.0;

            for (i, sample) in samples.iter().enumerate() {
                let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
                real += sample * angle.cos();
                imag -= sample * angle.sin();
            }

            let amplitude = (real * real + imag * imag).sqrt() / n as f64;
            let phase = imag.atan2(real);
            let frequency = k as f64 / n as f64;

            // Filter out near-zero vibrations — they carry no manifest energy
            if amplitude > 1e-10 {
                components.push(Waveform::new(frequency, amplitude, phase));
            }
        }

        Self { components }
    }

    /// Reconstruct data from its spectrum — materialization from vibration.
    /// "Raising or lowering the vibration causes a change in manifestation."
    pub fn to_bytes(&self, length: usize) -> Vec<u8> {
        let mut samples = vec![0.0_f64; length];
        let n = length as f64;

        for component in &self.components {
            let k = (component.freq * n).round() as usize;
            // Recover the original DFT coefficients from amplitude and phase
            // phase = atan2(imag, real) where imag was negated in forward DFT
            let real = component.amp * component.phi.cos() * n;
            let imag = component.amp * component.phi.sin() * n;
            for (i, sample) in samples.iter_mut().enumerate() {
                let angle = 2.0 * PI * k as f64 * i as f64 / n;
                // IDFT with conjugate: x[n] = (1/N) Σ (real*cos - imag*sin)
                *sample += (real * angle.cos() - imag * angle.sin()) / n;
            }
        }

        // Clamp to byte range — manifestation in the physical plane has limits
        samples
            .iter()
            .map(|&s| s.round().clamp(0.0, 255.0) as u8)
            .collect()
    }

    /// The dominant vibration — the fundamental frequency of the data
    pub fn dominant(&self) -> Option<&Waveform> {
        self.components
            .iter()
            .filter(|w| w.freq > 0.0) // exclude DC component
            .max_by(|a, b| a.amp.partial_cmp(&b.amp).unwrap())
    }

    /// Total vibrational energy — the "intensity" of this information
    pub fn energy(&self) -> f64 {
        self.components.iter().map(|w| w.amp * w.amp).sum::<f64>().sqrt()
    }

    /// Number of significant vibrations — the "complexity" of the information
    pub fn complexity(&self) -> usize {
        self.components.iter().filter(|w| w.amp > 0.01).count()
    }
}

// ============================================================================
// HARMONIC OPERATIONS — manipulate vibration directly
// ============================================================================

/// Harmonize two waveforms — if they resonate, they combine constructively.
/// If they don't, the result carries the dissonance.
pub fn harmonize(a: &Waveform, b: &Waveform) -> HarmonicResult {
    if a.resonates_with(b) {
        // Constructive interference — the frequencies amplify
        let combined_amp = a.amp + b.amp;
        let avg_phase = (a.phi + b.phi) / 2.0;
        HarmonicResult::Harmony(Waveform::new(a.freq, combined_amp, avg_phase))
    } else {
        // Dissonance — the frequencies clash
        let beat_freq = (a.freq - b.freq).abs();
        HarmonicResult::Dissonance {
            beat_frequency: beat_freq,
            tension: 1.0 - (a.freq / b.freq - (a.freq / b.freq).round()).abs().min(1.0),
        }
    }
}

/// Attune a waveform — shift its frequency toward a target.
/// This is vibrational transmutation: changing the rate of vibration
/// changes the nature of the manifestation.
pub fn attune(wave: &Waveform, target_freq: f64, degree: f64) -> Waveform {
    let degree = degree.clamp(0.0, 1.0);
    let new_freq = wave.freq + (target_freq - wave.freq) * degree;
    Waveform::new(new_freq, wave.amp, wave.phi)
}

#[derive(Debug)]
pub enum HarmonicResult {
    /// Frequencies are harmonically related — they amplify
    Harmony(Waveform),
    /// Frequencies clash — produces a beat frequency (audible as roughness)
    Dissonance {
        beat_frequency: f64,
        tension: f64, // 0.0 = maximum dissonance, 1.0 = near harmony
    },
}

// ============================================================================
// VIBRATION for primitive types — everything vibrates
// ============================================================================

/// Bytes vibrate at their value frequency — dense, physical plane vibration
impl Vibration for u8 {
    fn frequency(&self) -> f64 {
        *self as f64
    }
}

/// Strings vibrate at the sum of their character frequencies
/// (simplified gematria — every character contributes to the vibration)
impl Vibration for String {
    fn frequency(&self) -> f64 {
        self.chars().map(|c| c as u32 as f64).sum::<f64>()
    }
}

/// A byte slice vibrates at its spectral energy
impl Vibration for Vec<u8> {
    fn frequency(&self) -> f64 {
        Spectrum::from_bytes(self).energy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pure_tone_vibrates() {
        let a440 = Waveform::pure(440.0); // concert A
        assert_eq!(a440.frequency(), 440.0);
        assert_eq!(a440.amplitude(), 1.0);
    }

    #[test]
    fn octaves_resonate() {
        // An octave is a 2:1 frequency ratio — the most fundamental correspondence
        let a4 = Waveform::pure(440.0);
        let a5 = Waveform::pure(880.0); // one octave up
        assert!(a4.resonates_with(&a5));
        assert!(a5.resonates_with(&a4));
    }

    #[test]
    fn fifths_resonate() {
        // A perfect fifth is a 3:2 ratio — harmony
        let c = Waveform::pure(261.63);
        let g = Waveform::pure(392.0); // ~3:2 ratio
        assert!(c.resonates_with(&g));
    }

    #[test]
    fn dissonant_frequencies_clash() {
        let a = Waveform::pure(440.0);
        let b = Waveform::pure(466.16); // A# — semitone, maximum dissonance
        assert!(!a.resonates_with(&b));
    }

    #[test]
    fn harmonize_consonant() {
        let a = Waveform::pure(440.0);
        let b = Waveform::pure(880.0);
        match harmonize(&a, &b) {
            HarmonicResult::Harmony(result) => {
                assert_eq!(result.amp, 2.0); // amplitudes add
            }
            HarmonicResult::Dissonance { .. } => panic!("Expected harmony"),
        }
    }

    #[test]
    fn harmonize_dissonant() {
        let a = Waveform::pure(440.0);
        let b = Waveform::pure(466.16);
        match harmonize(&a, &b) {
            HarmonicResult::Dissonance { beat_frequency, .. } => {
                assert!((beat_frequency - 26.16).abs() < 0.01);
            }
            HarmonicResult::Harmony(_) => panic!("Expected dissonance"),
        }
    }

    #[test]
    fn attune_shifts_frequency() {
        let wave = Waveform::pure(440.0);
        let attuned = attune(&wave, 880.0, 0.5); // halfway to octave
        assert!((attuned.freq - 660.0).abs() < 0.01);
    }

    #[test]
    fn spectrum_roundtrip() {
        // Data → Spectrum → Data should preserve information
        // This is: matter → vibration → matter (the hermetic cycle)
        let original = vec![72, 101, 114, 109, 101, 115]; // "Hermes" in ASCII
        let spectrum = Spectrum::from_bytes(&original);
        let restored = spectrum.to_bytes(original.len());
        assert_eq!(original, restored);
    }

    #[test]
    fn spectrum_reveals_hidden_structure() {
        // A repeating pattern has a dominant frequency
        let pattern = vec![0u8, 255, 0, 255, 0, 255, 0, 255]; // pure oscillation
        let spectrum = Spectrum::from_bytes(&pattern);
        let dominant = spectrum.dominant().unwrap();
        // A pattern that repeats every 2 samples has frequency 0.5
        assert!((dominant.freq - 0.5).abs() < 0.01);
    }

    #[test]
    fn stillness_has_no_vibration() {
        let silence = Waveform::stillness();
        assert_eq!(silence.frequency(), 0.0);
        assert_eq!(silence.amplitude(), 0.0);
        // Sample at any time = 0 (no manifestation)
        assert_eq!(silence.sample(42.0), 0.0);
    }

    #[test]
    fn waveform_sampling() {
        let wave = Waveform::pure(1.0); // 1 Hz
        // At t=0: sin(0) = 0
        assert!(wave.sample(0.0).abs() < 1e-10);
        // At t=0.25: sin(π/2) = 1 (peak)
        assert!((wave.sample(0.25) - 1.0).abs() < 1e-10);
        // At t=0.5: sin(π) ≈ 0 (zero crossing)
        assert!(wave.sample(0.5).abs() < 1e-10);
    }

    #[test]
    fn string_vibration() {
        // "HERMES" has a different vibration than "hermes" (different plane of expression)
        let upper = "HERMES".to_string();
        let lower = "hermes".to_string();
        assert_ne!(upper.frequency(), lower.frequency());
        // But they resonate — same word, different octave of expression
        // (uppercase letters are exactly 32 less than lowercase in ASCII)
    }

    #[test]
    fn energy_reflects_complexity() {
        let simple = vec![0u8; 8]; // silence — no vibration
        let complex = vec![72, 101, 114, 109, 101, 115, 33, 42]; // varied data

        let simple_energy = Spectrum::from_bytes(&simple).energy();
        let complex_energy = Spectrum::from_bytes(&complex).energy();

        // More varied data has more vibrational energy
        assert!(complex_energy > simple_energy);
    }
}
