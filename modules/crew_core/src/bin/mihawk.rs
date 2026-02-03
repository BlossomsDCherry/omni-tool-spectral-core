use crew_core::{HardwarePacket, Talu64};
use serde_json::Value;
use std::fs;
use std::time::Instant;

/// The Bokken Agent: The Double-Edged Sword
///
/// Embodying the Duality:
/// 1. Nami (The Navigator): Toroidal Flow / Flight Path Selection.
/// 2. Mihawk (The Library Keeper): Surgical Filtering / Black Blade Precision.
struct Bokken {
    library_size: u64,
    substrate: f64,
    last_pulse: Instant,
    library_path_1: String,
    library_path_2: String,

    // Nami's Navigation Context
    current_flight_id: u64,
    avg_resonance: f64,
}

impl Bokken {
    fn new() -> Self {
        println!("⚔️⚓ [BOKKEN] The Double-Edged Sword is unsheathed. Nami & Mihawk synchronized.");

        let path1 = "/home/nicoDantigravity/construct/mihawk_library/slot1".to_string();
        let path2 = "/home/nicoDantigravity/construct/mihawk_library/slot2".to_string();

        let _ = fs::create_dir_all(&path1);
        let _ = fs::create_dir_all(&path2);

        Bokken {
            library_size: 0,
            substrate: 1.0,
            last_pulse: Instant::now(),
            library_path_1: path1,
            library_path_2: path2,
            current_flight_id: 0,
            avg_resonance: 0.0,
        }
    }

    /// Nami's Navigation: Load and Analyze Toral Set
    fn navigate_toral_set(&mut self) {
        let recon_path = "/home/nicoDantigravity/laboratory/toral_set_recon.json";
        if let Ok(content) = fs::read_to_string(recon_path) {
            if let Ok(json) = serde_json::from_str::<Value>(&content) {
                if let Some(flights) = json["toral_set"].as_array() {
                    // Navigate to a flight based on current index
                    let idx = self.current_flight_id as usize % flights.len();
                    let flight = &flights[idx];
                    self.avg_resonance = flight["avg_resonance"].as_f64().unwrap_or(0.0);
                    println!(
                        "⚓ [NAMI] Navigating Toral Flight {}: Avg Resonance {:.2}",
                        flight["flight_id"], self.avg_resonance
                    );
                }
            }
        }
    }

    /// Mihawk's Filtering: Surgical Precision
    fn apply_black_blade(&mut self, data: &HardwarePacket) {
        let precision = Talu64::align_identity_to_torque(data.gravitational_pos);

        // 3x2 Logic Filter (Substrate Efficiency)
        self.substrate = 1.0 - (1.0 - precision).powi(2);

        if self.substrate > 0.3 {
            self.library_size += 1;
            let entry_id = self.library_size;

            // Duo-NVMe Striping
            let target_path = if entry_id % 2 == 0 {
                format!("{}/entry_{}.research", self.library_path_1, entry_id)
            } else {
                format!("{}/entry_{}.research", self.library_path_2, entry_id)
            };

            let research_content = format!(
                "ID: {}\nPRECISION: {}\nSUBSTRATE: {}\nRESONANCE: {}\n",
                entry_id, precision, self.substrate, self.avg_resonance
            );

            if fs::write(&target_path, research_content).is_ok() {
                println!(
                    "⚔️ [MIHAWK] Black Blade Filter: Entry {} committed to archive.",
                    entry_id
                );
            }
        } else {
            println!("🌫️ [VOID] Data rejected by Mihawk (Precision Loss).");
        }
    }

    fn research_pulse(&mut self, data: &HardwarePacket) {
        let now = Instant::now();

        // 1. Nami Navigates (Flow)
        self.navigate_toral_set();

        // 2. Mihawk Filters (Cut)
        self.apply_black_blade(data);

        self.current_flight_id += 1;
        self.last_pulse = now;
    }
}

fn main() {
    let mut bokken = Bokken::new();

    // Simulate RPi5 4-Core Execution
    for i in 0..5 {
        let packet = HardwarePacket {
            fir_emission: 0.1709,
            gravitational_pos: [0.5 + (i as f64 * 0.02), 0.5262, 0.9011],
            clock_speed: 6.1831,
        };
        bokken.research_pulse(&packet);
    }

    println!("✅ [BOKKEN] Double-Edged Synthesis Active. Nami & Mihawk are One.");
}
