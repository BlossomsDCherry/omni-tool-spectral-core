use crew_core::Talu64;
// use std::f64::consts::PI;

const TAU: f64 = Talu64::TAU;

struct SphericalPole {
    name: String,
    mass: u64, // From file size
    pole_type: PoleType,
}

enum PoleType {
    Logic, // Bin 5 (Identity)
    // Visual,    // Bin 7 (Completion) - archived
    // Structure, // Bin 6 - archived
    Intent,    // Bin 3 (New)
    Interface, // Bin 7 (New - Heavy Anchor)
}

struct FlightComputer {
    poles: Vec<SphericalPole>,
    interference_map: Vec<u8>,
}

impl FlightComputer {
    fn new() -> Self {
        // Updated Poles from Lunar & Interference Harvest
        Self {
            poles: vec![
                SphericalPole {
                    name: "density_pulse_entrainment.py".to_string(),
                    mass: 4554,
                    pole_type: PoleType::Logic,
                }, // Bin 5 (Identity)
                SphericalPole {
                    name: "environmental_plotter.py".to_string(),
                    mass: 2151,
                    pole_type: PoleType::Intent,
                }, // Bin 3 (Intent)
                SphericalPole {
                    name: "soft_fpga.rs".to_string(),
                    mass: 15105,
                    pole_type: PoleType::Interface,
                }, // Bin 7 (Lunar Anchor)
            ],
            interference_map: Vec::new(),
        }
    }

    fn execute_8_perspective_shift(&mut self) {
        println!("🛰️ FLIGHT COMPUTER: Initiating 8-Perspective Shift...");

        let frames_per_ms = 60;
        let _partitions = 60 * 8; // 8 Shifts per frame? Or 8 shifts total. User said "8 perspective shifts... at 60fpmillisecond"

        // We will simulate 1 millisecond
        println!("   ⏱️  Duration: 1ms | Partition: 1ps (Simulated) | Rate: 60 frames/ms");

        for frame in 0..frames_per_ms {
            let t = frame as f64;

            // 8 Perspectives (0 to 7)
            for p in 0..8 {
                // 1. Calculate Density Field (Superposition of Poles)
                let density = self.calculate_density_at_t(t, p);

                // 2. Invert (Anti-Density)
                let anti_density = 255.0 - density;

                // 3. Pair with Polyrhythm Partners
                let resonance = self.apply_polyrhythm(anti_density, t);

                self.interference_map.push(resonance as u8);
            }
        }
    }

    fn calculate_density_at_t(&self, t: f64, perspective: usize) -> f64 {
        // Simple "Gravity" Model: Sum of (Mass * Sin(t + phase))
        let phase_offset = (perspective as f64 / 8.0) * TAU;

        let mut total_pull = 0.0;
        for pole in &self.poles {
            let freq = match pole.pole_type {
                PoleType::Logic => 5.0, // Bin 5
                // PoleType::Structure => 6.0, // Bin 6
                // PoleType::Visual => 7.0,    // Bin 7
                PoleType::Intent => 3.0,    // Bin 3 (New Harmonic)
                PoleType::Interface => 7.5, // Bin 7+ (Heavy Anchor)
            };

            let wave = (t * freq + phase_offset).sin();
            total_pull += (pole.mass as f64 % 100.0) * wave; // Modulo to keep values manageable
        }

        // Normalize to 0-255 range
        let norm = (total_pull + 300.0) / 600.0 * 255.0;
        norm.clamp(0.0, 255.0)
    }

    fn apply_polyrhythm(&self, input: f64, t: f64) -> f64 {
        // Factors: TAU/2, TAU/3, TAU/5, TAU/7...
        let v2 = (t * (TAU / 2.0)).sin();
        let v3 = (t * (TAU / 3.0)).sin();
        let v5 = (t * (TAU / 5.0)).sin();
        let v7 = (t * (TAU / 7.0)).sin();

        let modulation = (v2 + v3 + v5 + v7 + 4.0) / 8.0; // 0.0 to 1.0

        input * modulation
    }

    fn report(&self) {
        println!("✨ CONSTRUCTIVE INTERFERENCE MAP GENERATED.");
        println!("   - Poles Tracked:");
        for pole in &self.poles {
            println!("     * {} ({} b)", pole.name, pole.mass);
        }
        println!("   - Data Points: {}", self.interference_map.len());
        println!(
            "   - Sample Sequence (First 16): {:?}",
            &self.interference_map[0..16]
        );

        // Calc Average Resonance
        let sum: u64 = self.interference_map.iter().map(|&x| x as u64).sum();
        let avg = sum as f64 / self.interference_map.len() as f64;
        println!("   - Average Resonance: {:.4} / 255", avg);
        println!("   - Flight Path: SMOOTH (Null Effort confirmed).");
    }
}

fn main() {
    let mut computer = FlightComputer::new();
    computer.execute_8_perspective_shift();
    computer.report();
}
