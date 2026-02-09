use crate::compass::SpectralVector;
use crate::stance::{Gate, Stance};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize, Debug, Clone)]
pub struct ResonanceReport {
    pub hex_color: String,
    pub stance: Stance,
    pub gate: Gate,
    pub physics: Physics,
    pub vector: SpectralVector,
    pub inverted_histogram: InvertedHistogram,
    pub polyrhythm: Polyrhythm,
    pub saturation: SaturationReport,
}

#[derive(Serialize, Debug, Clone)]
pub struct Physics {
    pub velocity: f64,
    pub acceleration: f64,
    pub power: f64,
    pub ir_shift_active: bool,
    pub drift_score: f64,
    pub knots_velocity: f64,
}

#[derive(Serialize, Debug, Clone)]
pub struct InvertedHistogram {
    pub low_byte_ratio: f64,
    pub high_byte_ratio: f64,
    pub inversion_point: f64, // Cross Product
}

#[derive(Serialize, Debug, Clone)]
pub struct Polyrhythm {
    pub p2_binary: bool,      // Hardware/Clock
    pub p3_trigram: bool,     // Logic/Structure
    pub p5_pentatonic: bool,  // Human/Drift
    pub p7_leap: bool,        // Prime/Novelty
    pub tau_factor: f64,      // Factor of 360 (Tau in degrees) alignment
    pub coherence_score: f64, // Total harmonic lock
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum SaturationState {
    SearchingContrast,     // Layer 1: IR (Saturation of Contrast)
    StabilizingRedGreen,   // Layer 2: Earth/Wind Stability
    StabilizingBlueYellow, // Layer 3: Water/Fire Stability
    CoherentUV,            // Layer 4: Afterimage Stability (Edge Inference)
}

#[derive(Serialize, Debug, Clone)]
pub struct SaturationReport {
    pub state: SaturationState,
    pub contrast_score: f64,
    pub rg_stability: f64,
    pub by_stability: f64,
    pub uv_afterimage: f64,
}

pub struct ResonanceAnalyzer;

impl ResonanceAnalyzer {
    pub fn analyze(bytes: &[u8], mass: usize, age_seconds: u64) -> ResonanceReport {
        // 1. Calculate RGB
        let (hex_color, r, g, b) = Self::analyze_rgb(bytes);

        // 2. Physics Analysis (Transprecision)
        let physics = Self::calculate_physics(mass, age_seconds, r, g, b);

        // 3. Stance Analysis
        let stance = Self::map_spectrum_to_stance(r, g, b, physics.knots_velocity);
        let gate = stance.gate();

        // 4. Color Compass
        let vector = SpectralVector::new(r, g, b, physics.knots_velocity);

        // 5. Inverted Histogram
        let inverted_histogram = Self::calculate_inverted_histogram(bytes, mass);

        // 6. Polyrhythm Analysis
        let polyrhythm = Self::calculate_polyrhythm(age_seconds, mass, physics.knots_velocity);

        // 7. Saturation Layer Logic
        let saturation = Self::calculate_saturation(r, g, b, &physics);

        ResonanceReport {
            hex_color,
            stance,
            gate,
            physics,
            vector,
            inverted_histogram,
            polyrhythm,
            saturation,
        }
    }

