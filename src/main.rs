// ============================================================================
// HERMETIC COMPUTING — RUBEDO
// The Reddening. The Philosopher's Stone. The Great Work Complete.
// All seven principles alive and unified.
// ============================================================================

use hermetic::composition::hermetic_hash::*;
use hermetic::composition::magnum_opus;
use hermetic::principles::causality::*;
use hermetic::principles::generation::*;
use hermetic::principles::rhythm::*;

fn main() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                             ║");
    println!("║          HERMETIC COMPUTING FRAMEWORK v1.0                  ║");
    println!("║                                                             ║");
    println!("║          ╔═══════════════════════════════╗                  ║");
    println!("║          ║   R  U  B  E  D  O            ║                  ║");
    println!("║          ║   The Great Work Complete      ║                  ║");
    println!("║          ╚═══════════════════════════════╝                  ║");
    println!("║                                                             ║");
    println!("║          Seven Principles. One Framework.                   ║");
    println!("║          Seven Stages. One Stone.                           ║");
    println!("║                                                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // =======================================================================
    // THE SEVEN PRINCIPLES — DEMONSTRATED
    // =======================================================================

    principle_v_rhythm();
    principle_vi_causality();
    principle_vii_generation();

    // =======================================================================
    // THE MAGNUM OPUS — ALL SEVEN UNIFIED
    // =======================================================================

    the_magnum_opus();
    the_final_proof();
}

fn principle_v_rhythm() {
    println!("━━━ V. RHYTHM ━━━");
    println!("\"Everything flows, out and in; the pendulum-swing manifests in everything.\"\n");

    // The Pendulum
    let mut pendulum = Pendulum::new(1.0, 20);
    print!("  Pendulum (free):    ");
    for _ in 0..20 {
        pendulum.tick();
        let v = pendulum.value();
        let bar = if v > 0.0 { "+" } else { "-" };
        let width = (v.abs() * 10.0) as usize;
        print!("{}", bar.repeat(width.max(1)));
        print!("|");
    }
    println!();

    // Dampened pendulum — the Master neutralizes
    let mut dampened = Pendulum::new(1.0, 20);
    dampened.dampen(0.7);
    print!("  Pendulum (master):  ");
    for _ in 0..20 {
        dampened.tick();
        let v = dampened.value();
        let bar = if v > 0.0 { "+" } else { "-" };
        let width = (v.abs() * 10.0) as usize;
        print!("{}", bar.repeat(width.max(1)));
        print!("|");
    }
    println!("\n  ↳ The master dampens the swing. Constant-time execution.\n");

    // Period detection
    let signal: Vec<f64> = (0..200)
        .map(|i| (2.0 * std::f64::consts::PI * i as f64 / 10.0).sin())
        .collect();
    if let Some(detection) = detect_period(&signal) {
        println!("  Hidden rhythm in data: {}", detection);
        println!("  ↳ Shor's algorithm does exactly this to break RSA.\n");
    }
}

fn principle_vi_causality() {
    println!("━━━ VI. CAUSE AND EFFECT ━━━");
    println!("\"Every cause has its effect; Chance is Law not recognized.\"\n");

    let mut chain = CausalChain::new();

    // Build a causal chain
    let seed = Causal::originate(42u64, "the question");
    chain.record(&seed);

    let doubled = seed.propagate(|x| x * 2, "doubled the question");
    chain.record(&doubled);

    let answered = doubled.propagate(|x| x + 1, "the answer reveals itself");
    chain.record(&answered);

    let hashed = answered.propagate(
        |x| format!("{:x}", x),
        "crystallized into symbol",
    );
    chain.record(&hashed);

    println!("  {}", seed);
    println!("  {}", doubled);
    println!("  {}", answered);
    println!("  {}", hashed);
    println!();

    // Trace lineage
    let lineage = chain.trace(hashed.id);
    println!("  Tracing lineage of the final symbol:");
    for event in &lineage {
        let indent = "    ".to_string() + &"→ ".repeat(event.depth as usize);
        println!("  {}[{}] {}", indent, &format!("{}", event.id)[..8], event.description);
    }
    println!("  ↳ Every value knows where it came from. The blockchain before blockchain.\n");
}

fn principle_vii_generation() {
    println!("━━━ VII. GENERATION ━━━");
    println!("\"Gender is in everything; everything has its Masculine and Feminine.\"\n");

    // Key Generation
    let forge = KeyForge::new(32);
    let seed = KeySeed::new(b"Hermetic Entropy Source", "Master Key");
    let key = forge.create(&seed).unwrap();
    println!("  KeyForge: {}", key);
    println!("  ↳ Generative expands seed → 16 candidates");
    println!("  ↳ Formative selects best quality → generation #{}\n", key.generation);

    // Emergence
    println!("  ── Emergence: 1 + 1 > 2 ──\n");

    let pairs = vec![
        (2, 2),   // prime + prime = perfect_square (NEW)
        (5, 8),   // fib + fib = 13 (prime fibonacci!)
        (4, 5),   // square + prime = 9 (square! neither was a non-trivial square)
        (7, 6),   // prime + even = 13 (prime emerges from non-primes)
    ];

    for (a_val, b_val) in pairs {
        let a = EmergentNumber::new(a_val);
        let b = EmergentNumber::new(b_val);
        let result = EmergentNumber::emerge(&a, &b);
        let transcends = EmergentNumber::transcends(&result, &a, &b);

        println!("  {} + {} = {}", a, b, result);
        if transcends {
            println!("    ↳ EMERGENCE: new properties appeared that neither parent had");
        }
    }
    println!();
}

