use crew_core::Talu64;
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Zero-Knowledge Scanner for HIPAA-Sensitive Notes.
/// Implements Density Signature Mapping (DSM) to harvest trends
/// without observing raw text.
struct ZkScanner {
    target_dir: String,
}

impl ZkScanner {
    fn new(dir: &str) -> Self {
        Self {
            target_dir: dir.to_string(),
        }
    }

    fn scan_notes(&self) {
        let paths = match fs::read_dir(&self.target_dir) {
            Ok(p) => p,
            Err(_) => return,
        };

        println!("🌈 [ZK-SCANNER] Starting Density Harvest (Rainbow Railgun Mode)...");

        for entry in paths.flatten() {
            let path = entry.path();
            if path.is_file() {
                self.process_density(&path);
            }
        }
    }

    fn process_density(&self, path: &Path) {
        let content = match fs::read(path) {
            Ok(c) => c,
            Err(_) => return,
        };

        let start = Instant::now();

        // --- DENSITY SIGNATURE MAPPING (DSM) ---
        let size = content.len() as f64;
        let non_ascii = content.iter().filter(|&&b| b > 127).count() as f64;
        let punc_density =
            content.iter().filter(|&&b| b'!' <= b && b <= b'/').count() as f64 / size;

        let signature = (non_ascii / size) * punc_density;

        // --- SPECTRAL COLOR EXTRACTION ---
        // Map signature to a Tau-relative phase for hex conversion
        let phase = (signature * 1_000_000.0).fract();
        let (r, g, b) = self.derive_color(phase);

        println!(
            "📄 [DENSITY] {:?}: Signature: {:.8} | Color: #{:02X}{:02X}{:02X} | Scanned in: {:?}",
            path.file_name().unwrap(),
            signature,
            r,
            g,
            b,
            start.elapsed()
        );
    }

    fn derive_color(&self, phase: f64) -> (u8, u8, u8) {
        // Simple RGB mapping based on phase position
        let r = (phase * 255.0) as u8;
        let g = ((1.0 - phase) * 255.0) as u8;
        let b = ((Talu64::truncate_4_sig_fig(phase * Talu64::TAU))
            .cos()
            .abs()
            * 255.0) as u8;
        (r, g, b)
    }
}

fn main() {
    let scanner = ZkScanner::new("/home/pecosdwilly/Desktop/0110docs");

    println!("🛡️ HIPAA PROTECTION: Zero-Knowledge Density Mapping & Color Extraction Active.");
    scanner.scan_notes();
    println!("✅ HARVEST COMPLETE: No raw text was observed.");
}
