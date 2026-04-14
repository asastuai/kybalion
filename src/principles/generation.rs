// ============================================================================
// VII. GENERATION — The Principle of Creation
// "Gender is in everything; everything has its Masculine and Feminine Principles;
//  Gender manifests on all planes."
//
// Axiom: New complexity emerges from the interaction of two complementary
// forces — the generative (projective) and the formative (receptive).
// Neither creates alone. Creation requires both.
//
// This is NOT about biological sex. It's about the creative dynamic:
// - GENERATIVE force: projects possibilities, expands, radiates outward
// - FORMATIVE force: selects, shapes, constrains, gives form
//
// In computation:
// - GAN: generator (generative) vs discriminator (formative)
// - Compilation: source expansion (generative) → optimization/selection (formative)
// - Key generation: entropy pool (generative) → validity check (formative)
// - Evolution: mutation (generative) → selection (formative)
// - Type systems: values (generative) → types/constraints (formative)
//
// In cryptography:
// - Key derivation: seed expands (generative) → KDF constrains (formative)
// - RSA keygen: random primes (generative) → primality test (formative)
// - PoW: nonce search (generative) → difficulty check (formative)
//
// The crucial insight: EMERGENCE.
// The creation is MORE than the sum of its inputs.
// 1 + 1 > 2. The whole transcends the parts.
// This is the anti-reductionist principle.
// ============================================================================

use std::fmt;

/// The creative dynamic: one force generates possibilities,
/// another gives them form. Neither creates alone.
pub trait Generation: fmt::Debug {
    type Seed;
    type Possibility;
    type Creation;

    /// Generate: project the seed into possibility space (generative principle)
    fn generate(&self, seed: &Self::Seed) -> Vec<Self::Possibility>;

    /// Form: select and shape from possibilities (formative principle)
    fn form(&self, possibilities: &[Self::Possibility]) -> Option<Self::Creation>;

    /// Create: the full act — generate then form
    fn create(&self, seed: &Self::Seed) -> Option<Self::Creation> {
        let possibilities = self.generate(seed);
        self.form(&possibilities)
    }
}

/// Emergence: the result transcends its inputs.
/// The whole is greater than the sum of its parts.
pub trait Emergent: fmt::Debug + Clone {
    /// Combine two entities to produce something new
    fn emerge(a: &Self, b: &Self) -> Self;

    /// Does the result transcend its inputs?
    /// Default: check if the result has properties neither input has alone
    fn transcends(result: &Self, a: &Self, b: &Self) -> bool;
}

// ============================================================================
// KEY FORGE — Cryptographic key generation as hermetic creation
// ============================================================================

/// A KeyForge generates cryptographic keys through the generative/formative dynamic.
///
/// Generative phase: expand a seed into many candidate keys
/// Formative phase: test candidates against validity constraints
///
/// The seed is the "intent" — what you want to create.
/// The forge is the "womb" — the space where creation happens.
#[derive(Debug)]
pub struct KeyForge {
    /// Key length in bytes
    pub key_length: usize,
    /// Number of candidates to generate (exploration breadth)
    pub candidates: usize,
    /// Validity predicate name (for tracing)
    pub constraint: String,
}

impl KeyForge {
    pub fn new(key_length: usize) -> Self {
        Self {
            key_length,
            candidates: 16,
            constraint: "entropy_threshold".to_string(),
        }
    }
}

/// A seed from which keys grow
#[derive(Debug, Clone)]
pub struct KeySeed {
    pub entropy: Vec<u8>,
    pub purpose: String,
}

impl KeySeed {
    pub fn new(entropy: &[u8], purpose: &str) -> Self {
        Self {
            entropy: entropy.to_vec(),
            purpose: purpose.to_string(),
        }
    }
}

/// A generated key with its lineage
#[derive(Debug, Clone)]
pub struct GeneratedKey {
    pub bytes: Vec<u8>,
    pub quality: f64, // 0.0 - 1.0, how well it passed the formative test
    pub generation: usize, // which candidate number it was
}

impl fmt::Display for GeneratedKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex: String = self.bytes.iter().map(|b| format!("{:02x}", b)).collect();
        write!(
            f,
            "Key(gen={}, quality={:.2}): {}",
            self.generation, self.quality, hex
        )
    }
}

