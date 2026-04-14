// ============================================================================
// THE MAGNUM OPUS — The Great Work Complete
//
// All seven principles unified into a single cryptographic artifact:
// A key derivation and encipherment system where every operation
// is traceable to a hermetic principle.
//
// I.   MENTALISM       → Input is pure information, convertible between planes
// II.  CORRESPONDENCE  → Structure-preserving maps between plaintext and cipher domains
// III. VIBRATION       → Frequency analysis drives diffusion
// IV.  POLARITY        → Spectral transmutation provides confusion
// V.   RHYTHM          → Cyclic key schedule with neutralization
// VI.  CAUSALITY       → Every ciphertext byte traces to its origin
// VII. GENERATION      → Key expansion is generative/formative
//
// This is a STREAM CIPHER — it generates a keystream from the interaction
// of all seven principles, then XORs with plaintext.
//
// The keystream is not random in the conventional sense.
// It is DETERMINED — every byte has a cause.
// It is STRUCTURED — the structure is hidden in the vibrations.
// It is POLAR — it oscillates between states.
// It is RHYTHMIC — it follows cycles.
// It is CORRESPONDENT — operations above mirror operations below.
// It is GENERATED — it emerges from the marriage of forces.
// It is MENTAL — it is pure information transforming itself.
// ============================================================================

use std::f64::consts::PI;

/// The Magnum Opus: a hermetic stream cipher.
///
/// Initialize with a key (the Seed) and an intent (the Will).
/// The cipher generates an infinite keystream from the interaction
/// of all seven principles.
#[derive(Debug)]
pub struct MagnumOpus {
    // --- Internal state: the Athanor (alchemical furnace) ---

    // I. MENTALISM: the essence of the key, reduced to pure information
    essence: Vec<f64>,

    // II. CORRESPONDENCE: the veil between planes (derived from key)
    veil: Vec<f64>,

    // III. VIBRATION: frequency state — the spectral fingerprint
    spectrum: Vec<f64>,

    // IV. POLARITY: position on the polar spectrum for each state element
    poles: Vec<f64>,

    // V. RHYTHM: cyclic counters — each with a different period
    rhythms: Vec<RhythmState>,

    // VI. CAUSALITY: running hash of all prior output (causal chain)
    causal_accumulator: u64,
    bytes_produced: u64,

    // VII. GENERATION: the generative and formative seeds
    generative_seed: u64,
    formative_mask: u64,
}

#[derive(Debug, Clone)]
struct RhythmState {
    phase: f64,
    period: f64,
    amplitude: f64,
}

impl MagnumOpus {
    /// Initialize the Great Work.
    ///
    /// `key`: The alchemical seed — the materia prima
    /// `intent`: The magician's will — what this cipher is FOR
    ///
    /// The intent matters because two ciphers with the same key
    /// but different intents produce different streams.
    /// In conventional crypto, this is the nonce/IV.
    /// In hermetic terms, the same ingredients with different intent
    /// produce different results — as any practitioner knows.
    pub fn new(key: &[u8], intent: &[u8]) -> Self {
        let state_size = 32;

        // I. MENTALISM: reduce key to essence
        let essence = Self::extract_essence(key, intent, state_size);

        // II. CORRESPONDENCE: derive the veil from the essence
        let veil = Self::derive_veil(&essence);

        // III. VIBRATION: compute initial spectrum
        let spectrum = Self::initial_spectrum(&essence);

        // IV. POLARITY: set initial pole positions
        let poles: Vec<f64> = essence
            .iter()
            .map(|e| (e.sin() + 1.0) / 2.0) // map to [0, 1]
            .collect();

        // V. RHYTHM: initialize cyclic counters with prime periods
        let primes = [2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0, 19.0, 23.0, 29.0,
                      31.0, 37.0, 41.0, 43.0, 47.0, 53.0, 59.0, 61.0, 67.0, 71.0,
                      73.0, 79.0, 83.0, 89.0, 97.0, 101.0, 103.0, 107.0, 109.0, 113.0,
                      127.0, 131.0];
        let rhythms: Vec<RhythmState> = (0..state_size)
            .map(|i| RhythmState {
                phase: essence[i].fract().abs(),
                period: primes[i % primes.len()],
                amplitude: 0.5 + (essence[i].abs() % 0.5),
            })
            .collect();

        // VI. CAUSALITY: initialize the causal chain
        let causal_accumulator = key
            .iter()
            .fold(0xcbf29ce484222325u64, |acc, &b| {
                acc.wrapping_mul(0x100000001b3) ^ (b as u64)
            });

        // VII. GENERATION: extract generative and formative seeds
        let generative_seed = essence
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0) // even positions = generative
            .fold(0u64, |acc, (_, &v)| acc.wrapping_add((v.abs() * 1e15) as u64));

