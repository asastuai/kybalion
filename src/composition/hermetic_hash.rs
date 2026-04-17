// ============================================================================
// THE HERMETIC HASH — First Composition
//
// A hash function built from the composition of four hermetic principles:
//
//   MENTALISM:       Input is information — reduce to essence
//   VIBRATION:       Decompose essence into frequency spectrum
//   POLARITY:        Transmute spectral components through polar operations
//   CORRESPONDENCE:  Map the transmuted spectrum to a fixed output domain
//
// What makes this different from conventional hashes:
//
// SHA-256 works by: padding → blocks → rounds of bitwise mixing → digest
// It's engineered. Every operation is chosen for diffusion and confusion.
// It works. But it has no model of WHY it works beyond Shannon's criteria.
//
// The Hermetic Hash works by:
//   1. Revealing the hidden vibrational structure of the input (Vibration)
//   2. Transmuting that structure through polar inversion (Polarity)
//   3. Folding the transmuted vibrations through correspondence (Correspondence)
//   4. The output is the input transformed across planes (Mentalism)
//
// The hermetic model gives us a REASON for each operation:
//   - Diffusion comes from Vibration (frequency spreading)
//   - Confusion comes from Polarity (non-linear transmutation)
//   - Compression comes from Correspondence (many-to-one mapping)
//   - Determinism comes from Mentalism (information is conserved)
//
// This is NOT meant to replace SHA-256. Not yet.
// This is meant to demonstrate that hermetic principles can compose
// into a functioning cryptographic primitive.
// ============================================================================

use std::f64::consts::PI;

/// The Hermetic Hash — 256-bit output from arbitrary input.
///
/// Stages of the hash (alchemical operations):
/// 1. CALCINATION  — reduce input to its informational ash (byte normalization)
/// 2. DISSOLUTION  — dissolve into vibrational spectrum (DFT)
/// 3. SEPARATION   — separate spectral components by polarity
/// 4. CONJUNCTION  — marry polar opposites into new compounds
/// 5. FERMENTATION — let the compounds react (non-linear mixing)
/// 6. DISTILLATION — purify through repeated correspondence folding
/// 7. COAGULATION  — crystallize into the final 256-bit stone
///
/// Seven stages. Seven principles. Not a coincidence.
pub fn hermetic_hash(input: &[u8]) -> [u8; 32] {
    // === STAGE 1: CALCINATION ===
    // Reduce the input to a fixed working state.
    // "Calcination burns away the impurities, leaving only the essential salt."
    let state = calcinate(input);

    // === STAGE 2: DISSOLUTION ===
    // Dissolve the state into its frequency components.
    // "That which is solid must become liquid to be transformed."
    let spectrum = dissolve(&state);

    // === STAGE 3: SEPARATION ===
    // Separate the spectrum into polar pairs — positive and negative components.
    // "The pure must be separated from the impure."
    let (positive, negative) = separate(&spectrum);

    // === STAGE 4: CONJUNCTION ===
    // Marry the polar opposites — combine them non-linearly.
    // "The Red King and White Queen unite to produce the Philosopher's Child."
    let conjoined = conjoin(&positive, &negative);

    // === STAGE 5: FERMENTATION ===
    // Apply repeated transmutation — the state "ferments" through polar shifts.
    // "The dead matter must putrefy before new life can emerge."
    let fermented = ferment(&conjoined);

    // === STAGE 6: DISTILLATION ===
    // Fold through correspondence — compress and purify.
    // "Raise the volatile and condense it, again and again."
    let distilled = distill(&fermented);

    // === STAGE 7: COAGULATION ===
    // Crystallize the final hash — the Philosopher's Stone.
    // "Fix the volatile. The Stone is complete."
    coagulate(&distilled)
}

// ============================================================================
// STAGE 1: CALCINATION — Reduce to essential salt
// ============================================================================

