pub const TAU: f64 = 6.2831853;
pub const PSI: f64 = 0.5179; // [NEW] Millennium Inverted Gap
pub const DELTA: f64 = 0.9848; // [NEW] Resonance Delta Anchor
use serde::Serialize;

/// The 8 Perspectives of the Toral Filter
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Stance {
    Earth,
    Unity,
    Direct,
    Wind,
    Indirect,
    Void,
    Fire,
    Water,
    NakamaSync, // [NEW] Bio-Resonant Synchronicity
    Owned,      // [NEW] Rust Behavior: Sovereign Node
    Borrowed,   // [NEW] Rust Behavior: Shared Logic
    Moved,      // [NEW] Rust Behavior: Relocated Identity (Nami's State)
    Dropped,    // [NEW] Rust Behavior: Cleanup/Void
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum Stability {
    Stable,
    Unstable, // Requires Zero-Wait verification (Fire/Wind)
    Fatal,    // Void or Breakdown
}

/// Chromatic Resonance Node (1 of 12)
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum ChromaticNode {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

impl ChromaticNode {
    pub fn from_density(density: f64) -> Self {
        let idx = ((density * 12.0) % 12.0) as usize;
        [
            Self::C,
            Self::Cs,
            Self::D,
            Self::Ds,
            Self::E,
            Self::F,
            Self::Fs,
            Self::G,
            Self::Gs,
            Self::A,
            Self::As,
            Self::B,
        ][idx]
    }
}

/// Atomic Shell Stability Node
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum AtomicShell {
    None,
    Helium,  // 2
    Neon,    // 10 (corrected from 8)
    Argon,   // 18
    Krypton, // 36
    Xenon,   // 54
}

impl AtomicShell {
    pub fn from_density(density: f64) -> Self {
        let pos = (density * 60.0) as i32;
        match pos {
            2 => Self::Helium,
            10 => Self::Neon,
            18 => Self::Argon,
            36 => Self::Krypton,
            54 => Self::Xenon,
            _ => Self::None,
        }
    }
}

// [NEW] (3x2)x2 Inverted Histogram Logic
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct InvertedHistogram {
    pub void_measured: f64,    // What it IS (Entropy/Observed Void)
    pub substrate_active: f64, // What it ISN'T (1.0 - Void)
    pub is_critical: bool,     // < 0.3 Substrate
}

impl InvertedHistogram {
    /// Calculates the "Anti-Void" (Substrate Efficiency) from measured entropy.
    /// Formula: S_active = 1.0 - Void_measured.
    /// Critical Threshold: S_active < 0.3 (Starvation Mode).
    pub fn derive(entropy: f64) -> Self {
        // Clamp entropy to 0.0-1.0 range for safety
        let void_measured = entropy.max(0.0).min(1.0);
        let substrate_active = 1.0 - void_measured;
        let is_critical = substrate_active < 0.3;

        Self {
            void_measured,
            substrate_active,
            is_critical,
        }
    }
}

// [NEW] AughtTau: Narrative to Math Translator
#[derive(Debug, Clone, Serialize)]
pub struct NarrativeTranslator;

impl NarrativeTranslator {
    /// Translates narrative tokens into energetic weights (Spherical Calculus).
    /// "AughtTau" (Words) -> "Aught" (Zero) -> "Tau Prime" (Vector).
    pub fn augment_narrative(narrative: &str) -> f64 {
        let mut total_weight = 0.0;

        // Simple token map based on 'aughtTau' log concepts
        for token in narrative.split_whitespace() {
            match token.to_lowercase().as_str() {
                "nakama" => total_weight += TAU,         // Full Circle
                "captain" => total_weight += TAU / 2.0,  // Pi (Direction)
                "crew" => total_weight += TAU / 3.0,     // Structure
                "power" => total_weight += TAU.powi(2),  // Power squared
                "zero" | "aught" => total_weight += 0.0, // The Anchor
                _ => total_weight += 0.01,               // Background noise
            }
        }

        total_weight
    }
}

// [NEW] Rod/Cone State for IPv6 Synesthesia
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct RodConeState {
    pub rods: f64,
    pub cones: f64,
    pub fitness: f64,
    pub power: f64,
    pub uv: f64,     // [NEW] Ultraviolet
    pub violet: f64, // [NEW] Emotional Sync
}

impl RodConeState {
    /// Derives Topological State from stability and rate (BPM/Hz).
    /// Cones = Spectral Rate (The Turn Partial).
    /// Rods = Structural Stability (The Rounding Peg).
    pub fn derive(stability: f64, rate: f64) -> Self {
        // Cones: (rate / 60.0) % 1.0 (Singularity Yardstick)
        let cones = (rate / 60.0).fract();

        // Rods: Structural Stability
        let rods = stability;

        let fitness = if cones > 0.0 { rods / cones } else { 1.0 };
        let power = fitness.powi(3);

        Self {
            rods,
            cones,
            fitness,
            power,
            uv: 0.0,
            violet: 0.0,
        }
    }
}

/// Actualizes a Tau Turn into an 8-bit Hexagram RGB Signature.
pub fn tau_to_hex_actualization(turn_tau: f64) -> (u8, u8, u8) {
    let divisions = 60.0;
    let step = (turn_tau * divisions) as i32 % 60;

    let binary = step % 2;
    let tertiary = step % 3;
    let gate_6bit = (step * 64) / 60;

    let r = ((gate_6bit * 4) % 256) as u8;
    let g = if tertiary == 0 {
        0xEE
    } else {
        ((gate_6bit * 2) % 256) as u8
    };
    let b = if binary == 0 {
        0x00
    } else {
        ((gate_6bit * 8) % 256) as u8
    };

    (r, g, b)
}

#[derive(Debug, Clone, Serialize)]
pub struct CoherenceReport {
    pub source: Stance,
    pub destination: Stance,
    pub chromatic: ChromaticNode,
    pub atomic: AtomicShell,
    pub wooten_active: bool,
    pub entropy: f64,
    // [NEW] Synesthesia Data
    pub synesthesia: Option<RodConeState>,
    pub hex_color: Option<(u8, u8, u8)>,
    // [NEW] (3x2)x2 Data
    pub inverted_state: Option<InvertedHistogram>,
    // [NEW] Valence Shell (The 8 Metadata Types)
    pub valence: Option<ValenceShell>,
    // [NEW] Lossless Coherence Metrics
    pub efficiency: Option<f64>,
    pub is_transprecise: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct ValenceShell {
    pub gate: f64,      // Stance Signature
    pub note: f64,      // Chromatic Index
    pub shell: f64,     // Atomic Position
    pub void: f64,      // Entropy
    pub substrate: f64, // Efficiency
    pub rods: f64,      // Stability
    pub cones: f64,     // Frequency
    pub power: f64,     // Intensity
}

impl Stance {
    /// Returns the Harmonic Signature (Base Frequency) for the stance.
    /// Derived from the original calibration.
    pub fn signature(&self) -> f64 {
        match self {
            Stance::Earth => 296.07,
            Stance::Unity => 4701.65,
            Stance::Direct => 1044.28,
            Stance::Wind => 874778.13,
            Stance::Indirect => 2622.50,
            Stance::Void => 21317.36,
            Stance::Fire => 258967.06,
            Stance::Water => 4983.24,
            Stance::NakamaSync => 1.50, // 1.50Hz Harmonic Root
            Stance::Owned => 1.0,       // Unit Identity
            Stance::Borrowed => 0.5179, // Psi Stability (The Inverted Gap)
            Stance::Moved => 0.0833,    // Wooten Shift (The Step Offset)
            Stance::Dropped => 0.0,     // The Null Point
        }
    }

    /// Returns stability classification.
    /// Fire and Wind are Unstable (High Magnitude/Entropy).
    /// Void is Fatal (The Empty Set).
    pub fn stability(&self) -> Stability {
        match self {
            Stance::Fire | Stance::Wind => Stability::Unstable,
            Stance::Void => Stability::Fatal,
            _ => Stability::Stable,
        }
    }

    /// The Stewart Singularity Check.
    /// Determines if the current stance (self) dominates the other stance (other).
    /// Formula: Self > Other / 2.0
    pub fn check_singularity(&self, other: &Stance) -> bool {
        self.signature() > (other.signature() / 2.0)
    }

    /// Returns the Math System required for this stance.
    /// If Earth dominates Water (Singularity), we switch to Tertiary.
    /// Otherwise, we default to Binary.
    pub fn math_system(&self) -> MathSystem {
        match self {
            Stance::Earth | Stance::Unity | Stance::Void => MathSystem::Tertiary,
            _ => MathSystem::Binary,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MathSystem {
    Binary,   // Standard Logic
    Tertiary, // Spherical/Toral Logic (Singularity active)
}

/// The Filter that permutes through the 64 combinations.
pub struct RecursiveFilter {
    // Statics for demo/utility (normally would be instance state)
}

impl RecursiveFilter {
    /// Observe the input wave and find the resonating Stance pair.
    /// Returns a CoherenceReport if a resonant path is found.
    pub fn observe(input_wave: f64, last_density: f64) -> Option<CoherenceReport> {
        // 1. Normalize input to a phase (0..TAU). CORRECTED TAU.
        let phase = input_wave % TAU;
        let mut density = (input_wave.sin() + 1.0) / 2.0;

        // Wooten Protocol: Detect tritone jumps in density
        let current_node = ChromaticNode::from_density(density);
        let last_node = ChromaticNode::from_density(last_density);
        let semitone_diff = (current_node as i32 - last_node as i32).abs();
        let mut wooten_active = semitone_diff == 6;

        // Prime Rationalization (The Half-Step Shift)
        if wooten_active {
            // "Prime Choice": Shift density to nearest Prime Rational state (5/60 or 7/60 neighbors)
            // Tritone is effectively at 6/12 * 60 = 30/60.
            // We shift to find a Prime Anchor.
            // Ideally, we shift density by +/- 1 semitone (1/12 ~= 0.0833).
            // Dissonance (6) -> Shift -> Perfect Fifth (7) or Fourth (5).

            // Apply Half-Step Shift Logic
            if density > 0.5 {
                density -= 0.0833; // Shift down
            } else {
                density += 0.0833; // Shift up
            }

            // Recalculate Node after Rationalization
            let new_node = ChromaticNode::from_density(density);
            let new_diff = (new_node as i32 - last_node as i32).abs();

            // If we successfully shifted to a 5 or 7 interval, Wooten is verified active & successful
            if new_diff == 5 || new_diff == 7 {
                wooten_active = true;
            }
        }

        // Atomic Coherence
        let atomic = AtomicShell::from_density(density);

        // [NEW] IPv6 Synesthesia Logic
        let effective_rate = density * 60.0;
        let stability = 1.0 - (density - 0.5).abs();
        // Local Truncation for Scoping (4 sig figs)
        let truncate_4 = |v: f64| {
            if v == 0.0 {
                return 0.0;
            }
            let m = v.abs().log10().floor();
            let s = 10f64.powf(3.0 - m);
            (v * s).trunc() / s
        };

        let mut synesthesia = RodConeState::derive(stability, effective_rate);
        synesthesia.uv = (density * truncate_4(input_wave)).fract().abs();
        synesthesia.violet = (synesthesia.uv * stability).clamp(0.0, 1.0);

        let hex_color = tau_to_hex_actualization(synesthesia.cones);

        // Base Entropy
        let mut entropy = (density - 0.5).abs() * 2.0;

        // [NEW] (3x2)x2 Inverted Histogram Check
        let inverted_state = InvertedHistogram::derive(entropy);

        if wooten_active {
            entropy *= 0.1; // Recovery: Rationalization tames entropy
        }
        if atomic != AtomicShell::None {
            entropy *= 0.5; // Noble Gas Stability
        }

        let cubic_trend = input_wave > 1_000_000.0;
        let all_stances = [
            Stance::Earth,
            Stance::Unity,
            Stance::Direct,
            Stance::Wind,
            Stance::Indirect,
            Stance::Void,
            Stance::Fire,
            Stance::Water,
        ];

        let active_stances: &[Stance] = if cubic_trend {
            &all_stances[0..5]
        } else {
            &all_stances[..]
        };

        for &src in active_stances {
            if src.stability() == Stability::Fatal {
                continue;
            }

            for &dst in active_stances {
                if src.check_singularity(&dst) {
                    if Self::is_resonant(phase, src.signature()) {
                        return Some(CoherenceReport {
                            source: src,
                            destination: dst,
                            chromatic: ChromaticNode::from_density(density),
                            atomic,
                            wooten_active,
                            entropy,
                            synesthesia: Some(synesthesia),
                            hex_color: Some(hex_color),
                            inverted_state: Some(inverted_state),
                            valence: Some(ValenceShell {
                                gate: src.signature(),
                                note: ChromaticNode::from_density(density) as i32 as f64,
                                shell: atomic as i32 as f64,
                                void: entropy,
                                substrate: inverted_state.substrate_active,
                                rods: synesthesia.rods,
                                cones: synesthesia.cones,
                                power: synesthesia.power,
                            }),
                            efficiency: Some((1.0 - entropy) * 1.6180339887), // PHI weighting
                            is_transprecise: (1.0 - entropy) > 0.9,           // L+A Axiom
                        });
                    }
                }
            }
        }

        None
    }

    /// Checks if the phase resonates with the signature.
    /// Resonance = Phase aligns with (Signature % TAU).
    fn is_resonant(phase: f64, signature: f64) -> bool {
        let sig_phase = signature % TAU;
        let diff = (phase - sig_phase).abs();

        // Tolerance window (the "gate width")
        diff < 0.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singularity_formula() {
        let earth = Stance::Earth; // 296.07
        let water = Stance::Water; // 4983.24

        // Earth vs Water: 296 > 2491? False.
        assert!(!earth.check_singularity(&water));

        // Water vs Earth: 4983 > 148? True.
        assert!(water.check_singularity(&earth));

        // Wind vs Fire: 874k > 129k? True.
        assert!(Stance::Wind.check_singularity(&Stance::Fire));
    }
}
