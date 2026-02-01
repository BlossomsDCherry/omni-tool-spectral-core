pub const TAU: f64 = 6.2831853;
pub const PI: f64 = 3.1415927; // The Revolution Limit
pub const PHI: f64 = 1.6180340; // Resonance Scaling
pub const E: f64 = 2.7182818; // Epsilon/Revolution Cap
pub const PSI: f64 = 0.5; // Rotational/Imaginary Core (The i swap)
pub const I: f64 = 1.0; // Cognitive/System-wide Bridge (The Psi swap)

#[derive(Debug, PartialEq)]
pub enum MathSystem {
    Arabic, // Includes Zero (0 exists)
    Roman,  // No Zero (0 does not exist, starts at I)
}

#[derive(Debug)]
pub enum NavigationState {
    VoidExtraction,       // The extraction of the "Treadmill Recursion" field.
    GestaltActualization, // Stable entrainment.
    CometSlingshot,       // High-velocity transition.
    BiologicalResonance,  // Physical "Human Resistor" handshake.
    DimensionalFolding,   // Dimension 6 decision point.
    EnergyActualization,  // Dimension 8 sovereign energy (Tau).
    SpaceTimeEvent,       // The transcendental singularity (Tau).
    TauPrimeEmergence,    // The first emergent cycle (Tau').
    TripleSlitCoherence { model: PerceptionModel }, // Perception-driven synthesis.
    VioletFanActualization, // Dimension 7 rotational fan (Line to Circle).
    NeuroHarmonicActualization, // The Brain on Music (Ultimate Synchrony).
    SovereignInternetNavigation, // Dimension 11 IPv6 Shell Actualization.
    RainbowRailgunTrajectory, // High-speed IPv6 trajectory (D11+).
    RecurrentOrbit,       // High-order stable orbit (D11+).
    OrbitalRecovery,      // The return from Comet Slingshot to Orbit.
    ThermodynamicConvergence, // (M x T) x T Order Emergence.
}

