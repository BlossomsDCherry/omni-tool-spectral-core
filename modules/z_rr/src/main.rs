use ndarray::prelude::*;
use ndarray_npy::read_npy;
use rand::prelude::*;
use std::env;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use zip::ZipArchive;

/// Z-RR: Zip Railgun Core
/// "Evolutionary Annealing via Talu64 Constraints"

struct ZRailgun {
    // Talu64: AlphaTensor Factorizations (Modulo 2)
    // We use these schemas to "fold" data streams structurally
    talu64_matrix: Array2<i64>, // Placeholder for F2 loaded data
    entropy_seed: u64,
}

impl ZRailgun {
    fn new(seed: u64) -> Self {
        println!(
            "⚡ Z-RR: Initializing Railgun Protocol with Seed [{}]",
            seed
        );

        // Load Talu64 (AlphaTensor Modulo 2)
        // Path: /home/pecosdwilly/Downloads/alphatensor-main/algorithms/factorizations_f2.npz
        // Key: "4,4,4" (Standard 4x4 matmul factorization)
        let npz_path =
            "/home/pecosdwilly/Downloads/alphatensor-main/algorithms/factorizations_f2.npz";
        let talu64_matrix = match File::open(npz_path) {
            Ok(file) => {
                println!("   >> Loading Talu64 from: {}", npz_path);
                match ndarray_npy::NpzReader::new(file) {
                    Ok(mut npz) => {
                        // The shape is (3, 16, 47).
                        match npz.by_name::<ndarray::OwnedRepr<i64>, ndarray::Ix3>("4,4,4") {
                            Ok(arr) => {
                                println!(
                                    "   ✅ Loaded AlphaTensor '4,4,4'. Shape: {:?}",
                                    arr.shape()
                                );
                                // Flatten or slice for 2D usage in Z-RR (mock projection to 2D for now)
                                arr.index_axis(Axis(0), 0).to_owned()
                            }
                            Err(e) => {
                                println!("   ⚠️  Key '4,4,4' not found or shape mismatch: {}. Fallback to Identity.", e);
                                Array2::eye(64)
                            }
                        }
                    }
                    Err(e) => {
                        println!("   ⚠️  Invalid .npz archive: {}. Fallback to Identity.", e);
                        Array2::eye(64)
                    }
                }
            }
            Err(e) => {
                println!("   ⚠️  Talu64 file not found: {}. Fallback to Identity.", e);
                Array2::eye(64)
            }
        };

        Self {
            talu64_matrix,
            entropy_seed: seed,
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
        0.0 // Default to zero if reading fails
    }

    /// "Railguns" a byte buffer: Applies controlled entropy guided by Talu64 structure.
    fn fire(&mut self, data: &mut Vec<u8>) {
        let mut rng = StdRng::seed_from_u64(self.entropy_seed);
        let coherence = self.get_coherence();

        println!("   >> Wave Coherence: {:.4}", coherence);

        if coherence > 1.0 {
            // HIGH COHERENCE: RAILS ENERGIZED
            // Talu64 Structural Fold is ENABLED.
            // We use the matrix to "guide" the mutations (simulated here by lower turbulence but higher impact)
            println!("   ⚡ RAILS ENERGIZED. Applying Talu64 Fold.");

            // In a real implementation, this would be: data = talu64_matrix . data
            // Here, we simulate "Precision" by flipping fewer bits but specific ones.
            let turbulence = 1;
            for _ in 0..turbulence {
                let idx = rng.gen_range(0..data.len());
                data[idx] ^= 1 << rng.gen_range(0..8);
            }
        } else {
            // LOW COHERENCE: RAILS DORMANT
            // Raw Entropy / Chaos
            println!("   ❄️  RAILS DORMANT. Low Coherence. Applying Raw Turbulence.");
            let turbulence = rng.gen_range(5..15); // High chaos
            for _ in 0..turbulence {
                let idx = rng.gen_range(0..data.len());
                data[idx] ^= 1 << rng.gen_range(0..8);
            }
        }
    }
}

/// The Listener: Collapses the wave function by verifying structural integrity.
/// "The frequency of the incoming wave (Advertiser) is equal and opposite to the Listening circuit."
fn listener_collapse(data: &[u8]) -> bool {
    let cursor = Cursor::new(data);
    match ZipArchive::new(cursor) {
        Ok(_) => true,   // Singularity Achieved (Structure holds)
        Err(_) => false, // Chaos / Drift
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: zrr_core <target_zip>");
        return;
    }

    let target_path = &args[1];
    println!("🎯 Target: {}", target_path);

    // LIFO Stack for Zero-Wait Maneuvers
    let mut stack: Vec<Vec<u8>> = Vec::new();

    // Load Baseline
    let mut baseline = Vec::new();
    match File::open(target_path) {
        Ok(mut f) => {
            f.read_to_end(&mut baseline).unwrap();
        }
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };

    println!("📥 Baseline Loaded: {} bytes", baseline.len());
    stack.push(baseline.clone());

    let mut advertiser = ZRailgun::new(1337); // The Field Generator

    // Annealing Loop (Simulated Short Burst)
    for i in 0..5 {
        println!("\n🔥 Railgun Cycle #{}", i);

        // LIFO: Peek at top
        let mut hypercube_state = stack.last().unwrap().clone();

        // Advertiser fires (injects entropy/torque)
        advertiser.fire(&mut hypercube_state);
        advertiser.entropy_seed += 1;

        // Listener attempts to collapse the wave
        if listener_collapse(&hypercube_state) {
            println!("✅ Singularity Achieved: Hypercube Collapsed to Stable 5D Signature.");
            // If valid, we push it. In real logic, we'd compare compression ratios or hash complexity.
            // For now, we assume validity = survival.

            // "LIFO Zero-Wait": If it's better, it becomes the new top instantly.
            stack.push(hypercube_state);
        } else {
            println!("💥 Collapse Failed. Structural Integrity Lost.");
        }
    }

    println!("\n🏁 Z-RR Mission Complete. Survivors: {}", stack.len());
    // In real app, write the top of stack to disk.
}
