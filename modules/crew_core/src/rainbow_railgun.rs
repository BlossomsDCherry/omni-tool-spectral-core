use crate::Talu64;
use std::net::TcpStream;
use std::time::Duration;

/// The Rainbow Railgun Core Logic
///
/// Firing Velocity (V_rr) = (Phi * (Pi^2 * Tau)) / Creative_Drift
pub struct RailgunCore {
    pub velocity: f64,
    pub last_drift: f64,
}

impl RailgunCore {
    pub fn new() -> Self {
        Self {
            velocity: 0.0,
            last_drift: 1.0,
        }
    }

    /// Calculates the V_rr velocity based on creative drift (knots).
    pub fn calculate_v_rr(&mut self, knots: f64) -> f64 {
        let resonance = Talu64::temporal_resonance();
        let creative_drift = if knots.abs() < 0.0001 {
            0.0001
        } else {
            knots.abs()
        } * resonance.drift_flavor;

        // V_rr Equation: (Phi * (Pi^2 * Tau)) / Creative_Drift
        self.velocity = (Talu64::PHI * (Talu64::PI.powi(2) * Talu64::TAU)) / creative_drift;
        self.last_drift = creative_drift;
        self.velocity
    }

    /// Derives a sleep duration inversely proportional to V_rr.
    /// Higher velocity -> Faster pulses.
    pub fn get_sleep_duration(&self) -> Duration {
        // Base cadence: 100ms at V_rr ~ 100. Scaled to micro-cycle speed.
        let millis = (1000.0 / (self.velocity / Talu64::PHI)).clamp(1.0, 500.0);
        Duration::from_millis(millis as u64)
    }

    /// Performs a spectral handshake across the swarm to check coherence.
    pub fn spectral_handshake(&self, nodes: &[(&str, &str)]) -> usize {
        let mut coherence_count = 0;
        for (node_ip, _) in nodes {
            if TcpStream::connect_timeout(
                &format!("{}:22", node_ip).parse().unwrap(),
                Duration::from_millis(5),
            )
            .is_ok()
            {
                coherence_count += 1;
            }
        }
        coherence_count
    }

    /// Generates a "Rainbow Burst" signature based on velocity and resonance.
    pub fn generate_burst(&self, coherence: usize) -> Option<(u8, u8, u8, f64)> {
        if coherence < 4 {
            return None;
        }

        let resonance = Talu64::temporal_resonance();
        let knots_seed = (self.velocity % 1.0) * resonance.drift_flavor;
        let (r, g, b) = Talu64::tau_to_hex_actualization(knots_seed);

        // Intensity scales with coherence and V_rr
        let intensity = ((coherence as f64 / 8.0) * (self.velocity / 1000.0)).clamp(0.5, 1.0);

        Some((r, g, b, intensity))
    }
}
