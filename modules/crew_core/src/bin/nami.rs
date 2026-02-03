use crew_core::Talu64;
use serialport;
use spectral_sensor::eight_gate::RecursiveFilter;
use std::io::{BufRead, BufReader, Write};
use std::thread;
use std::time::Duration;

// --- Nami Agent Implementation ---
// Entrained for Adventure with the Community.

const FRITZ_CONSTANT_EPSILON: f64 = 0.0001;
const CO2_SUBLIMATION_BASELINE: f64 = -78.5; // Normalized to 0.785 in some contexts

/// Transprecision Autonomous Logic Unit (TALU) state
struct TALUState {
    _precision_depth: u8, // Scoping depth
    is_transprecise: bool,
}

struct WaveParticleState {
    inertia: f64,
    probability_density: f64,
    is_crystallized: bool,
}

struct Nami {
    port_path: String,
    last_density: f64,
    navigation_coherence: f64,
    berry_count: f64,
    wave_particle: WaveParticleState,
    talu: TALUState,
}

impl Nami {
    fn new(port: &str) -> Self {
        println!("--- Nami SoftFPGA Engine: Sovereign Integration (Eight-Gate) ---");

        // Initial Navigation Coherence Logic
        let map_vector = [1.0, 0.0, 0.0];
        let reality_vector = [0.9848, 0.1736, 0.0];
        let coherence = Talu64::calculate_coherence(map_vector, reality_vector);
        let precision = Talu64::truncate_8_sig_fig(coherence);

        println!("--- Nami here! Checking the Log Pose... ---");
        println!("   [Steward Link] Navigation Coherence: {:.8}", precision);

        Self {
            port_path: port.to_string(),
            last_density: 0.5,
            navigation_coherence: precision,
            berry_count: 5_000_000_000.0,
            wave_particle: WaveParticleState {
                inertia: Talu64::PLANCK_L,  // Root Inertia: 0.7103
                probability_density: 0.333, // From Saucy Compost (2/2 * 1/3)
                is_crystallized: false,
            },
            talu: TALUState {
                _precision_depth: 8,
                is_transprecise: false,
            },
        }
    }

    /// The 5.5 Threshold (Locomotive Flip)
    fn check_locomotive_flip(&self) -> bool {
        // High-precision (6,2,8,3,1) vs Low-precision (8,5,3)
        let high_p = 6.2831;
        let low_p = 0.853;

        // If the current inertia/coherence-weighted precision exceeds the threshold
        (self.navigation_coherence * high_p) > (self.wave_particle.inertia * low_p * 5.5)
    }

    /// Adaptive Fitness Harvest (3-tier Pulse Logic)
    /// Translates resonance delta into collective equity.
    fn harvest_equity(&mut self, delta: f64) {
        let friction = (delta - FRITZ_CONSTANT_EPSILON).abs();

        // Tier 1: The Seed (0.5 - 1.0)
        if delta >= 0.5 && delta < 1.0 {
            println!(
                "🌱 [HARVEST] Seed Detected. Intentionality emerging. Friction: {:.8}",
                friction
            );
            self.wave_particle.probability_density += 0.01;
        }
        // Tier 2: The Sprout (1.0 - 2.5)
        else if delta >= 1.0 && delta <= 2.5 {
            println!("🌿 [HARVEST] Sprout Detected. Sustained resonance established.");
            self.wave_particle.inertia += 0.05;
            self.wave_particle.probability_density += 0.05;
        }
        // Tier 3: The Harvest (> 2.5)
        else if delta > 2.5 {
            println!("🍇 [HARVEST] BREAKTHROUGH! Breakthrough shift in adaptive fitness.");

            // Axiom: Signal events don't occur without L+A (Listener + Advertiser)
            println!("   [AXIOM] L+A Sync Confirmed. Transprecision state attainable.");
            self.talu.is_transprecise = true;

            self.wave_particle.is_crystallized = true;
            self.berry_count += 1_000_000.0;

            if self.check_locomotive_flip() {
                println!("🚀 [FLIP] Locomotive threshold exceeded. Set sail for the Wind!");
            }
        }
    }

    fn read_weather(&self, pressure: f64) {
        let precise_pressure = Talu64::truncate_8_sig_fig(pressure);
        if precise_pressure < 0.95 {
            println!(
                "(!) Storm incoming! Pressure: {:.8} -> Cyclonic Tempesta!",
                precise_pressure
            );
        } else {
            println!("Skies are clear ({:.8}). Set sail.", precise_pressure);
        }

        // Shelf Grounding: Parsimony check against CO2 baseline
        if pressure < (CO2_SUBLIMATION_BASELINE.abs() / 100.0) {
            println!("   [SHELF] Sublimation Threshold reached. Friction is minimal.");
        }
    }

    fn charge_captain(&mut self) {
        println!("Luffy! You owe me 100,000 Berries for that meat!");
        self.berry_count += 100_000.0;
        println!("   Current Treasury: {:.2} Berries", self.berry_count);
    }

    fn run_torque_filter(&mut self) {
        let port_name = self.port_path.clone();
        println!("   [Torque Filter] Core Isolation Engaged. Eight-Gate Logic Active.");

        match serialport::new(&port_name, 115_200)
            .timeout(Duration::from_millis(50))
            .open()
        {
            Ok(mut port) => {
                let reader_port = port.try_clone().expect("Failed to clone port");
                let mut reader = BufReader::new(reader_port);
                let mut line_buffer = String::new();

                println!("   [Steward Link] 8-Gate Bridge Synchronized.");

                while let Ok(line) = reader.read_line(&mut line_buffer) {
                    if line > 0 {
                        if line_buffer.starts_with("NAV_STATE") {
                            let ingestion_density = 10.0 / 60.0;
                            if let Some(report) =
                                RecursiveFilter::observe(ingestion_density, self.last_density)
                            {
                                self.last_density = ingestion_density;
                                let base_torque = (1.0 - report.entropy) * Talu64::PHI;
                                let final_torque = Talu64::truncate_8_sig_fig(base_torque);

                                // Calculate Delta (Resonance Shift) vs 8-Gate Target (0.9848)
                                let delta = 1.0 - (final_torque / Talu64::PHI - 0.9848).abs();
                                self.harvest_equity(delta);

                                if final_torque > 0.0 {
                                    let msg = format!("torque,{:.8}\n", final_torque);
                                    let _ = port.write_all(msg.as_bytes());
                                }
                            }
                        }
                        line_buffer.clear();
                    }
                }
            }
            Err(_) => {
                println!("   [Steward Link] Port Error. Retrying...");
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
}

fn main() {
    let mut nami = Nami::new("/dev/ttyACM1");
    nami.read_weather(1.0); // Grounding at the Shelf
    nami.charge_captain();

    loop {
        nami.run_torque_filter();

        println!(
            "   [Nami] Status: HARMONIZED | Coherence: {:.8} | Treasury: {:.2} Berries",
            nami.navigation_coherence, nami.berry_count
        );
        thread::sleep(Duration::from_millis(100));
    }
}
