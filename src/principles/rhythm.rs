// ============================================================================
// V. RHYTHM — The Principle of Cycles
// "Everything flows, out and in; everything has its tides;
//  all things rise and fall; the pendulum-swing manifests in everything;
//  the measure of the swing to the right is the measure of the swing to the left."
//
// Axiom: All computation is cyclical. Linear execution is an illusion —
// every process is part of a larger oscillation.
// The master neutralizes the swing through understanding.
//
// In cryptography: timing is everything.
// - Side-channel attacks exploit RHYTHM (timing patterns in execution)
// - Constant-time code IS hermetic neutralization (dampening the swing)
// - Key rotation follows cycles (the key must change with the rhythm)
// - Nonces are counters — they track position in a cycle
//
// In quantum computing:
// - Quantum error correction fights decoherence — the rhythm of noise
// - Quantum annealing uses cooling schedules — controlled oscillation decay
// - Shor's algorithm finds the PERIOD of a function — the hidden rhythm
// ============================================================================

use std::f64::consts::PI;
use std::fmt;

/// A process that operates in cycles.
pub trait Rhythm: fmt::Debug {
    type State;

    /// Advance one tick in the cycle
    fn tick(&mut self) -> &Self::State;

    /// Current phase in the cycle [0.0, 1.0)
    fn phase(&self) -> f64;

    /// Full period of the cycle (in ticks)
    fn period(&self) -> u64;

    /// Current tick count
    fn current_tick(&self) -> u64;

    /// The Law of Neutralization: the master can dampen the swing.
    /// Returns the compensating force needed to neutralize the current phase.
    /// This is constant-time execution rendered as principle.
    fn compensate(&self) -> f64 {
        let p = self.phase();
        // The compensation is exactly what cancels the current swing
        -((2.0 * PI * p).sin())
    }

    /// Is the rhythm at a peak (maximum swing)?
    fn at_peak(&self) -> bool {
        let p = self.phase();
        (p - 0.25).abs() < 0.01 || (p - 0.75).abs() < 0.01
    }

    /// Is the rhythm at a zero crossing (transition point)?
    fn at_crossing(&self) -> bool {
        let p = self.phase();
        p < 0.01 || (p - 0.5).abs() < 0.01
    }
}

// ============================================================================
// PENDULUM — The archetypal rhythmic entity
// ============================================================================

/// A Pendulum is the simplest rhythmic system.
/// It swings between -amplitude and +amplitude with a fixed period.
/// The master can dampen the swing (neutralization).
#[derive(Clone)]
pub struct Pendulum {
    pub amplitude: f64,
    pub period_ticks: u64,
    tick: u64,
    dampening: f64, // 0.0 = free swing, 1.0 = fully neutralized
}

impl Pendulum {
    pub fn new(amplitude: f64, period: u64) -> Self {
        Self {
            amplitude,
            period_ticks: period,
            tick: 0,
            dampening: 0.0,
        }
    }

    /// The master dampens the swing — reducing the amplitude over time.
    /// This is the hermetic art of neutralization.
    pub fn dampen(&mut self, amount: f64) {
        self.dampening = (self.dampening + amount).clamp(0.0, 1.0);
    }

    /// Current value of the pendulum swing
    pub fn value(&self) -> f64 {
        let effective_amp = self.amplitude * (1.0 - self.dampening);
        effective_amp * (2.0 * PI * self.tick as f64 / self.period_ticks as f64).sin()
    }

    /// Reset to the start of the cycle
    pub fn reset(&mut self) {
        self.tick = 0;
    }
}

impl fmt::Debug for Pendulum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Pendulum(amp={:.2}, period={}, tick={}, dampening={:.2}, value={:.4})",
            self.amplitude, self.period_ticks, self.tick, self.dampening, self.value()
        )
    }
}

impl Rhythm for Pendulum {
    type State = f64;

    fn tick(&mut self) -> &f64 {
        self.tick += 1;
        // Rust borrowing: compute value after mutation
        &self.amplitude // placeholder — actual value via .value()
    }

    fn phase(&self) -> f64 {
        (self.tick % self.period_ticks) as f64 / self.period_ticks as f64
    }

    fn period(&self) -> u64 {
        self.period_ticks
    }

    fn current_tick(&self) -> u64 {
        self.tick
    }
}

// ============================================================================
// CYCLE DETECTOR — Find the hidden rhythm in data
// ============================================================================

