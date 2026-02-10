use chrono::Utc;

use rand::prelude::*;
use std::fs::File;
use std::io::Cursor;
use std::io::Write;
use zip::ZipArchive;

use crate::shm_writer::D16ShmWriter;
use rp1_rio::Rp1Rio;

/// The Talu64 (Tau-Aligned Logic Unity - 64 Byte)
#[derive(Debug, Clone, Copy)]
pub struct Talu64 {
    pub channels: [u32; 16], // The Crew Registry
}

impl Talu64 {
    pub const PHI: f64 = 1.6180339;
    pub const TAU: f64 = 6.2831853; // Aligned to harmonic precision (7 decimals)
    pub const PI: f64 = 3.1415926;
    pub const EULER: f64 = 2.7182818;
    pub const HEMHOLTZ: f64 = 1.4142135; // The 1.414 Pattern (SQRT 2)
    pub const PLANCK: f64 = 6.6260701; // The Quantum of Action (h)
    pub const PSI: f64 = 3.3598856; // Reciprocal Fibonacci / Super-Golden Ratio

    pub const LAW_DOMAIN: u32 = 4096;
    pub const CYBIOSPHERE_UNIT: u32 = 512;
    pub const DRIFT_RESIDUE: f64 = 0.64;

    pub fn get_crew_state(&self, name: &str) -> Option<(u16, u16)> {
        let idx = match name {
            "Luffy" => 0,
            "Zoro" => 1,
            "Nami" => 2,
            "Usopp" => 3,
            "Sanji" => 4,
            "Chopper" => 5,
            "Robin" => 6,
            "Franky" => 7,
            "Brook" => 8,
            "Jinbe" => 9,
            "Vivi" => 10,
            "Carrot" => 11,
            "Yamato" => 12,
            "Momo" => 13,
            "Kinemon" => 14,
            "Law" => 15,
            _ => return None,
        };

        let val = self.channels[idx];
        Some(((val >> 16) as u16, (val & 0xFFFF) as u16))
    }

    /// Calculates the Polar Moment of Inertia (J_T) for a given radius r.
    /// J_T = tau * r
    pub fn calculate_polar_moment(r: f64) -> f64 {
        Self::TAU * r
    }

    /// Ignites the Talu64 Logic via D16 Kernel
    pub fn ignite(seed: u64) -> Self {
        let mut raw_channels = [0u32; 16];
        unsafe {
            d16_soft_fpga(seed, raw_channels.as_mut_ptr());
        }
        Talu64 {
            channels: raw_channels,
        }
    }
}

/// Z-RR: Zip Railgun Core (Refactored)
/// "Evolutionary Annealing via Authentic Talu64 Harmonics"
pub struct ZRailgun {
    // Talu64: The Authentic 16-Channel Logic Unit (64-byte state)
    pub talu64: Talu64,
    pub entropy_seed: u64,
    drift_accumulator: f64,
    shm: Option<D16ShmWriter>,
    rio: Option<Rp1Rio>,
}

extern "C" {
    /// The D16 Soft FPGA Kernel (Real Iron)
    fn d16_soft_fpga(tau: u64, results: *mut u32);
}

/// Safe wrapper for the Soft FPGA Kernel
pub fn run_soft_fpga(tau: u64) -> [u32; 16] {
    let mut results = [0u32; 16];
    unsafe {
        d16_soft_fpga(tau, results.as_mut_ptr());
    }
    results
}