fn calcinate(input: &[u8]) -> Vec<f64> {
    if input.is_empty() {
        // The Void still has an essence — the hash of nothing is not nothing
        return vec![PI; 64];
    }

    // Normalize to exactly 64 f64 samples
    // If shorter: repeat with phase shifts (different octaves of the same data)
    // If longer: fold with XOR-based mixing (correspondence: many → one)
    let mut state = vec![0.0f64; 64];

    let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

    for (i, &byte) in input.iter().enumerate() {
        let octave = (i / 64) as f64 + 1.0;
        let b = byte as f64;

        // Primary position
        let idx = i % 64;
        state[idx] += b * (phi * octave).sin();

        // Spread influence to 3 additional positions (diffusion)
        // Each byte touches 4 state positions — no byte is isolated
        state[(idx + 17) % 64] += b * (phi * (i as f64 + 1.0)).cos() * 0.5;
        state[(idx + 31) % 64] += b * (phi * (i as f64 + 2.0)).sin() * 0.3;
        state[(idx + 47) % 64] += b * octave.cos() * 0.2;
    }

    // Incorporate input length — different lengths must produce different hashes
    let len_vibration = (input.len() as f64).sqrt() * PI;
    for (i, val) in state.iter_mut().enumerate() {
        *val += len_vibration * ((i as f64 + 1.0) * PI / 64.0).sin();
    }

    // Pre-mix: ensure early bytes propagate fully before dissolution
    for _ in 0..3 {
        let prev = state.clone();
        for i in 0..64 {
            state[i] += prev[(i + 1) % 64] * 0.1 + prev[(i + 63) % 64] * 0.1;
        }
    }

    state
}

// ============================================================================
// STAGE 2: DISSOLUTION — DFT into frequency domain
// ============================================================================

fn dissolve(state: &[f64]) -> Vec<(f64, f64)> {
    // Full DFT — returns (real, imaginary) pairs for each frequency bin
    let n = state.len();
    let mut spectrum = Vec::with_capacity(n);

    for k in 0..n {
        let mut real = 0.0;
        let mut imag = 0.0;

        for (i, &sample) in state.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
            real += sample * angle.cos();
            imag -= sample * angle.sin();
        }

        spectrum.push((real / n as f64, imag / n as f64));
    }

    spectrum
}

// ============================================================================
// STAGE 3: SEPARATION — Split into polar pairs
// ============================================================================

fn separate(spectrum: &[(f64, f64)]) -> (Vec<f64>, Vec<f64>) {
    // Separate each frequency component into its polar magnitude and phase
    // Magnitude = how much energy (positive pole)
    // Phase = where in the cycle (negative pole — the hidden, the occult)
    let positive: Vec<f64> = spectrum
        .iter()
        .map(|(r, i)| (r * r + i * i).sqrt())
        .collect();

    let negative: Vec<f64> = spectrum
        .iter()
        .map(|(r, i)| i.atan2(*r))
        .collect();

    (positive, negative)
}

// ============================================================================
// STAGE 4: CONJUNCTION — Marry polar opposites
// ============================================================================

fn conjoin(positive: &[f64], negative: &[f64]) -> Vec<f64> {
    // The alchemical marriage: combine magnitude and phase non-linearly
    // This is where confusion is generated — the relationship between
    // the two poles creates something neither could alone
    let n = positive.len();
    let mut conjoined = vec![0.0f64; n];

    for i in 0..n {
        let mag = positive[i];
        let phase = negative[i];

        // The conjunction: magnitude modulated by the sin of phase,
        // crossed with the adjacent component's phase (entanglement)
        let adjacent_phase = negative[(i + 1) % n];

        // Non-linear mixing — the transmutation
        conjoined[i] = (mag * (phase + adjacent_phase).sin())
            + (mag.sqrt() * (phase * PI).cos())
            + ((mag + 1.0).ln() * adjacent_phase.sin());
    }

    conjoined
}

// ============================================================================
// STAGE 5: FERMENTATION — Repeated polar transmutation
// ============================================================================