/// Detect the dominant period (rhythm) in a sequence of values.
/// This is what Shor's algorithm does to break RSA:
/// find the period of f(x) = a^x mod N.
///
/// "The measure of the swing to the right is the measure of the swing to the left."
/// If you know the period, you know the rhythm. If you know the rhythm, you control it.
pub fn detect_period(values: &[f64]) -> Option<PeriodDetection> {
    let n = values.len();
    if n < 4 {
        return None;
    }

    // Autocorrelation — measure how much the signal correlates with
    // shifted versions of itself. The lag with highest correlation
    // (after lag 0) reveals the period.
    let mean: f64 = values.iter().sum::<f64>() / n as f64;
    let variance: f64 = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;

    if variance < 1e-10 {
        return None; // constant signal — no rhythm
    }

    let mut best_lag = 0;
    let mut best_correlation = 0.0_f64;

    for lag in 1..n / 2 {
        let mut correlation = 0.0;
        for i in 0..n - lag {
            correlation += (values[i] - mean) * (values[i + lag] - mean);
        }
        correlation /= (n - lag) as f64 * variance;

        if correlation > best_correlation {
            best_correlation = correlation;
            best_lag = lag;
        }
    }

    if best_correlation > 0.3 {
        Some(PeriodDetection {
            period: best_lag,
            confidence: best_correlation,
            frequency: 1.0 / best_lag as f64,
        })
    } else {
        None // no clear rhythm detected
    }
}

#[derive(Debug)]
pub struct PeriodDetection {
    pub period: usize,
    pub confidence: f64,
    pub frequency: f64,
}

impl fmt::Display for PeriodDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Period: {} ticks (freq: {:.4}, confidence: {:.2}%)",
            self.period,
            self.frequency,
            self.confidence * 100.0
        )
    }
}

// ============================================================================
// SYNCHRONIZATION — Lock two rhythms together
// ============================================================================

/// Check if two rhythms are synchronized (in phase).
pub fn synchronized<R1: Rhythm, R2: Rhythm>(a: &R1, b: &R2) -> bool {
    (a.phase() - b.phase()).abs() < 0.05
}

/// Check if two rhythms are entrained (same period, any phase).
pub fn entrained<R1: Rhythm, R2: Rhythm>(a: &R1, b: &R2) -> bool {
    a.period() == b.period()
}

/// The interference pattern between two rhythms.
/// When synchronized: constructive (amplified).
/// When anti-phase: destructive (cancelled).
pub fn interference(phase_a: f64, phase_b: f64) -> f64 {
    let delta = (phase_a - phase_b).abs();
    // cos(π * delta) goes from +1 (in phase) to -1 (anti-phase)
    (PI * delta * 2.0).cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pendulum_oscillates() {
        let mut p = Pendulum::new(1.0, 100);
        // At tick 0, value should be 0 (sin(0) = 0)
        assert!(p.value().abs() < 1e-10);

        // At quarter period, should be at max amplitude
        for _ in 0..25 {
            p.tick();
        }
        assert!((p.value() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn pendulum_completes_cycle() {
        let mut p = Pendulum::new(1.0, 100);
        for _ in 0..100 {
            p.tick();
        }
        // After full cycle, should be back near zero
        assert!(p.value().abs() < 1e-10);
        assert!((p.phase() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn dampening_reduces_swing() {
        let mut free = Pendulum::new(1.0, 100);
        let mut dampened = Pendulum::new(1.0, 100);
        dampened.dampen(0.5); // 50% dampened

        // Advance both to quarter period (max amplitude)
        for _ in 0..25 {
            free.tick();
            dampened.tick();
        }

        assert!((free.value() - 1.0).abs() < 1e-10);
        assert!((dampened.value() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn full_dampening_is_stillness() {
        let mut p = Pendulum::new(1.0, 100);
        p.dampen(1.0); // fully neutralized

        for _ in 0..50 {
            p.tick();
        }

        // The master has neutralized the swing
        assert!(p.value().abs() < 1e-10);
    }

    #[test]
    fn detect_periodic_signal() {
        // Generate a signal with period 10
        let values: Vec<f64> = (0..200)
            .map(|i| (2.0 * PI * i as f64 / 10.0).sin())
            .collect();

        let detection = detect_period(&values).unwrap();
        // Period should be 10 or a multiple of 10
        assert_eq!(detection.period % 10, 0, "Detected period {} is not a multiple of 10", detection.period);
        assert!(detection.confidence > 0.5);
    }

    #[test]
    fn no_period_in_constant() {
        // Constant signal — no rhythm at all
        let values = vec![42.0; 100];
        assert!(detect_period(&values).is_none());
    }

    #[test]
    fn synchronization() {
        let a = Pendulum::new(1.0, 100);
        let b = Pendulum::new(2.0, 100); // different amplitude, same period
        assert!(synchronized(&a, &b)); // both at phase 0
        assert!(entrained(&a, &b));    // same period
    }

    #[test]
    fn interference_constructive() {
        // Two in-phase rhythms amplify
        assert!((interference(0.0, 0.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn interference_destructive() {
        // Two anti-phase rhythms cancel
        assert!((interference(0.0, 0.5) - (-1.0)).abs() < 1e-10);
    }

    #[test]
    fn compensation_at_peak() {
        let mut p = Pendulum::new(1.0, 100);
        for _ in 0..25 {
            p.tick();
        }
        // At peak (phase 0.25), compensation should be strongly negative
        let comp = p.compensate();
        assert!(comp < -0.9, "Compensation at peak should be negative: {}", comp);
    }
}