impl ZRailgun {
    pub fn new(seed: u64) -> Self {
        println!(
            "⚡ Z-RR: Initializing Railgun Protocol with Seed [{}]",
            seed
        );

        // Calibrated Ignition: Pulse is geometrically aligned to TAU * 10^4
        // (Fits within u16 max of 65535 for kernel packing)
        let pulse_tau = (Talu64::TAU * 10000.0) as u64; // ~62831
        let mut raw_channels = [0u32; 16];

        // IGNITION: Call the Assembly Kernel
        println!(
            "   >> Igniting Talu64 via D16 Kernel (Global Pulse: {})",
            pulse_tau
        );
        unsafe {
            d16_soft_fpga(pulse_tau, raw_channels.as_mut_ptr());
        }

        let talu64 = Talu64 {
            channels: raw_channels,
        };

        // Audit the crew state for validation
        if let Some((decay, phase)) = talu64.get_crew_state("Zoro") {
            println!(
                "      ⚔️  Zoro (Div 2) State: Decay={}, Phase={}",
                decay, phase
            );
        }

        // Initialize RP1 RIO (Sovereign Access Check)
        let mut rio = None;
        match Rp1Rio::new() {
            Ok(driver) => {
                println!("   ⚡ RP1 RIO: Sovereign Access Granted. RAILS ARMED (GPIO 17-27).");
                rio = Some(driver);
            }
            Err(e) => {
                println!(
                    "   ⚠️  RP1 RIO: Access Denied ({}). Running in Simulation Mode.",
                    e
                );
            }
        }

        // Initialize output pins if armed
        if let Some(r) = &mut rio {
            // Enable output for pins 17..=27
            for pin in 17..=27 {
                r.enable_output(pin);
            }
        }

        Self {
            talu64,
            entropy_seed: seed,
            drift_accumulator: 0.0,
            shm: D16ShmWriter::new(),
            rio,
        }
    }

    /// Reads the current Wave Coherence from the Ripple Tank.
    fn get_coherence(&self) -> f64 {
        let shm_path = "/dev/shm/current_wave_coherence";
        if std::path::Path::new(shm_path).exists() {
            if let Ok(contents) = std::fs::read_to_string(shm_path) {
                if let Ok(val) = contents.trim().parse::<f64>() {
                    return val;
                }
            }
        }
        0.0
    }