        let formative_mask = essence
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1) // odd positions = formative
            .fold(0u64, |acc, (_, &v)| acc.wrapping_add((v.abs() * 1e15) as u64));

        Self {
            essence,
            veil,
            spectrum,
            poles,
            rhythms,
            causal_accumulator,
            bytes_produced: 0,
            generative_seed,
            formative_mask,
        }
    }

    /// Generate the next byte of keystream.
    /// Every byte is the product of all seven principles interacting.
    fn next_byte(&mut self) -> u8 {
        let idx = (self.bytes_produced as usize) % self.essence.len();

        // III. VIBRATION: sample the current spectrum
        let vibration = self.spectrum[idx]
            * (2.0 * PI * self.bytes_produced as f64 / 256.0).sin();

        // IV. POLARITY: transmute the pole position
        self.poles[idx] = ((self.poles[idx] + 0.1 * vibration.sin()).abs()) % 1.0;
        let polar_contribution = (self.poles[idx] * 256.0) as u8;

        // V. RHYTHM: advance all rhythms and combine
        let rhythm_contribution: f64 = self.rhythms.iter_mut().enumerate().map(|(i, r)| {
            r.phase = (r.phase + 1.0 / r.period) % 1.0;
            r.amplitude * (2.0 * PI * r.phase).sin() * if i == idx { 2.0 } else { 0.1 }
        }).sum();

        // II. CORRESPONDENCE: apply the veil
        let veiled = ((self.veil[idx] * 256.0) as u8).wrapping_add(polar_contribution);

        // VII. GENERATION: generative expansion + formative selection
        self.generative_seed = self.generative_seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let generated = (self.generative_seed >> 33) as u8;
        let formed = generated & (self.formative_mask as u8 | 0x0F); // formative constrains

        // VI. CAUSALITY: incorporate all prior state
        self.causal_accumulator = self.causal_accumulator
            .wrapping_mul(0x100000001b3)
            ^ (veiled as u64)
            ^ (generated as u64)
            ^ ((rhythm_contribution.to_bits()) >> 32);
        let causal_byte = (self.causal_accumulator >> (8 * (idx % 8))) as u8;

        // I. MENTALISM: the final byte is the synthesis of all principles
        let output = veiled
            .wrapping_add(formed)
            .wrapping_add(causal_byte)
            .wrapping_add((rhythm_contribution.abs() * 50.0) as u8);

        // Update spectrum for next iteration (feedback)
        self.spectrum[idx] = (self.spectrum[idx] + output as f64 / 256.0).sin();

        self.bytes_produced += 1;
        output
    }

    /// Encipher plaintext — the Solve (dissolution)
    pub fn encipher(&mut self, plaintext: &[u8]) -> Vec<u8> {
        plaintext
            .iter()
            .map(|&byte| byte ^ self.next_byte())
            .collect()
    }

    /// Decipher ciphertext — the Coagula (reconstitution)
    /// XOR is its own inverse — solve and coagula are the same operation
    pub fn decipher(&mut self, ciphertext: &[u8]) -> Vec<u8> {
        ciphertext
            .iter()
            .map(|&byte| byte ^ self.next_byte())
            .collect()
    }

    /// Get the current causal state — the fingerprint of everything that has happened
    pub fn causal_state(&self) -> u64 {
        self.causal_accumulator
    }

    /// How many bytes have been produced
    pub fn bytes_produced(&self) -> u64 {
        self.bytes_produced
    }

    // --- Initialization helpers ---

    fn extract_essence(key: &[u8], intent: &[u8], size: usize) -> Vec<f64> {
        let mut essence = vec![0.0f64; size];
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

        // Mix key into essence
        for (i, &byte) in key.iter().enumerate() {
            let idx = i % size;
            essence[idx] += (byte as f64) * (phi * (i as f64 + 1.0)).sin();
        }

        // Mix intent — the Will modifies the essence
        for (i, &byte) in intent.iter().enumerate() {
            let idx = (i + key.len()) % size;
            essence[idx] += (byte as f64) * (PI * (i as f64 + 1.0)).cos();
        }

        // Length encoding
        let key_len = key.len() as f64;
        let intent_len = intent.len() as f64;
        for (i, val) in essence.iter_mut().enumerate() {
            *val += (key_len * intent_len).sqrt() * ((i as f64 + 1.0) * PI / size as f64).sin();
        }

        // Non-linear mixing pass
        for _ in 0..7 {
            let prev = essence.clone();
            for i in 0..size {
                let neighbor = prev[(i + 1) % size];
                essence[i] = (essence[i] + neighbor).sin() * 100.0
                    + essence[i].cos() * 50.0;
            }
        }

        essence
    }

    fn derive_veil(essence: &[f64]) -> Vec<f64> {
        essence
            .iter()
            .enumerate()
            .map(|(i, &e)| {
                ((e * PI).sin() + (e * (i as f64 + 1.0)).cos()) / 2.0
            })
            .collect()
    }

    fn initial_spectrum(essence: &[f64]) -> Vec<f64> {
        let n = essence.len();
        let mut spectrum = vec![0.0; n];

        // Mini-DFT of the essence
        for k in 0..n {
            for (i, &e) in essence.iter().enumerate() {
                let angle = 2.0 * PI * k as f64 * i as f64 / n as f64;
                spectrum[k] += e * angle.cos();
            }
            spectrum[k] /= n as f64;
        }

        spectrum
    }
}

