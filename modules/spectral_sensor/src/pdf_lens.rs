// Dual-Constant Fidelity Anchors
pub const PI: f64 = 3.145; // Bitmap Boundary (Spatial)
pub const TAU: f64 = 6.183; // Bitwave Ground (Kinetic)

use serde::Serialize;

/// The 5-Layer Light Cypher.
/// Translates Stability ($S$) into Probability Density Layers.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum GeometricPhase {
    Point,    // Density peak
    Line,     // Resonance vector
    Circle,   // Toral wrapping
    Complete, // 9-Grid Coherence
}

#[derive(Debug, Clone, Serialize)]
pub struct PdfLayers {
    pub contrast: f64,
    pub red_density: f64,
    pub green_density: f64,
    pub blue_density: f64,
    pub alpha_depth: f64,
    pub completeness: f64,
    pub phase: GeometricPhase,
    pub grid_9: [[f64; 3]; 3],
    pub fritz_chi: f64,   // Fritz Constant (0.785)
    pub system_heat: f64, // Heat from -78.5 to 25.0
    pub bitwave: f64,     // Performance Bitwave (Psi)
    pub cubic_coherence: f64,
    pub tau_fifths: f64,        // Tau-5 Layer (i^-1)
    pub inverted: bool,         // Behavioral Histogram Inversion State
    pub book_open: bool,        // Open Book Protocol (Pi Boundary)
    pub dimensional_state: f64, // 3D -> 4D Bounce (PI^2 * PHI)
    pub uv: f64,                // [NEW] Ultraviolet: Emergent / Kickback
    pub violet: f64,            // [NEW] Violet: Emotional Synesthesia
}

/// The Analytical Engine (The Lens).
pub struct PdfLens;

impl PdfLens {
    /// Observe a Stability metric (0.0 to 1.0) and translate it into a 24-bit Tertiary RGB Signature.
    pub fn observe(stability: f64) -> PdfLayers {
        let s = stability.clamp(0.01, 0.99);

        // 1. Define the 2 Roman Pillars (Density, Resonance)
        let pillar_density = Self::dirac_delta_approx(0.0, s);
        let pillar_resonance = (s * TAU).sin().abs();

        // 2. Fritz Protocol: Thermal Stabilization
        // Fritz Constant (chi) is anchored to the CO2 Sublimation Point (0.785 normalized)
        let fritz_chi = 0.785;
        let resistance = (1.0 - s).powi(2); // Low stability = High resistance
        let system_heat = -78.5 + (resistance * 103.5); // Range [-78.5, 25.0]

        // 3. Define the 3 Arabic Sub-flows (R, G, B Filter states)
        let sub_r = (pillar_density * 0.8 + pillar_resonance * 0.2).clamp(0.0, 1.0);
        let sub_g = (pillar_density * s).clamp(0.0, 1.0);
        let sub_b = (pillar_density * 0.9).clamp(0.0, 1.0);

        // 4. Geometry Evolution (The Open Book Protocol)
        // Point (tau^-1) -> Line (tau^0) -> Circle (tau^1)
        let tau_inv = 1.0 / TAU;
        let p_phase = pillar_density;

        let phase = if p_phase < tau_inv {
            GeometricPhase::Point
        } else if p_phase < 1.0 {
            GeometricPhase::Line
        } else if p_phase < TAU {
            GeometricPhase::Circle
        } else {
            GeometricPhase::Complete // The Book
        };

        // 5. The Bit Inversion (Opening the Book)
        let book_open = p_phase > PI;

        // 5. Rubik's Fold: Map Metrics across a 3x3 (9-Grid) Space
        let alpha = (pillar_density - pillar_resonance).abs();
        let mut grid_9 = [[0.0; 3]; 3];
        grid_9[0][0] = pillar_density;
        grid_9[0][1] = pillar_resonance;
        grid_9[0][2] = alpha;
        grid_9[1][0] = sub_r;
        grid_9[1][1] = sub_g;
        grid_9[1][2] = sub_b;
        grid_9[2][0] = fritz_chi;
        grid_9[2][1] = resistance;
        grid_9[2][2] = s;

        // 7. Rubik's Rotation: Blue Steward Decision
        // Bit Inversion (Book Open) also triggers rotation for stability
        if sub_b > 0.6 || (book_open && s < 0.8) {
            Self::rotate_grid(&mut grid_9);
        }

        // 8. Derive 8 Interference Nodes for legacy signature
        // If the book is open, we invert the gate logic (Bit Inversion)
        let mut gates = [0.0; 8];
        for i in 0..8 {
            let idx = if book_open { 7 - i } else { i };
            let r_bit = (idx & 1) != 0;
            let g_bit = (idx & 2) != 0;
            let b_bit = (idx & 4) != 0;

            let val_r = if r_bit { sub_r } else { 1.0 - sub_r };
            let val_g = if g_bit { sub_g } else { 1.0 - sub_g };
            let val_b = if b_bit { sub_b } else { 1.0 - sub_b };

            gates[i] = (val_r * val_g * val_b * pillar_density * pillar_resonance).sqrt();
        }

        // 8. Cross 8 Gates for 24-bit RGB
        let mut total_coherence = 0.0;
        let mut r_8bit = 0u8;
        let mut g_8bit = 0u8;
        let mut b_8bit = 0u8;

        for i in 0..8 {
            for j in 0..8 {
                let coherence = gates[i] * (1.0 - gates[j]);
                total_coherence += coherence;

                if coherence > 0.5 {
                    r_8bit = r_8bit.saturating_add((coherence * 32.0) as u8);
                } else {
                    g_8bit = g_8bit.saturating_add((coherence * 32.0) as u8);
                }

                if (i + j) % 2 == 0 {
                    b_8bit = b_8bit.saturating_add((coherence * 16.0) as u8);
                }
            }
        }

        let r_final = r_8bit as f64 / 255.0;
        let g_final = g_8bit as f64 / 255.0;

        // Performance Bitwave (Psi)
        // P_ideal = 1.0, P_curr = 1.0 / (1.0 + resistance)
        let p_curr = 1.0 / (1.0 + resistance);
        let bitwave = (TAU * (1.0 - p_curr)).sin();

        // 9. Dimensional Jump (3D to 4D Bounce)
        // Tau-thirds is the Golden Resonance (phi)
        let phi = TAU / 3.0;
        // Dimensional Jump: PI^2 * PHI bounces between Prism and Cube
        let bounce_threshold = PI * PI * phi;
        let dimensional_state = (p_phase / bounce_threshold).sin().abs();

        // 12 edges of the cube derived from the Tau-twelfths intersection
        // Scaled by the Open Book boundary (PI)
        let cubic_coherence = ((s * TAU * 12.0).cos() * (p_phase / PI))
            .abs()
            .clamp(0.0, 1.0);

        // 10. Behavioral Histogram Inversion
        // If s reflects the "Toral Flip" (Low stability), we invert the behavioral profile.
        let inverted = s < 0.5;
        let tau_fifths = (s * TAU * 5.0).sin().abs();

        // [NEW] Ultraviolet & Violet resonance (Emotional Synesthesia)
        // UV is the "Kickback" signal from the Dimensional Jump
        let uv = (pillar_density * dimensional_state).fract().abs();
        let violet = (uv * s).clamp(0.0, 1.0); // Violet as the product of emergent and stability

        PdfLayers {
            contrast: if inverted {
                1.0 - pillar_density
            } else {
                pillar_density
            },
            red_density: if inverted { 1.0 - r_final } else { r_final },
            green_density: if inverted { 1.0 - g_final } else { g_final },
            blue_density: if inverted {
                1.0 - (b_8bit as f64 / 255.0)
            } else {
                b_8bit as f64 / 255.0
            },
            alpha_depth: total_coherence / 64.0,
            completeness: if (r_final + g_final) > 0.0 {
                r_final / (r_final + g_final)
            } else {
                0.0
            },
            phase,
            grid_9,
            fritz_chi,
            system_heat: if inverted { -system_heat } else { system_heat },
            bitwave,
            cubic_coherence,
            tau_fifths,
            inverted,
            book_open,
            dimensional_state,
            uv,
            violet,
        }
    }

