// ============================================================================
// PURIFICATION DIAGNOSTIC
// Find where the impurity lives in the Hermetic Hash
// ============================================================================

use hermetic::composition::hermetic_hash::*;

fn main() {
    println!("━━━ PURIFICATION DIAGNOSTIC ━━━\n");

    // Test avalanche across many different inputs to find the true ratio
    let test_inputs: Vec<Vec<u8>> = (0..50)
        .map(|i| format!("test_input_{:04}", i).into_bytes())
        .collect();

    let mut total_ratio = 0.0;
    let mut min_ratio = 1.0f64;
    let mut max_ratio = 0.0f64;
    let mut worst_input = String::new();

    for input in &test_inputs {
        let result = avalanche_test(input);
        total_ratio += result.avalanche_ratio;
        if result.avalanche_ratio < min_ratio {
            min_ratio = result.avalanche_ratio;
            worst_input = String::from_utf8_lossy(input).to_string();
        }
        if result.avalanche_ratio > max_ratio {
            max_ratio = result.avalanche_ratio;
        }
    }

    let avg_ratio = total_ratio / test_inputs.len() as f64;

    println!("  Across {} inputs:", test_inputs.len());
    println!("  Average avalanche: {:.6}", avg_ratio);
    println!("  Min:               {:.6}", min_ratio);
    println!("  Max:               {:.6}", max_ratio);
    println!("  Range:             {:.6}", max_ratio - min_ratio);
    println!("  Worst input:       \"{}\"", worst_input);
    println!("  Deviation:         {:.4}%\n", (avg_ratio - 0.5).abs() * 100.0);

    // Per-byte analysis: which output bytes have the worst avalanche?
    println!("  ── Per-Byte Avalanche Analysis ──\n");
    let input = b"diagnostic input for per-byte analysis";
    let original = hermetic_hash(input);

    let mut byte_flip_counts = vec![0u64; 32]; // how many bits flip per output byte
    let mut total_tests = 0u64;

    let mut modified = input.to_vec();
    for byte_idx in 0..input.len() {
        for bit_idx in 0..8 {
            modified[byte_idx] ^= 1 << bit_idx;
            let new_hash = hermetic_hash(&modified);
            modified[byte_idx] ^= 1 << bit_idx;

            for i in 0..32 {
                byte_flip_counts[i] += (original[i] ^ new_hash[i]).count_ones() as u64;
            }
            total_tests += 1;
        }
    }

    println!("  Output byte | Avg bits flipped | Ratio  | Status");
    println!("  ------------|------------------|--------|-------");
    for i in 0..32 {
        let avg = byte_flip_counts[i] as f64 / total_tests as f64;
        let ratio = avg / 8.0; // 8 bits per byte, ideal = 0.5
        let status = if (ratio - 0.5).abs() < 0.02 {
            "OK"
        } else if (ratio - 0.5).abs() < 0.05 {
            "WARN"
        } else {
            "IMPURE"
        };
        println!("  {:>11} | {:.4}            | {:.4} | {}", i, avg, ratio, status);
    }

    // Per-input-bit analysis: which INPUT bits have the weakest influence?
    println!("\n  ── Per-Input-Bit Influence Analysis ──\n");
    let input = b"influence test";
    let original = hermetic_hash(input);

    println!("  Input byte | Avg output bits flipped | Ratio  | Status");
    println!("  -----------|-------------------------|--------|-------");

    let mut modified = input.to_vec();
    for byte_idx in 0..input.len() {
        let mut total_flipped = 0u32;
        for bit_idx in 0..8 {
            modified[byte_idx] ^= 1 << bit_idx;
            let new_hash = hermetic_hash(&modified);
            modified[byte_idx] ^= 1 << bit_idx;

            for i in 0..32 {
                total_flipped += (original[i] ^ new_hash[i]).count_ones();
            }
        }
        let avg = total_flipped as f64 / 8.0;
        let ratio = avg / 256.0;
        let status = if (ratio - 0.5).abs() < 0.02 {
            "OK"
        } else if (ratio - 0.5).abs() < 0.05 {
            "WARN"
        } else {
            "IMPURE"
        };
        let ch = if byte_idx < input.len() {
            input[byte_idx] as char
        } else {
            '?'
        };
        println!("  {:>10} | {:.1}                    | {:.4} | {} ('{}')",
            byte_idx, avg, ratio, status, ch);
    }
}