#[derive(Debug, Clone, Copy)]
pub enum PerceptionModel {
    Bit,     // Deterministic/Particle.
    BitWave, // Probabilistic/Timing.
    BitMap,  // Integrated Wave + Timing (Sovereign).
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HarmonicState {
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

impl HarmonicState {
    pub fn shift_fifth(&self) -> Self {
        use HarmonicState::*;
        let notes = [C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B];
        let current_idx = notes.iter().position(|&n| n == *self).unwrap();
        // Shift by 7 semitones (The Perfect Fifth)
        let next_idx = (current_idx + 7) % 12;
        notes[next_idx]
    }

    pub fn shift_half_step(&self) -> Self {
        use HarmonicState::*;
        let notes = [C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B];
        let current_idx = notes.iter().position(|&n| n == *self).unwrap();
        // Shift by 1 semitone (The Half-Step)
        let next_idx = (current_idx + 1) % 12;
        notes[next_idx]
    }

    pub fn from_density(density: f64) -> Self {
        use HarmonicState::*;
        let notes = [C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B];
        let idx = ((density * 12.0) % 12.0) as usize;
        notes[idx]
    }

    pub fn semitone_index(&self) -> usize {
        use HarmonicState::*;
        let notes = [C, Cs, D, Ds, E, F, Fs, G, Gs, A, As, B];
        notes.iter().position(|&n| n == *self).unwrap()
    }
}

pub struct SoftFPGA {
    pub system: MathSystem,
    pub relative_entropy: f64, // The delta between the "Ship" and the "Current".
    pub last_event_density: f64, // Tracking the Space-Time Delta
    pub harmonic_state: HarmonicState, // Current note in the Circle of Fifths
    pub last_note: HarmonicState, // Tracking the Past
}

impl SoftFPGA {
    pub fn new() -> Self {
        Self {
            system: MathSystem::Arabic,
            relative_entropy: 0.0,
            last_event_density: 0.0,
            harmonic_state: HarmonicState::C,
            last_note: HarmonicState::C,
        }
    }

    /// The Survivor's Calculus: Crossing the Gap.
    /// Grounded in the Zero-Wait Handshake and Relative Entropy.
    /// NOW SUPPORTING: "Hugging the Void" (High-speed trajectory correction).
    pub fn cross_gap(&mut self, density: f64) -> NavigationState {
        // 1. Calculate Relative Entropy: The delta between System Sync and Physical Presence.
        // Hugging the Void logic: Any received density is a course correction.
        self.relative_entropy = (density - 0.5).abs() * 2.0;

        // 12 Points of Resonance: Mapping the Chromatic Scale (Past/Present/Future)
        let current_note = HarmonicState::from_density(density);
        let semitone_diff =
            (current_note.semitone_index() as i32 - self.last_note.semitone_index() as i32).abs();

        // Tritone Detection: The most uncomfortable jump (6 semitones = PI distance)
        if semitone_diff == 6 {
            println!("🌀 TRITONE DETECTED. Engaging WOOTEN PROTOCOL (Half-Step Recovery).");
            // Perform a micro-shift to find the "right note"
            self.harmonic_state = self.harmonic_state.shift_half_step();
            // Reduce entropy via the half-step micro-correction
            self.relative_entropy *= 0.1;
            println!("   [WOOTEN] Always a half-step away. Coherence regained.");
        }
        self.last_note = current_note;

        // 2. Detection of proximity to the Void (0-collapse).
        if density <= 0.05 {
            println!("⚠️ VOID FIELD DETECTED. Extracting via ROMAN System (Existence Exists)...");
            self.system = MathSystem::Roman;
        } else if self.relative_entropy > 0.8 {
            // High entropy = High velocity correction required
            println!("🏎️ HUGGING THE VOID. Full Speed course correction enabled.");
            return NavigationState::CometSlingshot;
        } else if self.relative_entropy < 0.00000001 && self.last_event_density.abs() > 0.5 {
            // Orbital Recovery logic: Return from Toss to Orbit when precision is peak
            println!("🌀 ORBITAL RECOVERY. Comet Slingshot returning to Recurrent Orbit.");
            return NavigationState::OrbitalRecovery;
        }

        // 3. Thermodynamic Order Emergence (Power as Adaptive Fitness)
        // Calculating P = (x10/x60)^3. Here x10 is structural density, x60 is the temporal grid.
        let x10 = density;
        let x60 = 1.0 / 60.0; // Standard 1/60 timing grid
        let adaptive_power = (x10 / x60).powi(3);

        if adaptive_power >= 1.0 {
            println!(
                "🏆 THERMODYNAMIC CONVERGENCE. Power as Adaptive Fitness: {:.4}",
                adaptive_power
            );
            // Return early to signify the Order Emergence (D3 Gate)
            if density < 0.1 {
                // High-precision near-void emergence
                return NavigationState::ThermodynamicConvergence;
            }
        }

        match self.system {
            MathSystem::Roman => {
                // In Roman math, we anchor at I (1.0) because existence exists.
                let thrust = (self.relative_entropy + 1.0) * TAU;
                if thrust > TAU * 1.5 {
                    NavigationState::CometSlingshot
                } else {
                    NavigationState::VoidExtraction
                }
            }
            MathSystem::Arabic => {
                // Phase-Tracking: Green light (Intent) modulating Red light (Stability).
                let velocity = (density * TAU).sin();

                // Triple-Slit Coherence: Detecting Shifted Peaks
                let shift = TAU / 3.0;
                let peak1 = (density * TAU).sin();
                let peak2 = (density * TAU + shift).sin();
                let peak3 = (density * TAU + 2.0 * shift).sin();

                if peak1.abs() > 0.9 && peak2.abs() > 0.9 && peak3.abs() > 0.9 {
                    println!("🎭 TRIPLE-SLIT COHERENCE DETECTED. Selecting Perception Model...");

                    let model = if self.relative_entropy < 0.2 {
                        println!("⚛️ PERCEPTION: BIT (Particle).");
                        PerceptionModel::Bit
                    } else if self.relative_entropy > 0.6 {
                        println!("🌊 PERCEPTION: BIT WAVE (Timing).");
                        PerceptionModel::BitWave
                    } else {
                        println!("🧬 PERCEPTION: BIT MAP (Wave + Timing). Recombining.");
                        PerceptionModel::BitMap
                    };

                    return NavigationState::TripleSlitCoherence { model };
                }

                // Atomic Coherence: Electron Shell Stability Nodes (2, 10, 18, 26)
                // Scaling the density to a 60-division yardstick.
                // Stable shells: 2 (He), 10 (Ne), 18 (Ar), 26 (Fe - Iron Core).
                let shell_pos = (density * 60.0) as i32;
                let is_atomic_stable =
                    shell_pos == 2 || shell_pos == 10 || shell_pos == 18 || shell_pos == 26;

                if is_atomic_stable {
                    println!(
                        "⚛️ ATOMIC COHERENCE. Stability Node at shell {} detected.",
                        shell_pos
                    );
                    // Reduce friction/entropy at Noble Gas states
                    self.relative_entropy *= 0.5;
                }

                // 0D-8D Progression Logic
                if velocity.abs() > 0.1666 {
                    // ~10/60
                    println!(
                        "📐 DIMENSIONAL FOLDING (D6) DETECTED. Following the Yellow Brick Road?"
                    );
                    return NavigationState::DimensionalFolding;
                }

                if velocity.abs() > 0.5 {
                    // Approaching 8D/Tau
                    println!("⚡ ENERGY ACTUALIZATION (D8). The Book is Open.");

                    // Dimension 10 Check: Circle of Fifths Coherence
                    // Trigger a harmonic shift when velocity peaks
                    let old_state = self.harmonic_state;
                    self.harmonic_state = self.harmonic_state.shift_fifth();
                    println!(
                        "🎵 HARMONIC SHIFT: {:?} -> {:?}",
                        old_state, self.harmonic_state
                    );

                    // Space-Time Event Trigger Prep: Sync with Transcendental Jump
                    // Built-in imaginary layer (I) at the most precise limits
                    let jump_sync = (density * PSI).cos(); // Using PSI as Imaginary Core

                    // Dimension 7 Check: The Harmonic Fan Jump
                    // Using Circle of Fifths (7), Thirds (3/4), and Sevenths to jump through the fan
                    let degrees = (density * 360.0) % 360.0;
                    let is_harmonic_hit = (degrees % 30.0 < 0.1) || // Circle of Fifths (30 degree sectors)
                                         (degrees % 45.0 < 0.1) || // Thirds/Halves alignment
                                         (degrees % 51.4 < 0.1); // Sevenths (360/7)

                    if is_harmonic_hit {
                        println!(
                            "💜 HARMONIC FAN JUMP (D7). Interval aligned at {:.2}°",
                            degrees
                        );

                        // Neuro-Harmonic Trigger: If we are in high-order synchrony
                        if self.relative_entropy < 0.1 && (jump_sync.abs() > 0.99) {
                            println!(
                                "🧠 NEURO-HARMONIC ACTUALIZATION. This is your Brain on Music."
                            );

                            // Dimension 11 Check: Sovereign Internet Navigation
                            // If we hit the absolute peak of synchrony, we enter the IPv6 shell
                            if jump_sync.abs() > 0.999 {
                                println!("🌐 SOVEREIGN INTERNET NAVIGATION. (D11) IPv6 Shell Actualized.");

                                // The Rainbow Railgun: High-speed trajectory logic
                                if self.relative_entropy < 0.01 {
                                    let phi = 1.6180340;
                                    println!(
                                        "🌈 RAINBOW RAILGUN IGNITED. Navigating at protocol speed (Power: Φ={:.4}).",
                                        phi
                                    );

                                    // Golden Spiral Lock: Locking space-time-timing
                                    if self.last_event_density.abs() < 0.001 * phi {
                                        println!("🌀 GOLDEN SPIRAL LOCK. Space-Time-Timing synchronized.");
                                    }

                                    // Recurrent Orbit: High-order stability check
                                    // If we maintain this precision, the treadmill becomes an orbit
                                    if self.last_event_density.abs() < 0.001 {
                                        println!("🌀 RECURRENT ORBIT ACHIEVED. Permanent coherence locked.");
                                        return NavigationState::RecurrentOrbit;
                                    }

                                    return NavigationState::RainbowRailgunTrajectory;
                                }

                                // IPv6 Re-keying (The 10-Unlock)
                                // Rapid security handshake logic
                                for i in 1..=10 {
                                    println!("🔐 LOCK {} UNLOCKED. (Rapid Re-key Handshake)", i);
                                }

                                return NavigationState::SovereignInternetNavigation;
                            }

                            return NavigationState::NeuroHarmonicActualization;
                        }

                        return NavigationState::VioletFanActualization;
                    }

                    if jump_sync.abs() > (1.0 - E) {
                        // Capped by Epsilon (E)
                        let space_time_delta = (density - self.last_event_density).abs();

                        // If delta is too small, we haven't moved in space-time (Tau stable)
                        if space_time_delta < 0.001 {
                            println!("🌌 SPACE-TIME EVENT SYNTHESIZED. Tau identity locked.");
                            return NavigationState::SpaceTimeEvent;
                        } else {
                            // If we have a delta, we emerge into the next cycle (Tau Prime)
                            println!(
                                "🌱 TAU PRIME EMERGED. Space-Time Delta Actualized: {:.4}",
                                space_time_delta
                            );
                            self.last_event_density = density;
                            return NavigationState::TauPrimeEmergence;
                        }
                    }

                    return NavigationState::EnergyActualization;
                }

                if velocity.abs() < 0.1 {
                    NavigationState::VoidExtraction
                } else {
                    NavigationState::GestaltActualization
                }
            }
        }
    }
}
