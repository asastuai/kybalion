# Hermetic Computing — Whitepaper (Superseded)

> **This document is retained for history. The current technical write-up is [paper/hermetic-computing.md](paper/hermetic-computing.md). For a concise overview, see [README.md](README.md). For the narrative origin, see [GENESIS.md](GENESIS.md).**
>
> The earlier version of this whitepaper made stronger claims than the current evidence supports — in particular, claims of "structural isomorphism", "first cipher where intent is a cryptographic parameter", and framing of analogies as formal theorems. Those framings have been revised across the project. The honest version of the technical content lives in the paper linked above.
>
> The two appendices below are retained because they document the repository structure and test summary at a specific moment in the project's history, and that information may be useful for anyone reading the code.

---

## Appendix A: Repository Structure

```
hermetic-computing/
  src/
    lib.rs                          # Framework entry point
    principles/
      mod.rs                        # Module declarations
      mentalism.rs                  # Principle I:   Information axiom
      correspondence.rs             # Principle II:  Homomorphism
      vibration.rs                  # Principle III: Frequency domain
      polarity.rs                   # Principle IV:  Spectral logic + Qubit
      rhythm.rs                     # Principle V:   Cyclic computation
      causality.rs                  # Principle VI:  Provenance chains
      generation.rs                 # Principle VII: Generative/formative creation
    composition/
      mod.rs                        # Composition entry point
      hermetic_hash.rs              # 256-bit hash (7 alchemical stages, illustrative)
      magnum_opus.rs                # Intent-keyed stream cipher (illustrative)
  SPECIFICATION.md                  # Formal specification of all 7 principles
  WHITEPAPER.md                     # This document (superseded)
```

## Appendix B: Test Summary (as of April 2026)

| Module | Tests | Passed | Key Tests |
|--------|-------|--------|-----------|
| Mentalism | 4 | 4 | NaN rejection, plane hierarchy, essence roundtrip |
| Correspondence | 9 | 9 | Emerald Tablet proof, gematria, composition |
| Vibration | 13 | 13 | Spectrum roundtrip, resonance, pattern detection |
| Polarity | 15 | 15 | Qubit as polar entity, Hadamard, Born rule |
| Rhythm | 10 | 10 | Pendulum, dampening, period detection |
| Causality | 7 | 7 | Chain tracing, merge, deterministic IDs |
| Generation | 6 | 6 | Key forge, emergence proof, trait contract |
| Hermetic Hash | 10 | 10 | Avalanche, collisions, distribution |
| Magnum Opus | 12 | 12 | Roundtrip, intent-awareness, keystream quality |
| **Total** | **87** | **87** | |

These are correctness tests, not security tests. For discussion of what correctness tests do and do not verify, see Section 7 of [paper/hermetic-computing.md](paper/hermetic-computing.md).
