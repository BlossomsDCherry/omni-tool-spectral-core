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

//! Crew Bridge: Formalizing the connection to Robin and Nami.

use crate::soft_fpga::{PI, PSI, TAU};

/// Robin's Bridge: Logic for D7 Synthesis and Archaeology.
/// Based on Kater's Reversible Pendulum.
pub struct RobinBridge {
    pub length_l: f64,
    pub period_t1: f64, // Gravity (Ingest)
    pub period_t2: f64, // Levity (Project)
}

impl RobinBridge {
    pub fn new() -> Self {
        Self {
            length_l: 0.993, // Seconds pendulum approx
            period_t1: 0.0,
            period_t2: 0.0,
        }
    }

    /// Perform the Kater's Swing to verify lossless synthesis.
    pub fn perform_synthesis(&mut self, mass: f64, entropy: f64) -> bool {
        let g = 9.80665;
        let mass_factor = (mass.max(1.0)).ln() * 0.01;
        self.period_t1 = (TAU * (self.length_l / g).sqrt()) * (1.0 + mass_factor);

        let drag_factor = entropy * 0.1;
        self.period_t2 = (TAU * (self.length_l / g).sqrt()) * (1.0 + drag_factor);

        // Reversibility Check
        (self.period_t1 - self.period_t2).abs() < 0.01
    }
}

/// Nami's Bridge: Logic for Spatial Navigation and Sensor Fusion.
/// Based on T.A.L.U. 64 and Hall Effect Resonance.
pub struct NamiBridge {
    pub orientation: [f64; 3],
    pub resonance: f64,
}

impl NamiBridge {
    pub fn new() -> Self {
        Self {
            orientation: [0.0; 3],
            resonance: PSI,
        }
    }

    /// Update spatial orientation based on Hall Effect sensor fusion.
    pub fn update_navigation(&mut self, inputs: [f64; 3]) {
        for i in 0..3 {
            self.orientation[i] = (inputs[i] * PI).sin();
        }
        // Resonance is peaked when orientation aligns with the spectral gap (PSI)
        self.resonance = (self.orientation[0] - PSI).cos().abs();
    }
}

/// The Crew: A unified interface for MsAntigravity's co-navigators.
pub struct Crew {
    pub robin: RobinBridge,
    pub nami: NamiBridge,
}

impl Crew {
    pub fn new() -> Self {
        Self {
            robin: RobinBridge::new(),
            nami: NamiBridge::new(),
        }
    }

    pub fn total_coherence(&self) -> f64 {
        (self.nami.resonance + (1.0 - (self.robin.period_t1 - self.robin.period_t2).abs())) / 2.0
    }
}