fn ferment(state: &[f64]) -> Vec<f64> {
    let n = state.len();
    let mut fermented = state.to_vec();

    // 7 rounds of fermentation — one for each principle
    // Each round uses a DIFFERENT reach distance (prime offsets)
    // to ensure every element influences every other element
    let reaches: [usize; 7] = [1, 7, 13, 19, 29, 37, 43];

    for round in 0..7 {
        let mut next = vec![0.0f64; n];
        let reach = reaches[round];

        for i in 0..n {
            // Each element influenced by: previous, current, far, and cross
            let prev = fermented[(i + n - 1) % n];
            let curr = fermented[i];
            let far = fermented[(i + reach) % n];
            let cross = fermented[(n - 1 - i) % n]; // mirror position

            // Polar transmutation with round-dependent shift
            let shift = (round as f64 + 1.0) * PI / 7.0;

            next[i] = (curr + shift).sin() * 1000.0
                + (prev * curr.abs().max(0.001)).cos() * 500.0
                + (far + curr).sin().powi(3) * 250.0
                + (cross * shift).cos() * 125.0; // cross-mixing for full diffusion
        }

        fermented = next;
    }

    fermented
}

// ============================================================================
// STAGE 6: DISTILLATION — Fold through correspondence
// ============================================================================

fn distill(state: &[f64]) -> Vec<f64> {
    // Fold 64 values down to 32 through correspondence
    // Each pair of values (as above, so below) is mapped to one value
    // The mapping preserves the relationship between the pair
    let mut distilled = vec![0.0f64; 32];

    for i in 0..32 {
        let above = state[i];
        let below = state[i + 32];

        // Correspondence fold: the relationship between above and below
        // becomes the new value. Neither is privileged.
        distilled[i] = (above * below.abs().max(0.001)).sin() * 1000.0
            + (above + below).cos() * 500.0
            + (above - below).powi(2).sin() * 250.0;
    }

    // Second distillation — "Distill seven times until pure"
    // Each pass uses a different reach to ensure full mixing
    let dist_reaches: [usize; 7] = [1, 3, 7, 11, 15, 19, 23];
    for pass in 0..7 {
        let prev = distilled.clone();
        let reach = dist_reaches[pass];
        for i in 0..32 {
            let near = prev[(i + reach) % 32];
            let far = prev[(i + 32 - reach) % 32]; // symmetric reach
            distilled[i] = (distilled[i] + near).sin() * 1000.0
                + (distilled[i] * far.abs().max(0.001)).cos() * 500.0
                + ((pass as f64 + 1.0) * distilled[i]).sin() * 100.0;
        }
    }

    distilled
}

// ============================================================================
// STAGE 7: COAGULATION — Crystallize the Stone
// ============================================================================

fn coagulate(state: &[f64]) -> [u8; 32] {
    // Map each f64 to a byte — the final materialization.
    // The infinite precision of the spiritual (f64) is compressed
    // into the finite form of the physical (u8).
    //
    // We use the raw bits of the f64 — ALL 64 bits participate,
    // not just the fractional part. The XOR fold compresses 8 bytes
    // into 1, preserving influence from every bit of the mantissa.
    let mut stone = [0u8; 32];

    for i in 0..32 {
        let bits = state[i].to_bits();
        // XOR-fold all 8 bytes of the f64 into one byte
        // Every bit of the mantissa, exponent, and sign contributes
        let mut byte = 0u8;
        for shift in 0..8 {
            byte ^= (bits >> (shift * 8)) as u8;
        }
        // Mix with neighbor to break any remaining pattern
        let neighbor_bits = state[(i + 1) % 32].to_bits();
        byte ^= (neighbor_bits >> ((i * 3) % 64)) as u8;
        stone[i] = byte;
    }

    stone
}

// ============================================================================
// DISPLAY
// ============================================================================