/// Convenience: encipher a message with key and intent
pub fn encipher(key: &[u8], intent: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let mut opus = MagnumOpus::new(key, intent);
    opus.encipher(plaintext)
}

/// Convenience: decipher a message with key and intent
pub fn decipher(key: &[u8], intent: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let mut opus = MagnumOpus::new(key, intent);
    opus.decipher(ciphertext)
}

/// Format bytes as hex
pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encipher_decipher_roundtrip() {
        let key = b"The Hermetic Key";
        let intent = b"test encryption";
        let plaintext = b"As above, so below. As within, so without.";

        let ciphertext = encipher(key, intent, plaintext);
        let decrypted = decipher(key, intent, &ciphertext);

        assert_eq!(plaintext.to_vec(), decrypted);
    }

    #[test]
    fn ciphertext_differs_from_plaintext() {
        let key = b"secret";
        let intent = b"test";
        let plaintext = b"Hello, World!";

        let ciphertext = encipher(key, intent, plaintext);
        assert_ne!(plaintext.to_vec(), ciphertext);
    }

    #[test]
    fn different_keys_different_ciphertext() {
        let plaintext = b"same message";
        let c1 = encipher(b"key_alpha", b"intent", plaintext);
        let c2 = encipher(b"key_beta", b"intent", plaintext);
        assert_ne!(c1, c2);
    }

    #[test]
    fn different_intents_different_ciphertext() {
        // Same key, different intent = different stream
        // This is the hermetic principle: intent matters
        let plaintext = b"same message";
        let c1 = encipher(b"same_key", b"protect", plaintext);
        let c2 = encipher(b"same_key", b"conceal", plaintext);
        assert_ne!(c1, c2);
    }

    #[test]
    fn deterministic() {
        let key = b"deterministic";
        let intent = b"test";
        let plaintext = b"reproducible";

        let c1 = encipher(key, intent, plaintext);
        let c2 = encipher(key, intent, plaintext);
        assert_eq!(c1, c2);
    }

    #[test]
    fn empty_plaintext() {
        let ciphertext = encipher(b"key", b"intent", b"");
        assert!(ciphertext.is_empty());
        let decrypted = decipher(b"key", b"intent", &ciphertext);
        assert!(decrypted.is_empty());
    }

    #[test]
    fn long_message() {
        let key = b"The Key of Solomon";
        let intent = b"seal the grimoire";
        let plaintext = vec![42u8; 10000]; // 10KB message

        let ciphertext = encipher(key, intent, &plaintext);
        let decrypted = decipher(key, intent, &ciphertext);

        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn keystream_has_good_distribution() {
        // Generate 1000 bytes of keystream and check distribution
        let mut opus = MagnumOpus::new(b"distribution test", b"analysis");
        let keystream: Vec<u8> = (0..1000).map(|_| opus.next_byte()).collect();

        // Count byte frequencies
        let mut counts = [0u32; 256];
        for &byte in &keystream {
            counts[byte as usize] += 1;
        }

        // No byte value should appear more than ~20 times in 1000 bytes
        // (expected: ~3.9 per value)
        let max_count = *counts.iter().max().unwrap();
        assert!(
            max_count < 25,
            "Poor distribution: max byte count = {}",
            max_count
        );

        // At least 200 different byte values should appear
        let unique = counts.iter().filter(|&&c| c > 0).count();
        assert!(
            unique > 200,
            "Poor diversity: only {} unique byte values in 1000 bytes",
            unique
        );
    }

    #[test]
    fn causal_state_evolves() {
        let mut opus = MagnumOpus::new(b"causality", b"test");
        let state1 = opus.causal_state();
        opus.next_byte();
        let state2 = opus.causal_state();
        opus.next_byte();
        let state3 = opus.causal_state();

        // Each byte produced changes the causal state
        assert_ne!(state1, state2);
        assert_ne!(state2, state3);
        assert_ne!(state1, state3);
    }

    #[test]
    fn solve_et_coagula_symmetry() {
        // XOR is self-inverse: encrypt(encrypt(x)) = x
        // This is Solve et Coagula: dissolution then reconstitution
        let key = b"solve et coagula";
        let intent = b"demonstrate symmetry";
        let message = b"That which is above is like that which is below.";

        // Solve (dissolve into cipher)
        let dissolved = encipher(key, intent, message);
        assert_ne!(dissolved, message.to_vec());

        // Coagula (reconstitute from cipher)
        let reconstituted = decipher(key, intent, &dissolved);
        assert_eq!(reconstituted, message.to_vec());
    }

    #[test]
    fn wrong_key_fails() {
        let plaintext = b"secret message";
        let ciphertext = encipher(b"right_key", b"intent", plaintext);
        let wrong = decipher(b"wrong_key", b"intent", &ciphertext);
        assert_ne!(plaintext.to_vec(), wrong);
    }

    #[test]
    fn wrong_intent_fails() {
        let plaintext = b"secret message";
        let ciphertext = encipher(b"key", b"right_intent", plaintext);
        let wrong = decipher(b"key", b"wrong_intent", &ciphertext);
        assert_ne!(plaintext.to_vec(), wrong);
    }
}
