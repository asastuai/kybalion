// ============================================================================
// HERMETIC COMPUTING FRAMEWORK
//
// "The lips of wisdom are closed, except to the ears of Understanding."
//
// The Seven Principles of Hermes Trismegistus, expressed as Rust traits
// paired with established computational analogues. A reading, illustrated
// in executable code — a research artifact, not production cryptography.
// ============================================================================

pub mod principles;
pub mod composition;

#[cfg(feature = "wasm")]
pub mod wasm;
