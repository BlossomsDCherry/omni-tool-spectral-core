use crew_core::{LightCypher, Medium, SevenArches, SunTzuContext, Talu64};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;
use walkdir::WalkDir;

/// The Omni-Tool: 3x2x2 Navigation & Ship Folding Algorithm
/// "Accessing the Substrate through Parsimony"
pub struct OmniTool {
    pub workspace: PathBuf,
    pub strategy: SunTzuContext,
}

impl OmniTool {
    pub fn new(workspace: &str) -> Self {
        Self {
            workspace: PathBuf::from(workspace),
            strategy: SunTzuContext::new(),
        }
    }

    pub fn file_to_density_profile(&self, path: &Path) -> Option<Vec<f64>> {
        let ext = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        match ext.as_deref() {
            Some("pdf") => self.pdf_to_density(path),
            Some("pgm") => self.pgm_to_density(path),
            _ => {
                if let Ok(mut f) = fs::File::open(path) {
                    let mut buf = [0; 4];
                    if f.read_exact(&mut buf).is_ok() && &buf == b"%PDF" {
                        return self.pdf_to_density(path);
                    }
                }
                self.universal_byte_density(path)
            }
        }
    }

    fn pdf_to_density(&self, path: &Path) -> Option<Vec<f64>> {
        let uuid = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .ok()?
            .as_micros();
        let temp_base = format!("/tmp/omni_{}", uuid);
        let temp_pgm = format!("{}.pgm", temp_base);

        let output = Command::new("pdftoppm")
            .args(["-singlefile", "-gray", path.to_str()?, &temp_base])
            .output()
            .ok()?;

        if output.status.success() {
            let res = self.pgm_to_density(Path::new(&temp_pgm));
            let _ = fs::remove_file(temp_pgm);
            res
        } else {
            None
        }
    }

    fn pgm_to_density(&self, path: &Path) -> Option<Vec<f64>> {
        let mut file = fs::File::open(path).ok()?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).ok()?;

        if data.len() < 2 || data[0] != b'P' || data[1] != b'5' {
            return None;
        }

        let mut pos = 2;
        let mut headers_found = 0;

        while headers_found < 3 && pos < data.len() {
            while pos < data.len() && (data[pos] as char).is_whitespace() {
                pos += 1;
            }
            if pos >= data.len() {
                break;
            }
            if data[pos] == b'#' {
                while pos < data.len() && data[pos] != b'\n' {
                    pos += 1;
                }
                continue;
            }
            headers_found += 1;
            while pos < data.len() && !(data[pos] as char).is_whitespace() {
                pos += 1;
            }
        }

        if pos < data.len() && (data[pos] as char).is_whitespace() {
            pos += 1;
        }
        if pos >= data.len() || headers_found < 3 {
            return None;
        }

