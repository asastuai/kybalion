// ============================================================================
// WASM BINDINGS — The Playground Interface
//
// Exposes hermetic primitives to the browser via WebAssembly.
// Each function is callable from JavaScript.
// ============================================================================

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
use crate::composition::hermetic_hash;
#[cfg(feature = "wasm")]
use crate::composition::magnum_opus;
#[cfg(feature = "wasm")]
use crate::principles::correspondence::HebrewGematria;
#[cfg(feature = "wasm")]
use crate::principles::correspondence::Correspondence;

// ============================================================================
// HASH LAB
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hash_hex(input: &str) -> String {
    let hash = hermetic_hash::hermetic_hash(input.as_bytes());
    hermetic_hash::hash_to_hex(&hash)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hash_verbose(input: &str) -> String {
    let bytes = input.as_bytes();
    let hash = hermetic_hash::hermetic_hash(bytes);

    let mut result = String::new();
    result.push_str(&format!("Input: \"{}\" ({} bytes)\n", input, bytes.len()));
    result.push_str(&format!("Hash:  {}\n", hermetic_hash::hash_to_hex(&hash)));
    result.push_str(&format!("\nStages:\n"));
    result.push_str("  1. CALCINATION  — Input reduced to 64 vibrational samples\n");
    result.push_str("  2. DISSOLUTION  — DFT decomposes into 64 frequency bins\n");
    result.push_str("  3. SEPARATION   — Split into magnitude (visible) + phase (hidden)\n");
    result.push_str("  4. CONJUNCTION  — Non-linear marriage of polar opposites\n");
    result.push_str("  5. FERMENTATION — 7 rounds of cyclic transmutation\n");
    result.push_str("  6. DISTILLATION — Correspondence fold: 64 → 32 values\n");
    result.push_str("  7. COAGULATION  — Crystallize into 32 bytes (256 bits)\n");
    result
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn hash_compare(input_a: &str, input_b: &str) -> String {
    let ha = hermetic_hash::hermetic_hash(input_a.as_bytes());
    let hb = hermetic_hash::hermetic_hash(input_b.as_bytes());

    let diff_bits: u32 = ha.iter().zip(hb.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        .sum();

    let ratio = diff_bits as f64 / 256.0;

    let mut result = String::new();
    result.push_str(&format!("A: \"{}\" → {}\n", input_a, hermetic_hash::hash_to_hex(&ha)));
    result.push_str(&format!("B: \"{}\" → {}\n", input_b, hermetic_hash::hash_to_hex(&hb)));
    result.push_str(&format!("\nBits changed: {} / 256 ({:.1}%)\n", diff_bits, ratio * 100.0));
    result.push_str(&format!("Avalanche ratio: {:.4} (ideal: 0.5000)\n", ratio));

    if (ratio - 0.5).abs() < 0.05 {
        result.push_str("Quality: EXCELLENT — The Stone is pure\n");
    } else if (ratio - 0.5).abs() < 0.10 {
        result.push_str("Quality: GOOD — Minor impurities remain\n");
    } else {
        result.push_str("Quality: NEEDS WORK\n");
    }

    result
}

// ============================================================================
// CIPHER LAB
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn encipher(key: &str, intent: &str, plaintext: &str) -> String {
    let ct = magnum_opus::encipher(key.as_bytes(), intent.as_bytes(), plaintext.as_bytes());
    magnum_opus::to_hex(&ct)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn decipher(key: &str, intent: &str, ciphertext_hex: &str) -> String {
    let ct = hex_to_bytes(ciphertext_hex);
    let pt = magnum_opus::decipher(key.as_bytes(), intent.as_bytes(), &ct);
    String::from_utf8_lossy(&pt).to_string()
}

#[cfg(feature = "wasm")]
fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .filter_map(|i| {
            if i + 2 <= hex.len() {
                u8::from_str_radix(&hex[i..i + 2], 16).ok()
            } else {
                None
            }
        })
        .collect()
}

// ============================================================================
// GEMATRIA LAB
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn gematria(word: &str) -> String {
    let g = HebrewGematria;
    match g.ascend(&word.to_string()) {
        Ok(value) => format!("{} = {}", word, value),
        Err(_) => format!("Cannot compute gematria for '{}' — contains non-Hebrew characters", word),
    }
}

// ============================================================================
// QUBIT LAB
// ============================================================================

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn qubit_state(name: &str) -> String {
    use crate::principles::polarity::{Qubit, Polarity};

    let q = match name {
        "|0>" | "zero" => Qubit::zero(),
        "|1>" | "one" => Qubit::one(),
        "|+>" | "plus" => Qubit::plus(),
        "|->" | "minus" => Qubit::minus(),
        _ => Qubit::plus(),
    };

    let mut result = String::new();
    result.push_str(&format!("State: {}\n", q));
    result.push_str(&format!("Pole position: {:.4}\n", q.pole_position()));
    result.push_str(&format!("Polarity: {:?}\n", q.polarity_sign()));
    result.push_str(&format!("Phase: {:.4}\n", q.phi));
    result.push_str(&format!("P(|0⟩): {:.1}%\n", q.prob_zero() * 100.0));
    result.push_str(&format!("P(|1⟩): {:.1}%\n", q.prob_one() * 100.0));
    result
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn qubit_hadamard(name: &str) -> String {
    use crate::principles::polarity::Qubit;

    let q = match name {
        "|0>" | "zero" => Qubit::zero(),
        "|1>" | "one" => Qubit::one(),
        "|+>" | "plus" => Qubit::plus(),
        "|->" | "minus" => Qubit::minus(),
        _ => Qubit::plus(),
    };

    let h = q.hadamard();
    format!("{} → H → {}", q, h)
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn qubit_measure(name: &str, threshold: f64) -> String {
    use crate::principles::polarity::Qubit;

    let q = match name {
        "|0>" | "zero" => Qubit::zero(),
        "|1>" | "one" => Qubit::one(),
        "|+>" | "plus" => Qubit::plus(),
        "|->" | "minus" => Qubit::minus(),
        _ => Qubit::plus(),
    };

    let result = q.measure(threshold);
    let mut s = String::new();
    s.push_str(&format!("Input: {}\n", q));
    s.push_str(&format!("{}\n", result));
    s.push_str(&format!("Potential destroyed: {:.1}%\n", result.potential_lost * 100.0));
    s
}