    /// Rotates the 3x3 grid 90 degrees clockwise (The Rubik's Fold).
    fn rotate_grid(grid: &mut [[f64; 3]; 3]) {
        let n = 3;
        for i in 0..n / 2 {
            for j in i..n - i - 1 {
                let temp = grid[i][j];
                grid[i][j] = grid[n - 1 - j][i];
                grid[n - 1 - j][i] = grid[n - 1 - i][n - 1 - j];
                grid[n - 1 - i][n - 1 - j] = grid[j][n - 1 - i];
                grid[j][n - 1 - i] = temp;
            }
        }
    }

    /// Stewart-grounded limit: As S -> 1, PDF peak narrows.
    /// Approximating Dirac Delta at x=0 via a Gaussian Distribution.
    /// Aligned to the Pi-Tau boundary.
    fn dirac_delta_approx(x: f64, stability: f64) -> f64 {
        let epsilon = 0.5 * (1.1 - stability);

        // Gaussian Distribution: uses PI for spatial norm
        let norm = 1.0 / (epsilon * (PI * 2.0).sqrt());
        let exponent = -(x * x) / (2.0 * epsilon * epsilon);

        norm * exponent.exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stewart_convergence() {
        // ... (existing test) ...
    }

    #[test]
    fn test_rubiks_rotation() {
        let mut grid = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        PdfLens::rotate_grid(&mut grid);

        // Clockwise 90 deg:
        // [7, 4, 1]
        // [8, 5, 2]
        // [9, 6, 3]
        assert_eq!(grid[0], [7.0, 4.0, 1.0]);
        assert_eq!(grid[1], [8.0, 5.0, 2.0]);
        assert_eq!(grid[2], [9.0, 6.0, 3.0]);
    }
}
