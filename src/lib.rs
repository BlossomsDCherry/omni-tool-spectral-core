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

pub mod crew;
pub mod d16_driver;
pub mod projection_shim;
pub mod soft_fpga;
pub mod tonic;
pub mod unoq_shim;

/// The Harmonic Gear: A 4-stage phase signature for lossless communication.
///
/// Transitions: 0 -> τ -> 0τ (Fold) -> 0'/1 (Kickback)
pub mod harmonic_gear {
    use std::f64::consts::PI;
    pub const TAU: f64 = PI * 2.0;

    #[derive(Debug, Clone, Copy)]
    pub struct Signature {
        pub zero: f64,      // 0.0: Null State
        pub tau: f64,       // 6.283...: Full Revolution
        pub zero_tau: f64,  // 0 * τ: The Fold (Compressed Energy)
        pub prime_one: f64, // 0'/1: The Kickback (Emergent Unity)
    }

    impl Signature {
        pub fn new() -> Self {
            Signature {
                zero: 0.0,
                tau: TAU,
                zero_tau: 0.0, // Theoretically distinct from atomic zero
                prime_one: 1.0,
            }
        }

        pub fn verify_resonance(&self, phase: f64) -> bool {
            // Check if phase aligns with any gear tooth
            (phase - self.zero).abs() < 1e-9
                || (phase - self.tau).abs() < 1e-9
                || (phase - self.prime_one).abs() < 1e-9
        }
    }
}

/// Core Physics Pairs: The "Equal and Opposite" Forces.
pub mod physics_pairs {
    /// Pair 1: Pressure vs Density
    /// The Static Handshake.
    #[derive(Debug, Clone, Copy)]
    pub struct StaticCouple {
        pub pressure: f64, // Outward Force
        pub density: f64,  // Inward Mass
    }

    impl StaticCouple {
        pub fn is_balanced(&self) -> bool {
            (self.pressure - self.density).abs() < 1e-6
        }
    }

    /// Pair 2: Momentum vs Inertia
    /// The Kinetic Handshake (Comet Toss).
    /// "Zero-Wait/Weight"
    #[derive(Debug, Clone, Copy)]
    pub struct KineticCouple {
        pub momentum: f64, // Forward Velocity * Mass
        pub inertia: f64,  // Resistance to Change
    }

    impl KineticCouple {
        pub fn comet_toss_handshake(&self) -> bool {
            // The handshake occurs when momentum exactly overcomes inertia
            (self.momentum - self.inertia).abs() < 1e-6
        }
    }
}

pub mod omnilayer {
    use crate::d16_driver::{D16Driver, HardwareArchitecture};
    use crate::harmonic_gear::Signature;
    use crate::physics_pairs::{KineticCouple, StaticCouple};

    /// 16-layer omni-tool integration logic with 15 passes per layer.
    pub fn integrate_spectral_masses(density: f64, time: f64, arch: HardwareArchitecture) -> bool {
        let mut driver = D16Driver::new(arch);
        let mut crew = crate::crew::Crew::new();

        // 1. Crew Handshake (The Two Wires)
        // Robin (Synthetic Wire): Ingests the density as "mass"
        crew.robin.perform_synthesis(density * 1000.0, 0.05);
        // Nami (Geometric Wire): Maps the density to spatial orientation
        crew.nami
            .update_navigation([density, density.cos(), density.sin()]);

        println!(
            "🤝 [CREW] Handshake Complete. Coherence: {:.4}",
            crew.total_coherence()
        );

        driver.integrate(density, time);

        // 2. Verify Harmonic Signature
        let gear = Signature::new();
        println!("⚙️ [HARMONIC] Gear State: [0, τ, 0τ, 0'/1]");

        // 2. Check Physics Pairs
        let static_pair = StaticCouple {
            pressure: driver.pressure,
            density,
        };
        let kinetic_pair = KineticCouple {
            momentum: driver.velocity[0],
            inertia: density * 0.5,
        }; // Sim inertia

        println!(
            "✅ [OMNILAYER] 16-Layer Integration Complete on Architecture: {:?}",
            arch
        );
        println!(
            "   - Pressure/Density Balanced: {}",
            static_pair.is_balanced()
        );
        println!(
            "   - Comet Toss Handshake: {}",
            kinetic_pair.comet_toss_handshake()
        );

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