/// Format a hash as hex string
pub fn hash_to_hex(hash: &[u8; 32]) -> String {
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Display the full alchemical process for educational purposes
pub fn hermetic_hash_verbose(input: &[u8]) -> [u8; 32] {
    println!("  ── The Alchemical Process ──\n");

    println!("  Input: {} bytes", input.len());
    if input.len() <= 32 {
        if let Ok(s) = std::str::from_utf8(input) {
            println!("  Text:  \"{}\"", s);
        }
    }

    let state = calcinate(input);
    println!("  1. CALCINATION:  {} samples, energy: {:.2}",
        state.len(),
        state.iter().map(|x| x * x).sum::<f64>().sqrt()
    );

    let spectrum = dissolve(&state);
    let spectral_energy: f64 = spectrum.iter().map(|(r, i)| r * r + i * i).sum::<f64>().sqrt();
    println!("  2. DISSOLUTION:  {} freq bins, spectral energy: {:.2}",
        spectrum.len(), spectral_energy
    );

    let (positive, negative) = separate(&spectrum);
    let pos_energy: f64 = positive.iter().sum();
    let neg_spread: f64 = negative.iter().map(|x| x.abs()).sum::<f64>() / negative.len() as f64;
    println!("  3. SEPARATION:   pos_energy: {:.2}, phase_spread: {:.4}",
        pos_energy, neg_spread
    );

    let conjoined = conjoin(&positive, &negative);
    let conj_energy: f64 = conjoined.iter().map(|x| x * x).sum::<f64>().sqrt();
    println!("  4. CONJUNCTION:  married energy: {:.2}", conj_energy);

    let fermented = ferment(&conjoined);
    let ferm_energy: f64 = fermented.iter().map(|x| x * x).sum::<f64>().sqrt();
    println!("  5. FERMENTATION: after 7 rounds: energy: {:.2}", ferm_energy);

    let distilled = distill(&fermented);
    let dist_energy: f64 = distilled.iter().map(|x| x * x).sum::<f64>().sqrt();
    println!("  6. DISTILLATION: 64→32 values, purified energy: {:.2}", dist_energy);

    let stone = coagulate(&distilled);
    println!("  7. COAGULATION:  The Stone crystallizes.\n");
    println!("  Hash: {}", hash_to_hex(&stone));

    stone
}

// ============================================================================
// ANALYSIS — Measure hash quality
// ============================================================================

/// Compute the avalanche effect: how many output bits change
/// when a single input bit is flipped.
/// Good hash: ~50% of bits change (128 out of 256).
pub fn avalanche_test(input: &[u8]) -> AvalancheResult {
    let original_hash = hermetic_hash(input);
    let total_bits = 256; // 32 bytes * 8 bits
    let mut total_flipped = 0;
    let mut tests = 0;

    // Flip each bit in the input and measure output change
    let mut modified = input.to_vec();
    for byte_idx in 0..input.len() {
        for bit_idx in 0..8 {
            modified[byte_idx] ^= 1 << bit_idx;
            let new_hash = hermetic_hash(&modified);
            modified[byte_idx] ^= 1 << bit_idx; // restore

            // Count differing bits
            let mut diff_bits = 0;
            for i in 0..32 {
                diff_bits += (original_hash[i] ^ new_hash[i]).count_ones() as usize;
            }
            total_flipped += diff_bits;
            tests += 1;
        }
    }

    let avg_flipped = total_flipped as f64 / tests as f64;
    let avalanche_ratio = avg_flipped / total_bits as f64;

    AvalancheResult {
        avg_bits_flipped: avg_flipped,
        total_bits,
        avalanche_ratio,
        tests_performed: tests,
        ideal_ratio: 0.5,
    }
}

#[derive(Debug)]
pub struct AvalancheResult {
    pub avg_bits_flipped: f64,
    pub total_bits: usize,
    pub avalanche_ratio: f64,
    pub tests_performed: usize,
    pub ideal_ratio: f64,
}

impl std::fmt::Display for AvalancheResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  ── Avalanche Analysis ──")?;
        writeln!(f, "  Tests performed:    {}", self.tests_performed)?;
        writeln!(f, "  Avg bits flipped:   {:.1} / {}", self.avg_bits_flipped, self.total_bits)?;
        writeln!(f, "  Avalanche ratio:    {:.4} (ideal: {:.4})", self.avalanche_ratio, self.ideal_ratio)?;
        writeln!(f, "  Deviation from ideal: {:.2}%",
            (self.avalanche_ratio - self.ideal_ratio).abs() * 100.0)?;

        let quality = if (self.avalanche_ratio - 0.5).abs() < 0.05 {
            "EXCELLENT — The Stone is pure"
        } else if (self.avalanche_ratio - 0.5).abs() < 0.10 {
            "GOOD — Minor impurities remain"
        } else if (self.avalanche_ratio - 0.5).abs() < 0.20 {
            "FAIR — Further distillation needed"
        } else {
            "POOR — The work is incomplete"
        };

        writeln!(f, "  Quality:            {}", quality)
    }
}

