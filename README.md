# Hermetic Computing

**A computational reading of the Seven Hermetic Principles as cryptographic and quantum-computing concepts, implemented in Rust.**

A research artifact and educational framework — not production cryptography.

**[Try the Interactive Playground](https://asastuai.github.io/kybalion/)** | [Whitepaper](WHITEPAPER.md) | [Genesis](GENESIS.md) | [Research](RESEARCH.md) | [Paper (draft)](paper/hermetic-computing.md)

```
Tests:           87 passed, 0 failed
Avalanche ratio: 0.5001 (measured on a test sample, single-platform)
Dependencies:    1 (num-traits)
```

> **Status.** This project explores analogies between hermetic philosophy and modern computation. The cryptographic artifacts here use floating-point arithmetic, have not been formally cryptanalyzed, and are not cross-platform deterministic at the bit level. Do not use them to protect real data. For that, use an audited library (`ring`, `rustls`, `age`, `libsodium`).

---

## What this is

A computational exploration of the Seven Hermetic Principles (Kybalion, 1908) through implementation. Each principle is framed as a Rust trait and mapped to an established computational or cryptographic concept. The mapping is a *reading* — the code illustrates the correspondence, it does not establish it as a mathematical theorem.

| Hermetic Principle | Computational Analogue | Implementation |
|---|---|---|
| **Mentalism** — "The All is Mind" | Information as substrate | `trait Mentalism` |
| **Correspondence** — "As above, so below" | Structure-preserving maps (group homomorphism) | `trait Correspondence` — the additive case is analogous to the defining property of homomorphic encryption |
| **Vibration** — "Everything vibrates" | Frequency-domain representation (DFT) | `Spectrum` — DFT / IDFT roundtrip |
| **Polarity** — "Opposites are the same" | Continuous duals (qubits, spectral logic) | `Qubit` satisfies `trait Polarity` |
| **Rhythm** — "Everything flows" | Cyclic computation, timing dampening | `Pendulum` — analogous to constant-time execution |
| **Causality** — "Every cause has its effect" | Provenance (blockchain-like) | `CausalChain` |
| **Generation** — "Everything creates" | Generative-formative dynamics (GAN-like) | `KeyForge` |

From these primitives, two artifacts:

### The Hermetic Hash (256-bit, illustrative)

A hash composed of seven stages drawn from alchemical language:

```
Calcination → Dissolution → Separation → Conjunction → Fermentation → Distillation → Coagulation
```

Measured avalanche ratio of 0.5001 on the test set. Uses `f64`, `sin`, `cos`, and a DFT internally — so the output is **not guaranteed bit-identical across platforms or libm implementations**. Appropriate as a demonstration of how the seven stages can compose into a mixing function; inappropriate as a production hash. Not subjected to differential, linear, or preimage analysis.

### The Magnum Opus (Stream Cipher, illustrative)

A stream cipher where an "intent" — an arbitrary byte string supplied alongside the key — is mixed into the key schedule and thus into the keystream.

```rust
// Same key, different intent → different keystream → different ciphertext
let c1 = encipher(b"my_key", b"protect medical records", plaintext);
let c2 = encipher(b"my_key", b"conceal financial data",  plaintext);
assert_ne!(c1, c2);

// Wrong intent → bytes diverge
let wrong = decipher(b"my_key", b"wrong intent", &c1);
assert_ne!(plaintext, wrong);
```

**What this actually is.** Functionally, `intent` is additional key material — closer to a nonce than to a semantic policy. The cipher does not interpret the *meaning* of the intent string; it hashes its bytes into the key derivation. Framing that input as "intent" is a naming/UX choice, not a new cryptographic primitive.

**What this is not.** Cryptographic constructions that bind purpose or policy to encryption in a rigorous sense — Attribute-Based Encryption (Sahai–Waters, 2005), Functional Encryption (Boneh–Sahai–Waters, 2011) — are formally defined and substantially more rigorous than this artifact. This work is not in that lineage; it is an illustrative sketch.

---

## Readings we found interesting

These are *readings* of correspondence, not proofs.

### Correspondence as additive homomorphism

```
veil(a) + veil(b) = veil(a + b)   // when the map is linear
```

"As above, so below" captures the structural shape of a group homomorphism. Full homomorphic encryption (Gentry, 2009) requires both addition *and* multiplication on encrypted data; the additive case alone is much weaker.

### The qubit as a polar entity

|0⟩ and |1⟩ sit at the endpoints of a continuous spectrum; superposition lives between them. The `Qubit` type satisfies `trait Polarity` — a compatible description, not a discovery.

### Hadamard as *Solve et Coagula*

`H|0⟩ = |+⟩`, `H|+⟩ = |0⟩`, `H² = I`. The Hadamard gate is an involution, matching the alchemical pattern of dissolution-and-reconstitution. Many operations are involutions (NOT, XOR-with-fixed-key, bit-reversal); Hadamard is an elegant one.

### Polarity and phase

|+⟩ and |−⟩ have identical measurement probabilities but different global phase. Standard QM; the contribution here is vocabulary, not physics.

### Emergence in composition

```rust
let r = EmergentNumber::emerge(&a, &b);
// r may acquire properties (e.g. perfect_square) absent from both a and b.
```

A compact illustration of a phenomenon long studied in complexity theory and systems science: composition can produce properties absent from parts.

---

## Quick Start

```bash
git clone https://github.com/asastuai/kybalion.git
cd kybalion
cargo test
cargo run --bin hermetic
cargo run --bin purify
```

### Hash

```rust
use hermetic::composition::hermetic_hash::*;
let hash = hermetic_hash(b"As above, so below");
println!("{}", hash_to_hex(&hash));
```

### Cipher

```rust
use hermetic::composition::magnum_opus::*;
let ct = encipher(b"key", b"my purpose", b"secret message");
let pt = decipher(b"key", b"my purpose", &ct);
```

---

## Project Structure

```
src/
  principles/
    mentalism.rs        # I.   Information as substrate
    correspondence.rs   # II.  Structure-preserving maps
    vibration.rs        # III. Frequency domain (DFT/IDFT)
    polarity.rs         # IV.  Spectral logic + Qubit
    rhythm.rs           # V.   Cyclic computation + timing
    causality.rs        # VI.  Provenance chains
    generation.rs       # VII. Generative/formative creation
  composition/
    hermetic_hash.rs    # 256-bit hash (7 stages, illustrative)
    magnum_opus.rs      # Intent-keyed stream cipher (illustrative)
```

---

## Scope and Limitations

- **Not formally cryptanalyzed.** The 87 passing tests verify correctness — roundtrips, determinism on a single platform, absence of trivial collisions on 1000 inputs, avalanche on a sample. They do **not** verify: keystream period, linear or differential resistance, distinguishing-attack margins, preimage or second-preimage resistance, or behaviour under adversarial inputs.
- **Floating-point internals.** Both the hash and the cipher use `f64`, transcendental functions, and a DFT. Output is not guaranteed bit-identical across platforms, compilers, or math libraries. Reworking on integer arithmetic is on the roadmap.
- **"Intent-aware" is a framing, not a cryptographic category.** See the Magnum Opus section above.
- **Do not use this for real-world encryption or hashing.**

---

## Prior Art

Adjacent work read while building this:

- **Tavares (2020)** — hermetic axioms in software architecture (not cryptography).
- **Gentry (2009)** — fully homomorphic encryption; we borrow only the vocabulary of structure preservation.
- **Sahai & Waters (2005); Boneh, Sahai & Waters (2011)** — Attribute-Based / Functional Encryption; formally bind policy and function to cryptographic operations. These are the rigorous cousins of the "intent" framing.
- **Regev (2005)** — Learning With Errors / lattice-based cryptography.
- **Philosophical / interdisciplinary** — Morgan (2018) on hermeticism and quantum physics; writings on Kabbalah and computation.

We are not aware of prior work expressing the seven hermetic principles as Rust traits with executable crypto-style demonstrations. That is the framing contribution of this project. It is not a cryptographic contribution.

---

## Genesis

This framework was built in a single 46-day session with an AI collaborator, with no prior programming experience by the author at the start. The process itself is the subject of [Opus](https://asastuai.github.io/opus/).

Full narrative in [Genesis](GENESIS.md). Technical write-up in the [Whitepaper](WHITEPAPER.md).

---

## Roadmap

- [x] 7 principles as Rust traits
- [x] Hermetic Hash (floating-point, illustrative)
- [x] Magnum Opus (stream cipher, illustrative)
- [x] 87 correctness tests
- [x] Whitepaper (draft)
- [x] Interactive playground
- [ ] Integer-only reimplementation of hash and cipher — see [INTEGER-REWORK.md](INTEGER-REWORK.md) for design intent and contribution invitation
- [ ] Formal cryptanalysis (period, bias, differential, distinguishing)
- [ ] Paper repositioned for a philosophy-of-computing / interdisciplinary venue
- [ ] Python / TypeScript bindings (gated on the integer rework)

---

## License

MIT

---

## Author and body of work

Juan Cruz Maisu — `juancmaisu@outlook.com` — [github.com/asastuai](https://github.com/asastuai). Independent researcher, Buenos Aires, Argentina.

This framework is part of an evolving body of work:

- [Proof of Context (papers)](https://github.com/asastuai/proof-of-context) — v0.6 framework + v0.1 applied to verifiable inference
- [Proof of Context — reference implementation](https://github.com/asastuai/proof-of-context-impl) — Rust crate
- [SUR Protocol](https://github.com/asastuai/sur-protocol) — perp DEX with agent-native execution layer
- [intent-cipher](https://crates.io/crates/intent-cipher) — published crate, stream cipher with intent-keyed schedule
- [Hermetic Computing](https://github.com/asastuai/kybalion) — this repository

**Status:** open to research-engineering and applied-research roles in inference attestation, decentralized ML infrastructure, agent-native systems, and adjacent fields. Remote, full-time, any timezone.

---

*"The lips of wisdom are closed, except to the ears of Understanding."*
*— The Kybalion*
