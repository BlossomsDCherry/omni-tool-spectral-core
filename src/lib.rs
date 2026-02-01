// Copyright 2026 Pecos D. Willy
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Omni-Tool: Spectral Masses & Harmonic Entrainment
//!
//! This crate implements the core logic for spectral mass integration,
//! phase entrainment, and autonomous agentic frequency transformation.
//!
//! See README.md and SPECTRAL_NOTES.md for conceptual overview.

pub mod d16_driver;
pub mod projection_shim;
pub mod soft_fpga;
pub mod unoq_shim;

pub mod spectral {
    /// Spectral mass: a stable, lossless, quantized oscillatory configuration.
    #[derive(Debug, Clone, Copy)]
    pub struct SpectralMass {
        pub frequency: f64,
        pub amplitude: f64,
        pub phase: f64,
    }

    impl SpectralMass {
        pub fn new(freq: f64, amp: f64, phase: f64) -> Self {
            SpectralMass {
                frequency: freq,
                amplitude: amp,
                phase: phase,
            }
        }
    }
}

pub mod omnilayer {
    use crate::d16_driver::{D16Driver, HardwareArchitecture};

    /// 16-layer omni-tool integration logic with 15 passes per layer.
    pub fn integrate_spectral_masses(density: f64, time: f64, arch: HardwareArchitecture) -> bool {
        let mut driver = D16Driver::new(arch);
        driver.integrate(density, time);

        println!(
            "✅ [OMNILAYER] 16-Layer Integration Complete on Architecture: {:?}",
            arch
        );
        println!("🛸 [D16] Velocity: {:?}", driver.velocity);
        println!("⚖️ [D16] Pressure: {:.8}", driver.pressure);

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_mass_creation() {
        let mass = spectral::SpectralMass::new(1.0, 0.5, 0.0);
        assert!((mass.frequency - 1.0).abs() < 1e-9);
    }
}
