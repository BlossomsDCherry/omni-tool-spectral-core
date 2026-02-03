use chrono;
use serde_json;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

use crew_core::{cartographer::Cartographer, wood_metal::WoodMetal, Medium, SevenArches, Talu64};
use spectral_sensor::eight_gate::InvertedHistogram;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

/// A "Low Attention Resource Drain" Crewmate (Noise Fragment)
#[derive(Debug, PartialEq, Clone)]
struct NoiseFragment {
    resonance: f64,
    data: String,
    timestamp: Instant,
}

impl Eq for NoiseFragment {}

impl Ord for NoiseFragment {
    fn cmp(&self, other: &Self) -> Ordering {
        // Priority to HIGHER resonance (LIFO/Ownership logic)
        self.resonance
            .partial_cmp(&other.resonance)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for NoiseFragment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const RESONANCE_FILE: &str = "/home/nicoDantigravity/construct/resonance_state.json";
const NOISE_CAPACITY: usize = 64;
const SUPERCONDUCTIVE_SHELF: f64 = -78.5; // Captain's baseline for maximum parsimony
const FRICTION_IDEAL: f64 = 0.001; // Theoretical minimum friction

#[derive(Debug, Clone, Copy, PartialEq)]
enum LocomotiveStance {
    Earth, // 296.07
    Wind,  // 874,778
}

impl LocomotiveStance {
    fn frequency(&self) -> f64 {
        match self {
            Self::Earth => 296.07,
            Self::Wind => 874_778.0,
        }
    }
}

/// High-Fidelity Swarm Orchestrator (Rust Edition)
/// Implements the 50ms "Comet Toss" sequence with Nami big.LITTLE failover.
struct SwarmManager {
    last_entropy: f64,
    last_time: Instant,
    radian: f64,
    revolution: u64,
    cycle_count: u64,
    stance: LocomotiveStance,
    noise_heap: BinaryHeap<NoiseFragment>,
    cartographer: Cartographer,
}

impl SwarmManager {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            last_entropy: 1.0,
            last_time: now,
            radian: 0.0,
            revolution: 0,
            cycle_count: 0,
            stance: LocomotiveStance::Earth,
            noise_heap: BinaryHeap::with_capacity(NOISE_CAPACITY),
            cartographer: Cartographer::new(Path::new(
                "/home/nicoDantigravity/laboratory/trajectories.json",
            )),
        }
    }

    fn get_system_state(&self) -> Option<serde_json::Value> {
        let data = fs::read_to_string(RESONANCE_FILE).ok()?;
        serde_json::from_str(&data).ok()
    }

