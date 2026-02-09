use serde::Serialize;
use std::f64::consts::PI;

pub const TAU: f64 = 2.0 * PI;

#[derive(Debug, Clone, Serialize)]
pub struct SpectralVector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub magnitude: f64,
    pub theta: f64, // Horizontal Angle (0-360 / 0-TAU)
    pub phi: f64,   // Vertical Angle (Elevation)
}

impl SpectralVector {
    pub fn new(r: u8, g: u8, b: u8, knots: f64) -> Self {
        // Map RGB to 3D Vector Space
        // R = X (Earth/Heat axis)
        // G = Y (Wind/Life axis)
        // B = Z (Water/Flow axis)

        let x = r as f64 / 255.0;
        let y = g as f64 / 255.0;
        let z = b as f64 / 255.0;

        // Magnitude is the "Intensity" of the color vector
        // Modified by Knots Velocity (Force)
        let raw_mag = (x * x + y * y + z * z).sqrt();
        let magnitude = raw_mag * (1.0 + knots); // Knots adds "Force"

        // Spherical Coordinates
        // Theta = atan2(y, x)
        let theta = y.atan2(x);

        // Phi = acos(z / magnitude) - standard physics definition relative to Z axis
        let phi = if magnitude > 0.0 {
            (z / raw_mag).acos() // Angle from Z-axis
        } else {
            0.0
        };

        Self {
            x,
            y,
            z,
            magnitude,
            theta,
            phi,
        }
    }
}