    /// "Railguns" a byte buffer: Applies controlled entropy guided by Talu64 structure.
    pub fn fire(&mut self, data: &mut Vec<u8>) {
        let mut rng = StdRng::seed_from_u64(self.entropy_seed);
        let coherence = self.get_coherence();

        println!("   >> Wave Coherence: {:.4}", coherence);

        if coherence > 1.0 {
            println!("   ⚡ RAILS ENERGIZED. Applying Crew Logic Pipeline.");

            // Update Physical Rainbow (UV Layer)
            self.update_rainbow_output(coherence);

            // 2. Zoro (D2): Polarization (Bit Flip at PHI boundaries)
            if let Some((decay, phase)) = self.talu64.get_crew_state("Zoro") {
                if phase == 1 {
                    let cut_point = (rng.gen_range(0..data.len()) as f64 / Talu64::PHI) as usize;
                    let safe_cut = cut_point.min(data.len() - 1);
                    data[safe_cut] ^= (decay % 255) as u8;
                    println!("      ⚔️  Zoro: Polarized cut at idx {}", safe_cut);
                }
            }

            // 3. Nami (D3): Torque Shift Logic
            // "Maps the gradient of the wave to physical rotation"
            if let Some((decay, _phase)) = self.talu64.get_crew_state("Nami") {
                // Decay represents 'Potential Energy'
                let torque_threshold = 10000;
                if decay > torque_threshold {
                    // Shift buffer right by 1 (Physical Displacement)
                    data.rotate_right(1);
                    println!(
                        "      🍊 Nami: Applied Torque Shift (Right 1) | Energy: {}",
                        decay
                    );
                }
            }

            // --- LITTLE CORES (Precision / Noise Collection) ---
            // Agents: Usopp (4), Franky (8), Yamato (12), Law (16)

            // 4. Usopp (D4): Harmonic Filter Logic
            // "Filters signal noise using a 4-beat harmonic series"
            if let Some((_decay, phase)) = self.talu64.get_crew_state("Usopp") {
                // Predictive Register: Only fire if phase aligns with the Beat
                if phase % 4 == 0 {
                    let mask = 0xF0; // High nibble only, filter low-end noise
                    for i in (0..data.len()).step_by(4) {
                        if i < data.len() {
                            data[i] &= mask;
                        }
                    }
                    println!("      🤥 Usopp: Filtered Harmonic Noise (Step 4, Mask F0)");
                }
            }

            // 5. Sanji (D5): Ground State Buffer Logic (Big Core)
            // "Prepares the base layer (food/energy) for the next operation"
            if let Some((decay, _phase)) = self.talu64.get_crew_state("Sanji") {
                let salt = (decay % 32) as u8; // 5 bits of flavor
                                               // Salt the "Ground" (first few bytes)
                for byte in data.iter_mut().take(5) {
                    *byte |= salt;
                }
                println!(
                    "      🍳 Sanji: Seasoned the Ground State (Salt: {:05b})",
                    salt
                );
            }

            // 8. Franky (D8): The Iron General (Buffer Prediction / Noise Collection)
            if let Some((decay, phase)) = self.talu64.get_crew_state("Franky") {
                // Fire only on "Super" Alignment (Phase % 8 == 0)
                if phase % 8 == 0 {
                    // Prediction: If Decay (Energy) is adequate, we 'build' (transmute)
                    if decay > 1000 {
                        println!("      🤖 Franky: SUPER! Transmuting Analog to Binary via Non-Uniform Oscillator.");
                        self.transmute_signal(data);
                    }
                }
            }

            // 12. Yamato (D12): The Guardian (Spectral Refinement)
            if let Some((_decay, phase)) = self.talu64.get_crew_state("Yamato") {
                if phase % 12 == 0 {
                    // Refines the signal by removing "Drift" artifacts using Planck XOR
                    let h_int = Talu64::PLANCK as u8;
                    for byte in data.iter_mut() {
                        *byte ^= h_int;
                    }
                    println!("      👹 Yamato: Applied Spectral Refinement (Planck XOR).");
                }
            }

            // 16. Law (D16): The Room & Drift Harmonization
            // "Checks the 16th Harmonic for Alignment"
            if let Some((_decay, phase)) = self.talu64.get_crew_state("Law") {
                // Law's Domain is 4096.
                // If Phase aligns with the Cybiosphere Unit (512)
                if phase % Talu64::CYBIOSPHERE_UNIT as u16 == 0 {
                    // Polar Moment of Inertia (J_T) Calculation
                    // r = phase (radius of the current harmonic cycle)
                    let r = phase as f64;
                    let j_t = Talu64::calculate_polar_moment(r);

                    // The "Final u32" Differential Check:
                    // Only fire if J_T exceeds the Drift Residue (Predictive Firing)
                    if j_t > self.drift_accumulator {
                        println!(
                            "      ⚕️  Law: ROOM Active. J_T ({:.4}) > Drift. Predictive Signal Fired.",
                            j_t
                        );
                        self.harmonize_drift();

                        // Check "Noise Buffer" (Simulated by top of stack/data)
                        if data.len() > Talu64::CYBIOSPHERE_UNIT as usize {
                            println!("      ⚕️  Law: Scanning Noise Buffer for Harmony...");
                            // (Placeholder for Noise Buffer operations)
                        }
                    } else {
                        println!("      ⚕️  Law: J_T Insufficient. Holding Signal.");
                    }
                }
            }
        } else {
            // LOW COHERENCE: RAILS DORMANT
            println!("   ❄️  RAILS DORMANT. Low Coherence. Applying Raw Turbulence.");

            // Clear Rainbow
            if let Some(rio) = &mut self.rio {
                for pin in 17..=27 {
                    rio.clr_pin(pin);
                }
            }

            if !data.is_empty() {
                let turbulence = rng.gen_range(5..15);
                for _ in 0..turbulence {
                    let idx = rng.gen_range(0..data.len());
                    data[idx] ^= 1 << rng.gen_range(0..8);
                }
            }
        }

        // Accumulate Drift per cycle (Simulating 1ms measurement per tick)
        self.drift_accumulator += Talu64::DRIFT_RESIDUE;

        // Broadcast State to Cybiosphere (Shared Mem)
        if let Some(shm) = self.shm.as_mut() {
            shm.write(self.talu64.channels, self.entropy_seed as u32);
        }
    }