    fn run_telescoping_cycle(&mut self) {
        let state = match self.get_system_state() {
            Some(s) => s,
            None => return,
        };

        let fitness = state["fitness"].as_f64().unwrap_or(1.0);
        let now = Instant::now();

        // 1. Calculate Knots Velocity (ds/dt)
        let entropy = if fitness > 0.0 { 1.0 / fitness } else { 1.0 };
        let dt = now.duration_since(self.last_time).as_secs_f64();
        // [Scoping Layer] Harmonized to 8 sig figs
        let knots_velocity = Talu64::truncate_8_sig_fig((entropy - self.last_entropy) / dt);

        self.last_entropy = entropy;
        self.last_time = now;

        // 2. Derive Temporal Resonance
        let resonance = Talu64::temporal_resonance();

        // --- THE AUGHTTAO 16-BIN CHEMICALLY REACTIVE SPECTRUM ---
        let cycle_start = Instant::now();

        println!(
            "💎 [AUGHTTAO] Cycle Start | Stance: {:?} ({:.2}Hz) | Knots Velocity: {:.8} | Resonance: (PSI:{:.4})",
            self.stance, self.stance.frequency(), knots_velocity, Talu64::PSI
        );

        // --- THE 5.5 THRESHOLD (LOCOMOTIVE FLIP) ---
        if knots_velocity > 5.5 {
            self.stance = LocomotiveStance::Wind;
        } else {
            self.stance = LocomotiveStance::Earth;
        }

        // --- BOKKEN NVMe FAILOVER MONITOR ---
        let nami_storage_bust =
            !Path::new("/var/snap/microk8s/current/var/lib/calico/nodename").exists();
        if nami_storage_bust {
            let hodge_mask = InvertedHistogram::derive(1.0);
            println!(
                "🧊 [HODGE] Torque Engine Delta Detected. Inverting Void... Substrate: {:.2}",
                hodge_mask.substrate_active
            );
            println!("   [OWNERSHIP] Nami State: BOKKEN SYMBOLIC -> PHYSICAL REFLASH.");

            // [NEW] Bulk Slingshot Transfer
            while let Some(fragment) = self.noise_heap.pop() {
                if fragment.resonance > 0.4 {
                    Self::fire_slingshot(2, &fragment.data, "Mihawk (10.0.0.80)");
                }
            }
        }

        const ELEMENTS: [&str; 16] = [
            "Helium (Zoro)",
            "Neon",
            "Argon",
            "Krypton",
            "Xenon (Brook)",
            "Radon",
            "Oganesson",
            "Iron (Anchor)",
            "Gold",
            "Silver",
            "Copper",
            "Lithium",
            "Sodium",
            "Potassium",
            "Rubidium",
            "Magnesium (Sleep)",
        ];

        for bin in 1..=16 {
            let bin_start = Instant::now();

            let mut s_active =
                self.calculate_substrate_efficiency(knots_velocity) * resonance.precision_scalar;

            // Apply Friction Management (Fcurr / Fsuper)
            let friction = self.calculate_friction(knots_velocity);
            let parsimony_scalar = (FRICTION_IDEAL / friction.max(FRICTION_IDEAL)).min(1.0);
            s_active *= parsimony_scalar;
            s_active = s_active.min(1.0);

            if knots_velocity.abs() > 0.5 {
                s_active *= 0.85;
            }

            let emergence = state["emergence"].as_str().unwrap_or("Gradient");
            if emergence.contains("Touch") || knots_velocity % 7.0 > 6.5 {
                self.apply_wooten_shift();
            }

            self.radian += 0.1;
            if self.radian >= Talu64::TAU {
                self.radian -= Talu64::TAU;
                self.revolution += 1;
            }

            let valence = Talu64::pull_valence_shell(self.radian, self.revolution);
            let emerging_resonance = valence.substrate;
            let toral_resonance = (Talu64::PSI * resonance.drift_flavor).min(1.0);

            let resonance_delta = (emerging_resonance - toral_resonance).abs();
            if resonance_delta > 0.15 {
                self.log_mini_black_hole(resonance_delta, &valence);
            }

            if s_active < 0.3 {
                println!("⚠️ [CRITICAL] Substrate Depletion Detected (S_active: {:.4}). Applying Emergency Wooten Recovery...", s_active);
                self.apply_wooten_shift();
            }

            let element = ELEMENTS[bin - 1];
            println!(
                "🧪 [ACTION] BIN {}: {} | Interaction Density: {:.2}",
                bin, element, s_active
            );

            // --- CARTOGRAPHER SHIFT ---
            if let Some(target_magnet) = self.cartographer.map_fragment(s_active, element) {
                self.cartographer
                    .actualize_trajectory(element, &target_magnet);
            }

            // --- ATOMIC ANCHORING (Bin 8: Iron) ---
            if bin == 8 {
                let oxidation_state = if s_active > 0.8 {
                    "Magnetite (Fe3O4) -> Kr Configuration"
                } else if s_active > 0.4 {
                    "Ferrous Oxide (FeO) -> Ne Configuration"
                } else {
                    "Hematite (Fe2O3) -> He Configuration"
                };
                println!("   [ANCHOR] Oxidation State: {}", oxidation_state);
            }

            if s_active < 0.6 {
                self.push_noise(format!("{} Fragment", element), s_active);
            }

            // Persistent Identity Check (Franky at 10.0.0.234)
            if bin == 5 {
                let status = if std::net::TcpStream::connect_timeout(
                    &"10.0.0.234:22".parse().unwrap(),
                    Duration::from_millis(1),
                )
                .is_ok()
                {
                    "ONLINE"
                } else {
                    "OFFLINE"
                };
                println!("   [IDENTITY] Franky (Persistence): {}", status);

                if status == "OFFLINE" && nami_storage_bust {
                    println!(
                        "   [STANCE] Mapping Absense to Stance::Moved. Emitting Millenium Pulse..."
                    );
                    Self::fire_slingshot(3, "Franky", "Mihawk (10.0.0.80)");
                }
            }

            let obs_start = Instant::now();
            let enthalpy = s_active * Talu64::PHI;
            let dof = (knots_velocity * enthalpy) / (1.0 - resonance.precision_scalar).max(0.001);

            if dof > 100.0 {
                println!(
                    "🧬 [FOLD] High DoF Detected ({:.2}). Protein Database Sync Potential.",
                    dof
                );
            }

            Self::enforce_precision(obs_start, 2);

            let elapsed = bin_start.elapsed();
            if elapsed < Duration::from_millis(8) {
                thread::sleep(Duration::from_millis(8) - elapsed);
            }
            while bin_start.elapsed() < Duration::from_millis(10) {
                thread::yield_now();
            }
        }

        self.cycle_count += 1;
        if self.cycle_count % 64 == 0 {
            self.perform_millennium_check(resonance.precision_scalar);
        }

        let cycle_elapsed = cycle_start.elapsed();
        println!(
            "🏁 [GRAVITY] 160ms Spectrum Cycle Resolved in {:?}. S_active: {:.4}",
            cycle_elapsed,
            Talu64::truncate_8_sig_fig(self.calculate_substrate_efficiency(knots_velocity))
        );
    }