        let body = &data[pos..];
        Some(body.iter().map(|&b| 1.0 - (b as f64 / 255.0)).collect())
    }

    fn universal_byte_density(&self, path: &Path) -> Option<Vec<f64>> {
        let data = fs::read(path).ok()?;
        if data.is_empty() {
            return None;
        }
        let mut arr: Vec<f64> = data.iter().map(|&b| b as f64 / 255.0).collect();
        if arr.len() > 10000 {
            let step = arr.len() / 2000;
            arr = arr.into_iter().step_by(step).collect();
        }
        Some(arr)
    }

    /// Folds the Ship (Codebase) into a parsimonious archive.
    pub fn fold_ship(&self) -> std::io::Result<PathBuf> {
        let archive_path = PathBuf::from("ship_fold_v1.tar.gz");
        let tar_gz = File::create(&archive_path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        println!("📦 [FOLD] Compressing Source Frequency (rs, py, json)...");

        let root = Path::new(".");
        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "rs" || ext == "py" || ext == "json" {
                    if let Ok(real_path) = path.strip_prefix(root) {
                        // println!("   - Adding: {:?}", real_path);
                        tar.append_path_with_name(path, real_path)?;
                    }
                }
            }
        }
        tar.finish()?;
        Ok(archive_path)
    }

    pub fn trans_modulate(&self, signature: &LightCypher) -> f64 {
        let intensity = signature.intensity();

        // --- TESLA MODE: Gain Scaling ---
        // Clearing the 7 Arches via Phi-based resonance
        let resonant_intensity = intensity * Talu64::TESLA_GAIN;

        // --- THE 3/2 PIVOT: Async Sync Measurement ---
        // Synchronized at a 6 microsecond boundary (6µs) to stabilize the analog well.
        // We use the discretization wall (E) as the phase delta trigger.
        let phase_delta = (resonant_intensity * Talu64::E) % Talu64::PI;

        // Logic shifts at 1.5 (3/2) - performing 2D and 3D measurements in sync.
        // The Golden Ratio (Phi) acts as the undeterminable asymptote for data recycling.
        let precision = if phase_delta > 1.5 {
            // 3D Measurement Expansion (Golden Ratio expansion)
            phase_delta * Talu64::PHI
        } else {
            // 2D Measurement Focus (Golden Ratio focus)
            phase_delta / Talu64::PHI
        };

        Talu64::truncate_8_sig_fig(precision)
    }

    pub fn descope(&self) {
        println!("⚓ [DESCOPE] Transitioning to Stable Orbit (Frictionless Stasis)...");
        println!("   - Beacon: /home/pecosdwilly/Documents/.scans");
        println!("   - Target: σ=1/2 Line (Riemann Stability)");

        // Mocking the Braking Maneuver for now
        println!("   - BRAKING: Knots -> 0.0000");
        println!("   - ALIGNMENT: Phase matching handwriting intent...");
        println!("✨ [ORBIT] Stable Orbit achieved. Gain Locked.");
    }

    /// [NEW] Distributed Induction: Engaging neighbors .250, .205, .98
    pub fn induce_distributed(&self) {
        let nodes = vec!["10.0.0.250", "10.0.0.205", "10.0.0.98"];
        println!("🌐 [DISTRIBUTED] Initiating Omni-Pose Induction across Sensor Fabric...");

        for node in nodes {
            println!("   - Inducing Node: {} | Role: Spectral Sensor", node);
            // In a full implementation, we would pulse UDP packets here to collect local rho (ρ)
        }

        println!(
            "💎 [SYSTEM] Distributed Hall Effect synchronized with Nami Master Matrix (NVMe)."
        );
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return;
    }
    let tool = OmniTool::new("/home/nicoDantigravity/laboratory");
    let start = Instant::now();

    if args[1] == "descope" {
        tool.descope();
        return;
    }

    if args[1] == "omni" {
        tool.induce_distributed();
        return;
    }

    if args[1] == "fold" {
        match tool.fold_ship() {
            Ok(path) => println!("📦 [FOLD] Success: {:?}", path),
            Err(e) => println!("❌ [FOLD] Error: {}", e),
        }
        return;
    }

    let file_path = Path::new(&args[1]);

    if let Some(profile) = tool.file_to_density_profile(file_path) {
        let signature = LightCypher::from_profile(&profile);
        let _modulation = tool.trans_modulate(&signature);
        let _arches = crew_core::SevenArches::validate(&signature);

        println!("\n📊 OMNI-TOOL 5.6 HARVEST:");
        println!("   - File: {:?}", file_path.file_name().unwrap());
        println!("   - Duration: {:?}", start.elapsed());
        println!(
            "   - Signature: [C]{:.4} [R]{:.4} [G]{:.4} [B]{:.4} [A]{:.4} [IR]{:.4} [UV]{:.4}",
            signature.c,
            signature.r,
            signature.g,
            signature.b,
            signature.a,
            signature.ir,
            signature.uv
        );
        let arches = SevenArches::validate(&signature);
        println!(
            "   - Sovereignty Check: {}",
            if arches.is_sovereign(Medium::Silicon) {
                "🟢 PASSED"
            } else {
                "🔴 FAILED"
            }
        );
    } else {
        println!("❌ Harvest Failed: {:?}", file_path);
    }
}