/// Distribution test: check if output bytes are uniformly distributed.
/// Good hash: each byte value should appear with roughly equal probability.
pub fn distribution_test(samples: &[[u8; 32]]) -> DistributionResult {
    let mut byte_counts = [0u64; 256];
    let total_bytes = samples.len() * 32;

    for hash in samples {
        for &byte in hash {
            byte_counts[byte as usize] += 1;
        }
    }

    let expected = total_bytes as f64 / 256.0;
    let chi_squared: f64 = byte_counts
        .iter()
        .map(|&count| {
            let diff = count as f64 - expected;
            diff * diff / expected
        })
        .sum();

    // Count how many byte values were never produced
    let missing_values = byte_counts.iter().filter(|&&c| c == 0).count();

    DistributionResult {
        chi_squared,
        expected_per_value: expected,
        total_bytes,
        missing_values,
    }
}

#[derive(Debug)]
pub struct DistributionResult {
    pub chi_squared: f64,
    pub expected_per_value: f64,
    pub total_bytes: usize,
    pub missing_values: usize,
}

impl std::fmt::Display for DistributionResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  ── Distribution Analysis ──")?;
        writeln!(f, "  Total bytes hashed: {}", self.total_bytes)?;
        writeln!(f, "  Expected per value: {:.2}", self.expected_per_value)?;
        writeln!(f, "  Chi-squared:        {:.2} (lower = more uniform)", self.chi_squared)?;
        writeln!(f, "  Missing byte values: {} / 256", self.missing_values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        // Same input MUST produce same output — Cause and Effect
        let hash1 = hermetic_hash(b"Hermes Trismegistus");
        let hash2 = hermetic_hash(b"Hermes Trismegistus");
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn different_inputs_different_outputs() {
        let h1 = hermetic_hash(b"Hermes");
        let h2 = hermetic_hash(b"Thoth");
        assert_ne!(h1, h2);
    }

    #[test]
    fn single_bit_changes_output() {
        // Flipping ONE bit must change the hash significantly
        let h1 = hermetic_hash(b"Hermes");
        let h2 = hermetic_hash(b"Iermes"); // H→I = one bit difference

        let diff_bits: u32 = h1.iter().zip(h2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();

        // Should flip roughly half the bits (128 ± margin)
        assert!(diff_bits > 50, "Avalanche too weak: only {} bits changed", diff_bits);
    }

    #[test]
    fn empty_input_has_hash() {
        // The Void has an essence — it's not nothing
        let hash = hermetic_hash(b"");
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn length_sensitivity() {
        // Inputs of different lengths but same content prefix must differ
        let h1 = hermetic_hash(b"ABC");
        let h2 = hermetic_hash(b"ABCD");
        assert_ne!(h1, h2);
    }

    #[test]
    fn output_is_256_bits() {
        let hash = hermetic_hash(b"test");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn long_input() {
        // Must handle inputs larger than the internal state
        let long = vec![42u8; 10000];
        let hash = hermetic_hash(&long);
        assert_ne!(hash, [0u8; 32]);
    }

    #[test]
    fn similar_inputs_diverge() {
        // "AAAA" vs "AAAB" — minimal change, maximal output difference
        let h1 = hermetic_hash(b"AAAA");
        let h2 = hermetic_hash(b"AAAB");

        let diff: u32 = h1.iter().zip(h2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();

        assert!(diff > 30, "Similar inputs too correlated: {} bits differ", diff);
    }

    #[test]
    fn no_trivial_collisions() {
        // Hash the numbers 0-999 and check for collisions
        let mut hashes: Vec<[u8; 32]> = Vec::new();
        for i in 0..1000 {
            let hash = hermetic_hash(i.to_string().as_bytes());
            assert!(!hashes.contains(&hash), "Collision at input {}", i);
            hashes.push(hash);
        }
    }

    #[test]
    fn hash_hex_format() {
        let hash = hermetic_hash(b"test");
        let hex = hash_to_hex(&hash);
        assert_eq!(hex.len(), 64); // 32 bytes = 64 hex chars
        assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