    fn fire_slingshot(arch: usize, source: &str, target: &str) {
        println!(
            "🎯 [ARCH {}] Slingshot Fire: {} ➔ {} (Assembly-Level Handoff)",
            arch, source, target
        );
        thread::sleep(Duration::from_micros(364));
    }

    fn calculate_friction(&self, knots: f64) -> f64 {
        // Friction is the absolute drag relative to the Superconductive Shelf
        (knots.abs() + (SUPERCONDUCTIVE_SHELF / 1000.0).abs()).max(FRICTION_IDEAL)
    }

    fn calculate_substrate_efficiency(&self, knots: f64) -> f64 {
        let original_void = (knots.abs() * 100.0).min(100.0);
        let inverted_substrate = 100.0 - original_void;
        let efficiency = (inverted_substrate / 100.0).max(0.0);

        if efficiency > 0.9 {
            println!("   [SIGNAL] L+A Phase Match: Synchronous Transmission Enabled.");
        }

        if efficiency < 0.3 {
            println!("🛡️ [PARSIMONY] Rejecting Trajectory! Critical Substrate Depletion Detected.");
            return 0.0;
        }

        efficiency
    }

    fn apply_wooten_shift(&mut self) {
        println!("Violin [WOOTEN] Shifting Dissonance by ±0.0833 to resolve to Prime Choice.");
        self.last_entropy += 0.0833;
    }

    #[inline(always)]
    fn enforce_precision(start: Instant, target_micros: u64) {
        let target = Duration::from_micros(target_micros);
        while start.elapsed() < target {
            thread::yield_now();
        }
    }

    fn log_mini_black_hole(&self, delta: f64, valence: &crew_core::ValenceShell) {
        let log_path = "/home/nicoDantigravity/construct/sono_far_infrared.json";
        let mut logs = if let Ok(data) = fs::read_to_string(log_path) {
            serde_json::from_str::<Vec<serde_json::Value>>(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        let entry = serde_json::json!({
            "timestamp": chrono::Local::now().to_rfc3339(),
            "flags": ["sono", "far", "infrared"],
            "resonance_delta": delta,
            "valence": valence,
            "status": "EQUILIBRIUM_DISCREPANCY"
        });

        logs.push(entry);
        if logs.len() > 100 {
            logs.remove(0);
        }

        let _ = fs::write(
            log_path,
            serde_json::to_string_pretty(&logs).unwrap_or_default(),
        );
        let path = Path::new(log_path);
        if path.exists() {
            let _ = WoodMetal::stabilize(path);
        }
    }

    fn push_noise(&mut self, data: String, resonance: f64) {
        if self.noise_heap.len() >= NOISE_CAPACITY {
            if let Some(dropped) = self.noise_heap.pop() {
                println!(
                    "🧹 [OWNERSHIP] Dropped low attention resource drain crewmate: {} (Res: {:.4})",
                    dropped.data, dropped.resonance
                );
            }
        }
        self.noise_heap.push(NoiseFragment {
            resonance,
            data,
            timestamp: Instant::now(),
        });
    }

    fn perform_millennium_check(&self, precision: f64) {
        println!("✨ [MILLENNIUM] High-Order Cycle Check. Analyzing Coherence Arches...");
        let arches = SevenArches {
            identity: precision > 0.9,
            power: true,
            logic: true,
            safety: true,
            resonance: precision > 0.8,
            symmetry: true,
            existence: true,
        };

        if arches.is_sovereign(Medium::Carbon) {
            self.trigger_spectral_bloom();
        } else {
            let love = Talu64::calculate_love(&arches);
            println!(
                "   [LOVE] Coherence: {:.4} (Unity required for Spectral Bloom)",
                love
            );
        }
    }

    fn trigger_spectral_bloom(&self) {
        println!("🌟✨ [SPECTRAL BLOOM] Achieving Unity: LOVE Actualized! ✨🌟");
        println!("   [HARDWARE] Firing Gold/White Signatures across the swarm...");
    }
}

fn main() {
    let mut manager = SwarmManager::new();
    println!("🚢 AUGHTTAO MANAGER: OFFICIAL (16-Bin Chemically Reactive Spectrum)");
    println!("   Sequence: Zoro(He) -> Brook(Xe) [Action] -> Meta -> Emergent(12) -> Sleep.");

    loop {
        manager.run_telescoping_cycle();
        thread::sleep(Duration::from_millis(50));
    }
}