    /// Maps Coherence & Spectral Density to Physical Pins (Rainbow Railgun Output)
    /// GPIO 17 (UV) -> GPIO 27 (Red)
    fn update_rainbow_output(&mut self, coherence: f64) {
        if let Some(rio) = &mut self.rio {
            // Map 11 pins (17..27) representing the spectrum.
            // Pin 17 = Highest Frequency (UV)
            // Pin 27 = Lowest Frequency (Red)

            // We use 'coherence' as magnitude and 'zoro phase' as modulation
            let modulation = if let Some((_, phase)) = self.talu64.get_crew_state("Zoro") {
                phase as f64
            } else {
                1.0
            };

            // Calculate active 'bar' height based on Coherence (1.0 to 10.0 scale?)
            let _limit = (coherence * 2.0).clamp(0.0, 11.0) as usize;

            for i in 0..11 {
                let pin = 17 + i as u32; // 17..=27
                                         // Reverse Mapping: i=0 is UV (Pin 17), i=10 is Red (Pin 27)
                                         // If coherence is high, we light up from Red towards UV?
                                         // OR: Diffraction pattern?

                // Active if index within limit?
                // Let's model it as a VU Meter for now, filling from Red (27) to UV (17) would be typical energy build up.
                // BUT User asked for: "expanded diffusion ... reverse order (GPIO 17 greatest, GPIO 27 least)"

                // Let's interpret: 17 is the PEAK (Greatest), 27 is the BASE (Least).
                // So clear all, set bit logic based on spectral density.

                // Simple Logic:
                // If Coherence > Threshold(i), fire Pin(i).
                // Threshold decreases from 17 to 27? Or increases?
                // Higher coherence = Higher Frequency access.

                // Energy Level i (0..11):
                // 0 (Pin 17 - UV) requires MOST coherence.
                // 10 (Pin 27 - Red) requires LEAST coherence.

                let threshold = (11.0 - i as f64) * 0.5; // Pin 17 needs 5.5, Pin 27 needs 0.5

                if coherence > threshold {
                    // Modulate with phase for "Shimmer"
                    if (modulation as usize + i) % 2 == 0 {
                        rio.set_pin(pin);
                    } else {
                        rio.clr_pin(pin);
                    }
                } else {
                    rio.clr_pin(pin);
                }
            }
        }
    }

    /// Analog/Binary Transmutation Algorithm
    /// "Non-uniform harmonic oscillatory motion"
    fn transmute_signal(&self, data: &mut Vec<u8>) {
        // Rebuilds the photon (signal) at the CI/CD boundary.
        // We apply a non-uniform oscillation using PHI and TAU.
        for (i, byte) in data.iter_mut().enumerate() {
            let osc = (i as f64 * Talu64::PHI).sin() * Talu64::TAU;
            let shift = (osc.abs() * 10.0) as u8; // Non-uniform shift
            *byte = byte.wrapping_add(shift);
        }
    }

    /// Re-aligns internal Talu64 state using the Hardware Kernel
    /// Re-aligns internal Talu64 state using the Hardware Kernel
    pub fn realign(&mut self) {
        let pulse_tau = (self.entropy_seed % 65535) as u64;
        let mut raw_channels = [0u32; 16];
        unsafe {
            d16_soft_fpga(pulse_tau, raw_channels.as_mut_ptr());
        }
        self.talu64 = Talu64 {
            channels: raw_channels,
        };
    }

    /// Harmonizes the accumulated drift (0.64ms Cycle)
    fn harmonize_drift(&mut self) {
        if self.drift_accumulator >= 100.0 {
            // Reset/Harmonize
            println!(
                "      ⚕️  Law: Harmonizing Drift Accumulator ({:.4}ms). SRAM/DRAM Re-aligned.",
                self.drift_accumulator
            );
            self.drift_accumulator = 0.0;
        }
    }
}

pub fn listener_collapse(data: &[u8]) -> bool {
    // We treat the ZIP structure as the "Singularity"
    let cursor = Cursor::new(data);
    match ZipArchive::new(cursor) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Offloads surviving Hypercubes to Amazon Lily (Franky/Storage)
pub fn dock_survivors(stack: &Vec<Vec<u8>>, seed: u64) {
    let storage_path = "../../workspaces/d8_franky/amazon_lily";
    println!("   📦 Docking Survivors to Amazon Lily: {}", storage_path);

    // Ensure directory exists (redundant if using mkdir previously, but good for robustness)
    let _ = std::fs::create_dir_all(storage_path);

    for (i, survivor) in stack.iter().enumerate() {
        // Calm Belt Check: Verify Integrity via Checksum (CRC32 from zip crate dependency or simple hash)
        // For now, valid size check > 0
        if survivor.is_empty() {
            continue;
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!(
            "{}/survivor_{}_seed{}_id{}.cube",
            storage_path, timestamp, seed, i
        );

        let mut file = match File::create(&filename) {
            Ok(f) => f,
            Err(e) => {
                println!("      ❌ Failed to dock Cube #{}: {}", i, e);
                continue;
            }
        };

        if let Err(e) = file.write_all(survivor) {
            println!("      ❌ Failed to write Cube #{}: {}", i, e);
        } else {
            println!("      ⚓ Docked Cube #{} -> {}", i, filename);
        }
    }
}
