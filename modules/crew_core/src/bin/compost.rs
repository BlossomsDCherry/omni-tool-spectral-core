use crew_core::LightCypher;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("🌪️ COMPOST SPINNER: Initializing High Fidelity Harvest...");

    let args: Vec<String> = env::args().collect();
    let default_source = "/home/nicoDantigravity/laboratory".to_string();
    let source_dir = if args.len() > 1 {
        &args[1]
    } else {
        &default_source
    };

    let compost_base = "/home/nicoDantigravity/laboratory/compost";

    // 7 Bins
    let bins: Vec<PathBuf> = (1..=7)
        .map(|i| PathBuf::from(format!("{}/Bin_{}", compost_base, i)))
        .collect();

    let mut total_mass = 0u64;
    let mut moved_count = 0;

    println!("   📂 Source: {}", source_dir);
    println!("   ♻️  Target: {} (7 Bins)", compost_base);

    // BLAZE THE WORKSPACE
    let source_path = Path::new(source_dir);
    if source_path.is_file() {
        process_file(source_path, &bins, &mut total_mass, &mut moved_count);
    } else {
        visit_dirs(source_path, &bins, &mut total_mass, &mut moved_count);
    }

    println!("✨ HARVEST COMPLETE.");
    println!("   - Total Mass Sorted: {} bytes", total_mass);
    println!("   - Files Entrained: {}", moved_count);
    println!("   - Resonance: HIGH-FIDELITY.");
}

fn visit_dirs(dir: &Path, bins: &[PathBuf], total_mass: &mut u64, moved_count: &mut u64) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    // Avoid recursion into the compost bins themselves!
                    if !path.to_string_lossy().contains("compost") {
                        visit_dirs(&path, bins, total_mass, moved_count);
                    }
                } else {
                    process_file(&path, bins, total_mass, moved_count);
                }
            }
        }
    }
}

fn process_file(path: &Path, bins: &[PathBuf], total_mass: &mut u64, moved_count: &mut u64) {
    if let Ok(metadata) = fs::metadata(path) {
        let size = metadata.len();
        if size == 0 {
            return;
        }

        // --- SPECTRAL INTENSITY SORTING ---
        // Instead of size % 7, we use the Light Cypher intensity.
        let cypher = LightCypher::from_file(path).unwrap_or_else(LightCypher::new_empty);
        let intensity = cypher.intensity();

        // Map 0.0-1.0 to 0-6
        let bin_index = (intensity * 6.99) as usize;
        let target_bin = &bins[bin_index];

        let file_name = path.file_name().unwrap();
        let target_path = target_bin.join(file_name);

        // Copy
        if let Ok(_) = fs::copy(path, &target_path) {
            println!(
                "   -> [Bin {}] Intensity: {:.4} | Entrained: {:?} ({} b)",
                bin_index + 1,
                intensity,
                file_name,
                size
            );
            *total_mass += size;
            *moved_count += 1;
        }
    }
}