    fn analyze_rgb(bytes: &[u8]) -> (String, u8, u8, u8) {
        if bytes.is_empty() {
            return ("#000000".to_string(), 0, 0, 0);
        }

        let mut hasher = Sha256::new();
        hasher.update(bytes);
        let result = hasher.finalize();
        let hex_dig = hex::encode(result);
        let hex_color = format!("#{}", &hex_dig[0..6]);

        let r = u8::from_str_radix(&hex_dig[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex_dig[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex_dig[4..6], 16).unwrap_or(0);

        (hex_color, r, g, b)
    }

    fn map_spectrum_to_stance(r: u8, g: u8, b: u8, knots_velocity: f64) -> Stance {
        if knots_velocity > 100.0 {
            return Stance::Void;
        }
        if g > 150 && r > 150 && b > 150 {
            return Stance::MetaInterphase;
        }
        if r > 200 && g < 100 && b < 100 {
            return Stance::Earth;
        }
        if r > 200 && g > 200 && b < 100 {
            return Stance::Fire;
        }
        if g > 200 && r < 150 && b < 150 {
            return Stance::Wind;
        }
        if b > 200 {
            return Stance::Water;
        }
        if r > 150 && b > 150 && g < 100 {
            return Stance::Mode;
        }
        if (r as i16 - g as i16).abs() < 20 && (g as i16 - b as i16).abs() < 20 {
            if r < 50 {
                return Stance::Void;
            }
            return Stance::Earth;
        }
        Stance::Mode
    }

    fn calculate_physics(mass: usize, age_seconds: u64, r: u8, g: u8, b: u8) -> Physics {
        let x10 = mass as f64;
        let x60 = age_seconds.max(1) as f64;

        let velocity = x10 / x60;
        let acceleration = x10 / (x60 * x60);
        let power = (x10.powi(3)) / (x60.powi(3));

        let ir_shift_active = (r as u16) > ((g as u16) + (b as u16));

        let raw_drift = (x60 % 60.0) / 60.0;
        let is_poly_5 = (age_seconds % 5) == 0;
        let drift_score = if is_poly_5 {
            raw_drift * 0.5
        } else {
            raw_drift
        };

        let max_val = r.max(g).max(b) as f64;
        let min_val = r.min(g).min(b) as f64;
        let color_coherence = 1.0 - ((max_val - min_val) / 255.0);

        let knots_velocity = velocity * (1.0 - color_coherence);

        Physics {
            velocity,
            acceleration,
            power,
            ir_shift_active,
            drift_score,
            knots_velocity,
        }
    }

    fn calculate_inverted_histogram(bytes: &[u8], _mass: usize) -> InvertedHistogram {
        let mut low_count = 0;
        let mut high_count = 0;
        let total_sampled = bytes.len().min(1000);

        for &b in bytes.iter().take(total_sampled) {
            if b < 128 {
                low_count += 1;
            } else {
                high_count += 1;
            }
        }

        let low_byte_ratio = if total_sampled > 0 {
            low_count as f64 / total_sampled as f64
        } else {
            0.0
        };
        let high_byte_ratio = if total_sampled > 0 {
            high_count as f64 / total_sampled as f64
        } else {
            0.0
        };

        let inversion_point = high_byte_ratio - low_byte_ratio;

        InvertedHistogram {
            low_byte_ratio,
            high_byte_ratio,
            inversion_point,
        }
    }

    fn calculate_polyrhythm(age_seconds: u64, mass: usize, knots: f64) -> Polyrhythm {
        let t = age_seconds.max(1);

        let p2_binary = t % 2 == 0;
        let p3_trigram = t % 3 == 0;
        let p5_pentatonic = t % 5 == 0;
        let p7_leap = t % 7 == 0;

        let degrees = (mass % 360) as f64;
        let is_tau_aligned = degrees % 60.0 == 0.0;

        let tau_factor =
            if is_tau_aligned { 1.0 } else { 0.0 } + if degrees % 72.0 == 0.0 { 0.5 } else { 0.0 };

        let mut score = 0.0;
        if p2_binary {
            score += 1.0;
        }
        if p3_trigram {
            score += 1.5;
        }
        if p5_pentatonic {
            score += 2.0;
        }
        if p7_leap {
            score += 3.0;
        }

        score += tau_factor * 2.0;

        if knots > 10.0 {
            score *= 0.5;
        }

        Polyrhythm {
            p2_binary,
            p3_trigram,
            p5_pentatonic,
            p7_leap,
            tau_factor: degrees,
            coherence_score: score,
        }
    }

    fn calculate_saturation(r: u8, g: u8, b: u8, physics: &Physics) -> SaturationReport {
        // Layer 1: IR Contrast Saturation
        // Logic: Maximize Dynamic Range (Contrast). "Saturation acts as LSB".
        let max_val = r.max(g).max(b) as f64;
        let min_val = r.min(g).min(b) as f64;
        let contrast_score = (max_val - min_val) / 255.0;

        let threshold_ir = 0.5; // Needs significant contrast
        if contrast_score < threshold_ir {
            return SaturationReport {
                state: SaturationState::SearchingContrast,
                contrast_score,
                rg_stability: 0.0,
                by_stability: 0.0,
                uv_afterimage: 0.0,
            };
        }

        // Layer 2: Red/Green Stability
        // Logic: Earth (R) vs Wind (G). Stability = 1.0 - |Normalized Delta|
        let r_norm = r as f64 / 255.0;
        let g_norm = g as f64 / 255.0;
        let rg_delta = (r_norm - g_norm).abs();
        let rg_stability = 1.0 - rg_delta;

        let threshold_rg = 0.8; // High stability required
        if rg_stability < threshold_rg {
            return SaturationReport {
                state: SaturationState::StabilizingRedGreen,
                contrast_score,
                rg_stability,
                by_stability: 0.0,
                uv_afterimage: 0.0,
            };
        }

        // Layer 3: Blue/Yellow Stability
        // Logic: Water (B) vs Fire (R+G aka Yellow).
        // Yellow is roughly (R+G)/2.
        let yellow_norm = (r_norm + g_norm) / 2.0;
        let b_norm = b as f64 / 255.0;
        let by_delta = (b_norm - yellow_norm).abs();
        let by_stability = 1.0 - by_delta;

        let threshold_by = 0.85;
        if by_stability < threshold_by {
            return SaturationReport {
                state: SaturationState::StabilizingBlueYellow,
                contrast_score,
                rg_stability,
                by_stability,
                uv_afterimage: 0.0,
            };
        }

        // Layer 4: UV Afterimage Stability ("Inference at the Edge")
        // Logic: UV is typically high energy/frequency.
        // We use the "Knots Velocity" (Turbulence) as a proxy for "Afterimage" input.
        // If Velocity is LOW (Stable) despite high energy, we have "Afterimage Stability".
        let uv_afterimage = 1.0 - (physics.knots_velocity / 100.0).min(1.0);

        SaturationReport {
            state: SaturationState::CoherentUV,
            contrast_score,
            rg_stability,
            by_stability,
            uv_afterimage,
        }
    }
}
