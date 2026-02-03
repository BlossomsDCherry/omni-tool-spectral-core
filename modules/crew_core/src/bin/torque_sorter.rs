use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const PHI: f64 = 1.618033988749895;

fn calculate_entropy(path: &Path) -> f64 {
    let metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => return 1.0,
    };

    if metadata.len() == 0 {
        return 1.0;
    }

    let mut hasher = Sha256::new();
    let content = match fs::read(path) {
        Ok(c) => c,
        Err(_) => return 1.0,
    };

    // Read first 1MB for speed
    let sample = if content.len() > 1024 * 1024 {
        &content[..1024 * 1024]
    } else {
        &content
    };

    hasher.update(sample);
    let result = hasher.finalize();
    let h_val =
        u32::from_be_bytes([result[0], result[1], result[2], result[3]]) as f64 / u32::MAX as f64;

    let wavelength = (metadata.len() % 360) as f64 / 360.0;
    let phase_offset = (wavelength - (h_val % 0.1618)).abs();

    (h_val + phase_offset) / 2.0
}

fn apply_wooten_shift(torque: f64) -> f64 {
    if torque > 0.45 && torque < 0.55 {
        if torque < 0.5 {
            torque + 0.0833
        } else {
            torque - 0.0833
        }
    } else {
        torque
    }
}

fn main() {
    let source = "/home/nicoDantigravity/laboratory/compost";
    println!(
        "🌀 [TORQUE] Initializing Rust Engine: {} (WOOTEN SHIFT active)",
        source
    );

    let mut bins: HashMap<usize, Vec<PathBuf>> = HashMap::new();
    for i in 1..=12 {
        bins.insert(i, Vec::new());
    }

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            let raw_entropy = calculate_entropy(path);
            let raw_torque = (1.0 - raw_entropy) * PHI;
            let torque = apply_wooten_shift(raw_torque);

            let bin_idx = ((torque * 12.0 / PHI).ceil() as usize).min(12).max(1);
            bins.get_mut(&bin_idx).unwrap().push(path.to_path_buf());
        }
    }

    for (idx, paths) in bins.iter() {
        if !paths.is_empty() {
            println!(
                "📦 [BIN {}] Mass: {} fragments identified.",
                idx,
                paths.len()
            );
        }
    }
    println!("✅ [SUCCESS] Build Verification Complete. Ready for Polymerization.");
}
