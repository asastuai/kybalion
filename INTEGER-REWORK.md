# Integer Reimplementation — Design Intent & Invitation

> Current status: the Hermetic Hash and Magnum Opus cipher use floating-point arithmetic internally (`f64`, `sin`, `cos`, DFT). This is appropriate for illustrative/educational use but disqualifies them as production cryptography, because IEEE 754 does not guarantee bit-identical output across platforms.
>
> This document describes what a principled integer-only reimplementation should preserve (the seven-principle framing) and what it can freely replace (the internal mixing functions). The goal is to invite contribution from anyone with cryptography background who finds the framing interesting.

---

## What to preserve

The framing is the contribution. An integer reimplementation should keep:

1. **Seven internal state components**, one per hermetic principle, each named:
   - `essence` (Mentalism)
   - `veil` (Correspondence)
   - `spectrum` (Vibration)
   - `poles` (Polarity)
   - `rhythms` (Rhythm)
   - `causal` accumulator (Causality)
   - `seeds` — generative + formative (Generation)

2. **The seven-stage hash composition** (alchemical names, each mapped to a principle):
   - I. Calcination · II. Dissolution · III. Separation · IV. Conjunction · V. Fermentation · VI. Distillation · VII. Coagulation

3. **The trait architecture** in `src/principles/` — each principle remains a Rust trait with its axioms. The trait abstractions do not depend on floating-point.

4. **XOR self-inverse** for the cipher (*Solve et Coagula* symmetry). Encryption and decryption are the same operation applied to the keystream.

5. **Zero external crypto dependencies.** The current crate uses only `std` (and `num-traits` for trait bounds). That should remain true. No linking to `ring`, `aes`, etc.

6. **The illustrative + educational framing.** Even a good integer reimplementation should be clearly documented as "research artifact" until it has survived public cryptanalysis.

## What can freely change

The *internals*. Specifically:

- All floating-point state (`f64` → `u64` / `u128`).
- The DFT in Vibration → integer mixing (e.g., NTT over a prime field, ARX rounds, rotations).
- `sin`, `cos`, `sqrt` on floats → integer constants and bit operations.
- The coagulation step that XOR-folds `f64::to_bits()` → a proper integer finalization function.
- The specific non-linear mixing functions in Conjunction and Fermentation.

## Suggested architectures

Three established directions to choose from. None preferred; the chooser should pick what they can defend in analysis.

### ARX (Add-Rotate-XOR), ChaCha20-inspired

- **Ops:** 64-bit or 32-bit addition, bitwise rotation, XOR.
- **Precedent:** ChaCha20 (Bernstein, 2008), BLAKE3.
- **Fit for the framing:** Rotations are natural for Rhythm; XOR for Polarity and Causality; modular addition for Correspondence.
- **Pro:** Simple, fast, well-studied, side-channel tolerant.
- **Con:** No direct analogue for the DFT in Vibration — the "frequency domain" framing becomes metaphorical rather than literal.

### Sponge construction (Keccak-inspired)

- **Ops:** Large state (e.g., 1600 bits), a single permutation applied between absorb/squeeze phases.
- **Precedent:** Keccak / SHA-3 (NIST, 2015).
- **Fit for the framing:** The seven stages can live inside the permutation; absorb ↔ Calcination, squeeze ↔ Coagulation.
- **Pro:** Hash and stream cipher share machinery. Duplex mode gives intent-keyed encryption cleanly.
- **Con:** Larger state, more complex permutation to design.

### Feistel network, 7 rounds

- **Ops:** Split state in half, apply round function to one half, XOR into the other, swap.
- **Precedent:** DES, Blowfish, Camellia; provably invertible regardless of round function.
- **Fit for the framing:** Seven rounds = seven principles, one per round. Preserves symbolic resonance.
- **Pro:** Invertibility is free. Clear structure.
- **Con:** Round function quality is everything; designing a good one is non-trivial.

## Mapping each principle to integer operations

One compact suggestion (not prescriptive):

| Principle      | Integer operation                                                        |
|----------------|--------------------------------------------------------------------------|
| Mentalism      | Byte absorption into a canonical state representation                    |
| Correspondence | Invertible linear map over GF(2) (matrix × vector in Z/2Z)              |
| Vibration      | Integer mixing: NTT over a small prime field, or ARX permutation rounds  |
| Polarity       | Bitwise complement / rotation as spectrum position (no more \[0,1\] floats)|
| Rhythm         | Round counter + prime-offset rotations (e.g., `state = rotate(state, ROT[round_idx % 7])`) |
| Causality      | Running XOR hash of all produced output bytes (FNV-style is fine)        |
| Generation     | Integer LCG or xorshift + a mask derived from the key                    |

## Quality bar for "good enough to claim"

Minimum properties before claiming anything beyond "illustrative":

- **Bit-identical across platforms.** Same input → same output on x86, ARM, WASM, any target.
- **Avalanche ratio** on the hash: 0.5 ± 0.01 on a 10k-input sample.
- **Byte distribution** uniform (χ² close to expected under null).
- **Keystream period** on the cipher established (or bounded with argument). No short cycles.
- **Distinguishing attack resistance** — keystream should be statistically indistinguishable from random over large samples.
- **Zero collisions** on the hash within a 10^6 random input sample.
- **Performance** at least 100 MB/s single-threaded on a modern laptop, or document why not.

These are necessary, not sufficient. Serious deployment needs more (differential/linear cryptanalysis, peer review).

## How to contribute

1. Fork the repo: <https://github.com/asastuai/kybalion>
2. Pick one artifact to rework (start with the hash — smaller scope).
3. Implement on integers, keeping the trait architecture and the seven-stage structure.
4. Add tests: roundtrip, avalanche, distribution, collisions, platform-determinism.
5. Open a PR with:
   - Your architecture choice and reasoning
   - The mapping from principle to integer op
   - Test results
   - A short "here's what I preserved, here's what I changed" section in the PR description
6. The maintainer commits to reviewing within 2 weeks and co-crediting.

A partial contribution (just the hash, or just the cipher) is welcome. The two artifacts are independent.

## Roadmap note

The maintainer (Juan) plans to take on the **hash rework** as a learning project after the current round of paper submissions lands. The **cipher rework** remains open to contribution — it is more involved and benefits from outside eyes.

---

*If you are reading this and find the framing interesting but feel the cryptography is under-developed — you are right, and that is what this document is for. Improve it.*