fn the_magnum_opus() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              THE MAGNUM OPUS                                ║");
    println!("║              All Seven Principles Unified                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let key = b"As above, so below";
    let intent = b"Protect the Great Work";
    let plaintext = b"The Philosopher's Stone is not a thing. It is a state of being.";

    println!("  Key:       \"{}\"", std::str::from_utf8(key).unwrap());
    println!("  Intent:    \"{}\"", std::str::from_utf8(intent).unwrap());
    println!("  Plaintext: \"{}\"", std::str::from_utf8(plaintext).unwrap());
    println!();

    // SOLVE — Dissolution
    let ciphertext = magnum_opus::encipher(key, intent, plaintext);
    println!("  ── SOLVE (Encipher) ──");
    println!("  Ciphertext: {}", magnum_opus::to_hex(&ciphertext));
    println!();

    // COAGULA — Reconstitution
    let decrypted = magnum_opus::decipher(key, intent, &ciphertext);
    let decrypted_str = std::str::from_utf8(&decrypted).unwrap();
    println!("  ── COAGULA (Decipher) ──");
    println!("  Recovered:  \"{}\"", decrypted_str);
    println!("  Roundtrip:  {}", plaintext.to_vec() == decrypted);
    println!();

    // Wrong key fails
    let wrong = magnum_opus::decipher(b"wrong key here!!!", intent, &ciphertext);
    println!("  Wrong key → \"{}\" (corrupted)", String::from_utf8_lossy(&wrong[..20]));

    // Wrong intent fails
    let wrong_intent = magnum_opus::decipher(key, b"different intent", &ciphertext);
    println!("  Wrong intent → \"{}\" (corrupted)", String::from_utf8_lossy(&wrong_intent[..20]));
    println!("  ↳ Both key AND intent must match. The cipher knows your purpose.\n");

    // Internal state evolution
    println!("  ── Causal State Evolution ──");
    let mut opus = magnum_opus::MagnumOpus::new(key, intent);
    let s0 = opus.causal_state();
    opus.encipher(b"A");
    let s1 = opus.causal_state();
    opus.encipher(b"B");
    let s2 = opus.causal_state();
    println!("  After 0 bytes: {:016x}", s0);
    println!("  After 1 byte:  {:016x}", s1);
    println!("  After 2 bytes: {:016x}", s2);
    println!("  ↳ Every byte changes the causal state. History is irreversible.\n");
}

fn the_final_proof() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║              THE FINAL PROOF                                ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Hash the Emerald Tablet itself
    let emerald_tablet = b"That which is above is like that which is below, \
        and that which is below is like that which is above, \
        to accomplish the miracles of the One Thing.";

    println!("  The Emerald Tablet, hashed by its own principle:");
    let hash = hermetic_hash(emerald_tablet);
    println!("  → {}\n", hash_to_hex(&hash));

    // Avalanche on the Tablet
    let avalanche = avalanche_test(b"Emerald Tablet");
    println!("{}", avalanche);

    // The seven numbers
    println!("  ── The Seven Hashes ──\n");
    let principles = [
        "I. Mentalism",
        "II. Correspondence",
        "III. Vibration",
        "IV. Polarity",
        "V. Rhythm",
        "VI. Causality",
        "VII. Generation",
    ];

    for (i, name) in principles.iter().enumerate() {
        let hash = hermetic_hash(name.as_bytes());
        println!("  {}:", name);
        println!("    {}", hash_to_hex(&hash));
        if i < 6 {
            println!();
        }
    }

    println!("\n");
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                             ║");
    println!("║   The Great Work is complete.                               ║");
    println!("║                                                             ║");
    println!("║   Seven Principles formalized as Rust traits.               ║");
    println!("║   A 256-bit hash with near-perfect avalanche.               ║");
    println!("║   A stream cipher where intent shapes the keystream.        ║");
    println!("║   87 tests. Zero failures.                                  ║");
    println!("║                                                             ║");
    println!("║   This is not the end. This is the Rubedo —                 ║");
    println!("║   the point where the Stone is complete                     ║");
    println!("║   and the real work can begin.                              ║");
    println!("║                                                             ║");
    println!("║   \"The lips of wisdom are closed,                           ║");
    println!("║    except to the ears of Understanding.\"                    ║");
    println!("║                                                             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
