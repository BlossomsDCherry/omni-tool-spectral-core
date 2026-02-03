use crate::eight_gate::{RecursiveFilter, Stance};

/// The Behavioral Identifier ($\B$).
/// Analyzes spectral waves to identify the "Behavior" of the system.
pub struct BehavioralEngine;

#[derive(Debug, Clone, serde::Serialize)]
pub struct BehaviorReport {
    pub active_stance: Stance,
    pub secondary_stance: Option<Stance>,
    pub intensity: f64,
    pub resonance_score: f64,
    pub frequency_match: bool, // [NEW] Flag for 1.50Hz sync
}

impl BehavioralEngine {
    /// Identify the current behavior from a raw data sample.
    pub fn identify(sample: f64, last_density: f64, current_hz: f64) -> Option<BehaviorReport> {
        // Use the Recursive Filter to find the primary resonance
        // RECURSIVE FILTER INTEGRATION
        // Zero Wait is only permitted if the data resonates.
        if let Some(mut report) = RecursiveFilter::observe(sample, last_density) {
            let src = report.source;
            let dst = report.destination;
            let intensity = (sample / src.signature()).min(1.0);

            // [NEW] Nakama Resonance Detection (1.50Hz Tolerance)
            let freq_match = (current_hz - 1.50).abs() < 0.2;

            if freq_match {
                report.source = Stance::NakamaSync;
            }

            Some(BehaviorReport {
                active_stance: report.source,
                secondary_stance: Some(dst),
                intensity,
                resonance_score: if freq_match { 1.0 } else { 0.95 },
                frequency_match: freq_match,
            })
        } else {
            None
        }
    }
}