impl Generation for KeyForge {
    type Seed = KeySeed;
    type Possibility = Vec<u8>;
    type Creation = GeneratedKey;

    /// Generative phase: expand the seed into candidate keys.
    /// Each candidate is a different "projection" of the seed.
    fn generate(&self, seed: &KeySeed) -> Vec<Vec<u8>> {
        let mut candidates = Vec::with_capacity(self.candidates);

        for candidate_num in 0..self.candidates {
            let mut key = vec![0u8; self.key_length];

            for (i, byte) in key.iter_mut().enumerate() {
                // Mix seed entropy with candidate number and position
                // Using a simple but effective expansion
                let seed_byte = if seed.entropy.is_empty() {
                    0
                } else {
                    seed.entropy[i % seed.entropy.len()]
                };

                let mixed = (seed_byte as u64)
                    .wrapping_mul(candidate_num as u64 + 1)
                    .wrapping_add(i as u64 * 31)
                    .wrapping_mul(0x9e3779b97f4a7c15); // golden ratio constant

                *byte = (mixed >> (8 * (i % 8))) as u8;
            }

            candidates.push(key);
        }

        candidates
    }

    /// Formative phase: select the best candidate.
    /// "Best" = highest entropy, most uniform byte distribution.
    fn form(&self, possibilities: &[Vec<u8>]) -> Option<GeneratedKey> {
        if possibilities.is_empty() {
            return None;
        }

        let mut best_idx = 0;
        let mut best_quality = 0.0_f64;

        for (idx, candidate) in possibilities.iter().enumerate() {
            let quality = evaluate_key_quality(candidate);
            if quality > best_quality {
                best_quality = quality;
                best_idx = idx;
            }
        }

        Some(GeneratedKey {
            bytes: possibilities[best_idx].clone(),
            quality: best_quality,
            generation: best_idx,
        })
    }
}

/// Evaluate key quality — the formative judgment.
/// Measures byte distribution uniformity (Shannon entropy approximation).
fn evaluate_key_quality(key: &[u8]) -> f64 {
    if key.is_empty() {
        return 0.0;
    }

    // Count byte frequencies
    let mut counts = [0u32; 256];
    for &byte in key {
        counts[byte as usize] += 1;
    }

    let n = key.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = count as f64 / n;
            entropy -= p * p.log2();
        }
    }

    // Normalize to [0, 1] where 1 = maximum entropy (all bytes unique)
    let max_entropy = (n.min(256.0)).log2();
    if max_entropy > 0.0 {
        (entropy / max_entropy).min(1.0)
    } else {
        0.0
    }
}

// ============================================================================
// EMERGENT NUMBER — Demonstrates emergence in arithmetic
// ============================================================================

/// An EmergentNumber carries properties that emerge from combination.
/// When two EmergentNumbers combine, the result has properties
/// that neither parent had alone.
#[derive(Debug, Clone)]
pub struct EmergentNumber {
    pub value: u64,
    pub properties: Vec<String>,
}

impl EmergentNumber {
    pub fn new(value: u64) -> Self {
        Self {
            value,
            properties: detect_properties(value),
        }
    }

    pub fn has_property(&self, prop: &str) -> bool {
        self.properties.iter().any(|p| p == prop)
    }
}

fn detect_properties(n: u64) -> Vec<String> {
    let mut props = Vec::new();
    if n == 0 {
        props.push("void".into());
        return props;
    }
    if n == 1 {
        props.push("unity".into());
    }
    if is_prime(n) {
        props.push("prime".into());
    }
    if is_perfect_square(n) {
        props.push("perfect_square".into());
    }
    if n % 2 == 0 {
        props.push("even".into());
    } else {
        props.push("odd".into());
    }
    if is_fibonacci(n) {
        props.push("fibonacci".into());
    }
    let digit_sum: u64 = n.to_string().chars().filter_map(|c| c.to_digit(10)).map(|d| d as u64).sum();
    if digit_sum == 9 || (n > 9 && digit_sum % 9 == 0) {
        props.push("divisible_by_9".into());
    }
    props
}

fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n < 4 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

fn is_perfect_square(n: u64) -> bool {
    let s = (n as f64).sqrt() as u64;
    s * s == n
}

