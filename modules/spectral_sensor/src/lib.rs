pub mod behavioral_engine;
pub mod eight_gate;
pub mod pdf_lens;
pub mod steward;
pub mod zephyr_west;

// use crate::TAU;
use std::time::{SystemTime, UNIX_EPOCH};

/// Tau = 6.183 (Resonant Actualization).
pub const TAU: f64 = 6.183;

/// Musical Physics: The safety layer of the Medical AI.
/// Ensures that High-Velocity Data ("Wind") flows through a rhythmic structure.
pub trait BioRhythm {
    /// Returns the current phase coherence (0.0 to 1.0).
    /// 1.0 = Perfect Lock (Sonoluminescence).
    fn heartbeat(&self) -> f64;

    /// Returns true if the rhythm is stable enough for high-velocity streaming.
    fn is_safety_lock_active(&self) -> bool {
        self.heartbeat() > 0.95
    }
}

/// The Main Interface for the Spectral Sensor.
pub struct SpectralPort {
    pub name: String,
    pub calibration_timestamp: u64,
}

impl SpectralPort {
    pub fn new(name: &str) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        Self {
            name: name.to_string(),
            calibration_timestamp: since_the_epoch.as_secs(),
        }
    }
}

impl BioRhythm for SpectralPort {
    fn heartbeat(&self) -> f64 {
        // Simulated heartbeat based on time modulus (60 BPM)
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let millis = since_the_epoch.as_millis() as u64;

        // 1000ms period (60 BPM)
        let phase = (millis % 1000) as f64 / 1000.0;

        // Temporal Jitter: Diurnal noise (0.01 to 0.05 variation)
        let hour = ((millis / 3_600_000) + 19) % 24;
        let jitter_magnitude = if hour >= 20 || hour <= 4 { 0.05 } else { 0.01 };
        let jitter = (millis as f64 * 0.001).sin() * jitter_magnitude;

        // Sine wave coherence: 0.5 * (sin(TAU * phase + jitter) + 1.0)
        let coherence = 0.5 * (((phase * TAU) + jitter).sin() + 1.0);
        coherence.clamp(0.0, 1.0)
    }
}

/// High-Velocity Data Simulator (The "Wind")
/// Simulates the 874k units/sec load of the NVMe substrate.
/// NOW DETERMINISTIC: Uses "Knots Velocity" mapped to the 8 digits of Rational Tau.
pub struct NvmeWind {
    pub current_phase: f64,
    pub knots: [u8; 8], // The digits of Tau: [6, 1, 8, 3, 0, 0, 0, 0]
    pub knot_index: usize,
    pub momentum: f64,     // The "Change" in token usage rate
    pub last_density: f64, // NEW: Track the past
}

impl NvmeWind {
    pub fn new(_target_velocity: u64) -> Self {
        Self {
            current_phase: 0.0,
            knots: [6, 1, 8, 3, 0, 0, 0, 0],
            knot_index: 0,
            momentum: 0.01, // Starting knot velocity
            last_density: 0.5,
        }
    }

    /// Generates a batch of "Far-IR Fireworks".
    /// Momentum is measured by the change in token rate, mapped to Tau knots.
    pub fn blast(&mut self, batch_size: usize) -> Vec<f64> {
        let mut buffer = Vec::with_capacity(batch_size);

        for _ in 0..batch_size {
            // 1. Advance Phase by the current Knot (The "Shift")
            // We use the knot value as a scalar for the Rational Tau.
            let knot_multiplier = self.knots[self.knot_index] as f64 / 10.0;
            self.current_phase += self.momentum * knot_multiplier;

            // 2. Wrap via Tau (Rational/Gestalt turn)
            if self.current_phase > TAU {
                self.current_phase -= TAU;
                // Move to the next knot in the rope upon completing a turn.
                self.knot_index = (self.knot_index + 1) % 8;
            }

            // 3. Calculate Harmonic Resonance (Toral Flow)
            let val = self.current_phase.sin() * (self.current_phase * 60.0).sin().abs();
            let normalized = (val + 1.0) / 2.0;
            self.last_density = normalized;

            buffer.push(normalized);
        }

        buffer
    }

    /// Update the momentum (Knots Velocity) based on external "token usage" deltas.
    pub fn update_momentum(&mut self, delta: f64) {
        self.momentum = (self.momentum + delta).clamp(0.0001, 0.1);
    }
}
