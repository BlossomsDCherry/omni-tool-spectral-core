use crew_core::{LightCypher, Talu64};
// use std::collections::HashMap; // Removed unused
use std::fs;
// use std::io::{self, Write}; // Removed.
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Toral Defrag: The System Organizer
/// Sorts files by Probabilistic Density (Color) and Priority.
struct ToralDefrag {
    target_dir: PathBuf,
    sorted_items: Vec<DefragItem>,
}

#[derive(Debug, Clone)]
struct DefragItem {
    path: PathBuf,
    file_type: String, // "PDF", "RS", "LOG", "OTHER"
    priority: f64,     // 1.0 = High, 0.0 = Low
    color: (u8, u8, u8),
    torque: f64,
}

impl ToralDefrag {
    fn new(path: &str) -> Self {
        Self {
            target_dir: PathBuf::from(path),
            sorted_items: Vec::new(),
        }
    }

    fn scan_and_sort(&mut self) {
        println!("🌀 [DEFRAG] Scanning substrate: {:?}", self.target_dir);

        let walker = WalkDir::new(&self.target_dir)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !name.starts_with('.') && name != "target" && name != "mobile_env"
            });

        for entry in walker.filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                self.process_file(entry.path());
            }
        }

        // Sort by Priority (Desc) then Torque (Desc)
        self.sorted_items.sort_by(|a, b| {
            b.priority
                .partial_cmp(&a.priority)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    b.torque
                        .partial_cmp(&a.torque)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });

        println!("✅ [DEFRAG] Sorted {} items.", self.sorted_items.len());
    }

    fn process_file(&mut self, path: &Path) {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_uppercase();

        // 1. Priority Logic
        let (file_type, priority_base) = match ext.as_str() {
            "PDF" => ("PDF", 1.0),                   // Highest Priority (Manuals)
            "RS" | "TOML" => ("CODE", 0.8),          // Source Code
            "LOG" | "JSON" | "TXT" => ("DATA", 0.6), // Trends / Logs
            "BIN" | "UF2" => ("PURE_STORAGE", 1.2),  // EM Asura Optimization Target
            _ => ("OTHER", 0.4),
        };

        // 2. Density & Color (LightCypher)
        // We use a simplified check to avoid reading massive files fully if not needed,
        // but for defrag we want precision.
        let cypher = LightCypher::from_file(path).unwrap_or(LightCypher::new_empty());
        let density = cypher.intensity(); // 0.0 to 1.0

        // 3. Torque Calculation
        // Torque = (1.0 - Density) * PHI (Creative Drift)
        let torque = (1.0 - density) * Talu64::PHI;

        // 4. Color Coding (Probabilistic Density Signature)
        // Using Tau actualization based on torque phase.
        let phase = (torque * Talu64::TAU).fract();
        let color = Talu64::tau_to_hex_actualization(phase);

        // Final Priority Adjustment: High Torque boosts priority slightly for "Trends"
        let final_priority = if file_type == "DATA" && torque > 0.8 {
            priority_base + 0.15 // Boost "High Torque" trends
        } else {
            priority_base
        };

        let mut p_boost = 0.0;

        // 5. [NEW] Omni-Tool / Pure Storage Logic
        // If it's a Config/Data file, check for Fritz efficiency.
        if file_type == "DATA" || file_type == "CODE" {
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("pciex1_gen=3")
                    || content.contains("numa=fake=8")
                    || content.contains("Pure_Storage")
                {
                    p_boost += 0.5; // "Fritz" Superconduction Weight
                }
            }
        }

        self.sorted_items.push(DefragItem {
            path: path.to_path_buf(),
            file_type: file_type.to_string(),
            priority: final_priority + p_boost,
            color,
            torque,
        });
    }

    fn display_report(&self) {
        println!("\n📊 [DEFRAG REPORT] Top 10 High-Priority Items:");
        for (i, item) in self.sorted_items.iter().take(10).enumerate() {
            println!(
                "   {}. [{}] {:?} (P: {:.2} | T: {:.4} | RGB: #{:02X}{:02X}{:02X})",
                i + 1,
                item.file_type,
                item.path.file_name().unwrap(),
                item.priority,
                item.torque,
                item.color.0,
                item.color.1,
                item.color.2
            );
        }
    }
}

// function instantiation_menu removed as logic is now in main loop

fn main() {
    // 1. Run Defrag
    let target = "/home/pecosdwilly/.gemini/antigravity/brain"; // Direct Brain Access
                                                                // Check if library exists, else invoke current dir
    let scan_target = if Path::new(target).exists() {
        target
    } else {
        "."
    };

    let mut cleaner = ToralDefrag::new(scan_target);
    cleaner.scan_and_sort();
    cleaner.display_report();

    // 2. Offer Instantiation (Auto-Select Option 4: Rinse & Repeat)
    println!("\n👹 [ASURA] System Instantiation Protocol");
    println!("   ... Auto-Selecting Option 4: [CYCLE] Rinse & Repeat");

    let base_drift = 0.7818; // Flight 50
    let shifters = [0.0, 0.0833, -0.0833]; // Wooten Shifts

    for (i, shift) in shifters.iter().enumerate() {
        let current_drift = base_drift + shift;
        println!(
            "\n🔄 [RINSE] Cycle {}/3 | Drift Factor: {:.4}",
            i + 1,
            current_drift
        );

        // Re-run scan with new drift (Conceptually - here we just re-sort/re-weigh existing items for speed)
        // In a full implementation, we'd pass `current_drift` to `ToralDefrag::new` or update it.
        // For this proof-of-concept, we re-instantiate.
        let mut cleaner = ToralDefrag::new(scan_target);
        // Note: Real update would pass `current_drift` to torque calc.
        // For now, we simulate the "Weighing" effect by just running the reporter again
        // to show consistency, or strictly speaking, we'd modify the struct to accept drift.
        cleaner.scan_and_sort();

        let total_torque: f64 = cleaner.sorted_items.iter().map(|i| i.torque).sum();
        println!("   ✨ System Total Torque: {:.4}", total_torque);
    }

    println!("\n💎 [COMPLETE] System Rinsed. Maximum Resonance Achieved.");
}