fn is_fibonacci(n: u64) -> bool {
    // A number is Fibonacci if 5n²+4 or 5n²-4 is a perfect square
    let check1 = 5 * n * n + 4;
    is_perfect_square(check1) || (n > 0 && is_perfect_square(5 * n * n - 4))
}

impl Emergent for EmergentNumber {
    /// Combine two numbers — the result may have NEW properties
    fn emerge(a: &Self, b: &Self) -> Self {
        let new_value = a.value.wrapping_add(b.value);
        Self::new(new_value)
    }

    /// Does the result have properties that NEITHER input had?
    fn transcends(result: &Self, a: &Self, b: &Self) -> bool {
        result.properties.iter().any(|prop| {
            !a.has_property(prop) && !b.has_property(prop)
        })
    }
}

impl fmt::Display for EmergentNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} [{}]", self.value, self.properties.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_forge_creates_key() {
        let forge = KeyForge::new(32);
        let seed = KeySeed::new(b"hermetic entropy", "test key");
        let key = forge.create(&seed);
        assert!(key.is_some());
        let key = key.unwrap();
        assert_eq!(key.bytes.len(), 32);
        assert!(key.quality > 0.0);
    }

    #[test]
    fn different_seeds_different_keys() {
        let forge = KeyForge::new(32);
        let k1 = forge.create(&KeySeed::new(b"seed_alpha", "key 1")).unwrap();
        let k2 = forge.create(&KeySeed::new(b"seed_beta", "key 2")).unwrap();
        assert_ne!(k1.bytes, k2.bytes);
    }

    #[test]
    fn deterministic_generation() {
        let forge = KeyForge::new(32);
        let seed = KeySeed::new(b"fixed seed", "deterministic test");
        let k1 = forge.create(&seed).unwrap();
        let k2 = forge.create(&seed).unwrap();
        assert_eq!(k1.bytes, k2.bytes);
    }

    #[test]
    fn formative_selects_best() {
        // The formative principle should select the highest quality candidate
        let forge = KeyForge::new(32);
        let seed = KeySeed::new(b"test", "quality test");
        let candidates = forge.generate(&seed);
        let key = forge.form(&candidates).unwrap();

        // Verify it selected the best quality
        for (i, candidate) in candidates.iter().enumerate() {
            let quality = evaluate_key_quality(candidate);
            if i != key.generation {
                assert!(quality <= key.quality + 1e-10);
            }
        }
    }

    #[test]
    fn emergence_creates_new_properties() {
        // 4 (even, perfect_square) + 3 (prime, odd) = 7 (prime, odd)
        // 7 is prime — a NEW property that neither 4 nor 3+4=7... wait
        // Let's find a clear case:
        // 2 (prime, even) + 2 (prime, even) = 4 (even, perfect_square)
        // perfect_square is NEW — neither input had it
        let a = EmergentNumber::new(2);
        let b = EmergentNumber::new(2);
        let result = EmergentNumber::emerge(&a, &b);

        assert_eq!(result.value, 4);
        assert!(result.has_property("perfect_square"));
        assert!(!a.has_property("perfect_square"));
        assert!(!b.has_property("perfect_square"));
        assert!(EmergentNumber::transcends(&result, &a, &b));
    }

    #[test]
    fn emergence_demonstration() {
        // 5 (prime, odd, fibonacci) + 8 (even, fibonacci) = 13 (prime, odd, fibonacci)
        // 13 is prime — 8 wasn't prime. 13 is fibonacci — but both inputs were too.
        // The emergence: 13 is prime while 8 was not.
        let a = EmergentNumber::new(5);
        let b = EmergentNumber::new(8);
        let result = EmergentNumber::emerge(&a, &b);

        assert_eq!(result.value, 13);
        assert!(result.has_property("prime"));
        assert!(result.has_property("fibonacci"));
    }

    #[test]
    fn generation_trait_contract() {
        // The Generation trait must satisfy: create(seed) = form(generate(seed))
        let forge = KeyForge::new(16);
        let seed = KeySeed::new(b"contract test", "verification");

        let via_create = forge.create(&seed).unwrap();
        let candidates = forge.generate(&seed);
        let via_manual = forge.form(&candidates).unwrap();

        assert_eq!(via_create.bytes, via_manual.bytes);
    }
}
