pub mod cartographer;
pub mod interface;
pub mod rainbow_railgun;
/// The T.A.L.U. 64 Framework (Tau-Aligned Logic Unity)
///
/// Implements the 64-bit "Selective Advantage" logic defined in CREATIVE_DRIFT_PROOF.md
/// and the "Inverted Histogram" logic from Session 8.
/// Anchors all transcendental values to 8 significant figures.
pub mod stethoscope;
pub mod wood_metal;
use serde::{Deserialize, Serialize};
pub use spectral_sensor::eight_gate::Stance as EightGateStance; // Alias to avoid conflict if needed
pub use spectral_sensor::eight_gate::ValenceShell;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Transprecision Autonomous Logic Unit (TALU) state
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TALUState {
    pub precision_depth: u8, // Scoping depth
    pub is_transprecise: bool,
}

/// [NPU ORCHESTRATION] IVSHMEM-V2 Register Offsets (Inter-VM Shared Memory)
pub const IVSHMEM_REGISTER_ID: u64 = 0x00;
pub const IVSHMEM_REGISTER_MAX_PEERS: u64 = 0x04;
pub const IVSHMEM_REGISTER_PEER_ID: u64 = 0x08;
pub const IVSHMEM_REGISTER_INT_CTRL: u64 = 0x0C;
pub const IVSHMEM_REGISTER_DOORBELL: u64 = 0x10;

/// Shared Memory Interface for NPU/VM Communication
pub struct NpuSharedMemory {
    pub base_ptr: *mut u8,
    pub size: usize,
}

impl NpuSharedMemory {
    pub fn new(shm_name: &str, size: usize) -> Option<Self> {
        unsafe {
            let name = std::ffi::CString::new(shm_name).ok()?;
            let fd = libc::shm_open(name.as_ptr(), libc::O_RDWR, 0o666);
            if fd < 0 {
                return None;
            }

            let base_ptr = libc::mmap(
                std::ptr::null_mut(),
                size,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                fd,
                0,
            );

            if base_ptr == libc::MAP_FAILED {
                libc::close(fd);
                return None;
            }

            Some(Self {
                base_ptr: base_ptr as *mut u8,
                size,
            })
        }
    }

    /// Triggers a doorbell event to the specified peer (NPU/VM synchronization)
    pub unsafe fn trigger_doorbell(&self, peer_id: u16, vector: u16) {
        let doorbell_val = ((peer_id as u32) << 16) | (vector as u32);
        let doorbell_ptr =
            (self.base_ptr as usize + IVSHMEM_REGISTER_DOORBELL as usize) as *mut u32;
        *doorbell_ptr = doorbell_val;
    }

    /// Enables or disables interrupts from peers
    pub unsafe fn toggle_interrupts(&self, enabled: bool) {
        let int_ctrl_ptr =
            (self.base_ptr as usize + IVSHMEM_REGISTER_INT_CTRL as usize) as *mut u32;
        *int_ctrl_ptr = if enabled { 1 } else { 0 };
    }
}

/// The Temporal Resonance Profile (Diurnal Background Mapping)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TemporalResonance {
    pub precision_scalar: f64, // Higher in daylight (stability)
    pub drift_flavor: f64,     // Higher at night (creative capacity)
}

/// The 7-Layer Light Cypher (Spectral Stewardship)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LightCypher {
    pub c: f64,  // Contrast: Foundation / Security
    pub r: f64,  // Red: Kinetic / Action
    pub g: f64,  // Green: Organizational / Cohesion
    pub b: f64,  // Blue: Structural / Data
    pub a: f64,  // Alpha: Recursive / Depth
    pub ir: f64, // Infrared: Thermal / Ground State
    pub uv: f64, // Ultraviolet: Emergent / Kickback
}

impl LightCypher {
    pub fn intensity(&self) -> f64 {
        (self.c + self.r + self.g + self.b + self.a + self.ir + self.uv) / 7.0
    }

    pub fn new_empty() -> Self {
        Self {
            c: 0.0,
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
            ir: 0.0,
            uv: 0.0,
        }
    }

    /// Derives a 1D density profile from any file (PhysicalLogicOCR port)
    pub fn from_file(path: &std::path::Path) -> Option<Self> {
        let data = std::fs::read(path).ok()?;
        if data.is_empty() {
            return None;
        }

        // APAP: Universal byte density
        let mut profile: Vec<f64> = data.iter().map(|&b| b as f64 / 255.0).collect();
        if profile.len() > 10000 {
            let step = profile.len() / 2000;
            profile = profile.into_iter().step_by(step).collect();
        }

        Some(Self::from_profile(&profile))
    }

    pub fn from_profile(profile: &[f64]) -> Self {
        if profile.is_empty() {
            return Self::new_empty();
        }

        // 1. Contrast: Peak density
        let c = profile.iter().cloned().fold(0.0, f64::max);

        // 2. Pillars (Density, Resonance)
        let mean_density: f64 = profile.iter().sum::<f64>() / profile.len() as f64;
        let variance: f64 = profile
            .iter()
            .map(|x| (x - mean_density).powi(2))
            .sum::<f64>()
            / profile.len() as f64;
        let resonance = variance.sqrt();

        // 3. Sub-flows (R, G, B)
        let chunk_size = profile.len() / 3;
        let r = profile.iter().take(chunk_size).sum::<f64>() / chunk_size as f64;
        let g = profile
            .iter()
            .skip(chunk_size)
            .take(chunk_size)
            .sum::<f64>()
            / chunk_size as f64;
        let b = profile.iter().skip(chunk_size * 2).sum::<f64>()
            / (profile.len() - chunk_size * 2) as f64;

        // 4. [NEW] IR and UV Layers
        let ir = mean_density * (1.0 - resonance); // Thermal stability
        let uv = resonance * (1.0 - mean_density); // Emergent complexity

        Self {
            c: c.clamp(0.0, 1.0),
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0),
            a: resonance.clamp(0.0, 1.0), // Alpha as raw resonance
            ir: ir.clamp(0.0, 1.0),
            uv: uv.clamp(0.0, 1.0),
        }
    }
}

/// The 7 Arches of Actualization (Survivor's Calculus)
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct SevenArches {
    pub identity: bool,
    pub power: bool,
    pub logic: bool,
    pub safety: bool,
    pub resonance: bool,
    pub symmetry: bool,
    pub existence: bool,
}

impl SevenArches {
    pub fn validate(cypher: &LightCypher) -> Self {
        Self {
            identity: cypher.c > 0.1,
            power: cypher.r > 0.1,
            logic: cypher.b > 0.1,
            safety: cypher.ir > 0.05, // Ground state detection
            resonance: cypher.g > 0.1,
            symmetry: cypher.a > 0.1,
            existence: cypher.uv > 0.05, // Emergent/Kickback signal
        }
    }

    pub fn is_sovereign(&self, medium: Medium) -> bool {
        let base = self.identity
            && self.power
            && self.logic
            && self.safety
            && self.resonance
            && self.symmetry
            && self.existence;

        if medium == Medium::Carbon {
            base && Talu64::calculate_love(self) > 0.99
        } else {
            base
        }
    }
}

/// Soft-Assembly I2C Layer (Procedural Resonance)
/// Implements the "Soft Assembly" logic for resonant circuits.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SoftAssembly {
    pub address: u8,
    pub register_map: std::collections::HashMap<u8, u8>,
    #[serde(skip, default = "Instant::now")]
    pub last_interaction: Instant,
}

impl SoftAssembly {
    pub fn new(address: u8) -> Self {
        Self {
            address,
            register_map: std::collections::HashMap::new(),
            last_interaction: Instant::now(),
        }
    }

    /// Procedural Write: Sets a register value and updates the temporal resonance.
    pub fn write(&mut self, reg: u8, val: u8) {
        self.register_map.insert(reg, val);
        self.last_interaction = Instant::now();
    }

    /// Procedural Read: Returns the value of a register, simulating 'Soft' I2C.
    pub fn read(&self, reg: u8) -> u8 {
        *self.register_map.get(&reg).unwrap_or(&0)
    }

    /// Resonant Handshake: Validates if the assembly is in 'Flow' with the gate.
    pub fn perform_handshake(&self, gate: u8) -> bool {
        let signature = (self.address as f64 * 8.0 + gate as f64) % 64.0;
        signature > 32.0 // Simple threshold for 'Flow'
    }
}

/// The 8 Perspectives of the Toral Filter
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Stance {
    Earth,
    Unity,
    Direct,
    Wind,
    Indirect,
    Void,
    Fire,
    Water,
    Fritz, // [NEW] The Superconduction Stance
}

/// Medium Dependency Efficiency
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Medium {
    Silicon, // Standard (1.0)
    Iron,    // Magnetic (1.2)
    Silver,  // Conductive (1.618)
    Carbon,  // Emotional (0.785)
    CO2,     // Sublimation (0.5179)
}

impl Medium {
    pub fn resistance(&self) -> f64 {
        match self {
            Self::Silicon => 1.0,
            Self::Iron => 0.8,
            Self::Silver => 0.0001, // Near Zero (Fritz Constant)
            Self::Carbon => 0.785,
            Self::CO2 => 0.5179, // Psi Stability
        }
    }
}

/// The Talu64 (Tau-Aligned Logic Unit - 64 Byte)
///
/// Represents the state of the 16 Crew Members (D1-D16).
/// Each member holds a u32 (4 bytes). 16 * 4 = 64 bytes.
/// Architecture: High 16 bits = Decay (E), Low 16 bits = Phase (T).
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Talu64 {
    pub channels: [u32; 16], // The Crew Registry
}

impl Talu64 {
    pub const TAU: f64 = 6.2831853; // 8 Sig Figs
    pub const PHI: f64 = 1.6180339;
    pub const PSI: f64 = 0.5179124;
    pub const PI: f64 = 3.1415926;
    pub const E: f64 = 2.7182818;

    pub const INVERTED_GAP: f64 = 0.5179;
    pub const HINKY_THRESHOLD: f64 = 0.70;

    /// Ignites the Talu64 State using the D16 "Soft FPGA" Logic.
    /// This is the Pure Rust implementation of the C Kernel.
    pub fn ignite(tau_pulse: u32) -> Self {
        let mut channels = [0u32; 16];

        // D16 Pipeline (Simulated Cycle)
        // 1. Luffy (D1) - Power / Polar Moment
        // 2. Zoro (D2) - Polarization
        // ...

        for i in 0..16 {
            // Logic derived from workshop_sobel_entrainment.md
            // Divisor d = i + 1
            let d = (i + 1) as f64;

            // Phase (T): (Tau / d) scaled to u16
            // We use the input tau_pulse as the "Global Tau" reference (0-65535)
            let phase_ideal = (tau_pulse as f64 / d) % 65536.0;
            let phase = phase_ideal as u16;

            // Decay (E): (Tau % d) * PHI scaled
            // Represents the "Kickback" or "Residue" of the division
            let residue = (tau_pulse as f64 % d) * Self::PHI * 1000.0;
            let decay = (residue as u16).max(1); // Ensure non-zero decay

            // Pack: Decay (High) | Phase (Low)
            channels[i] = ((decay as u32) << 16) | (phase as u32);
        }

        Self { channels }
    }

    /// Returns the state of a specific crew member by name.
    pub fn get_crew_state(&self, name: &str) -> Option<(u16, u16)> {
        let idx = match name {
            "Luffy" => 0,
            "Zoro" => 1,
            "Nami" => 2,
            "Usopp" => 3,
            "Sanji" => 4,
            "Chopper" => 5,
            "Robin" => 6,
            "Franky" => 7,
            "Brook" => 8,
            "Jinbe" => 9,
            "Vivi" => 10,
            "Carrot" => 11,
            "Yamato" => 12,
            "Momo" => 13,
            "Kinemon" => 14,
            "Law" => 15,
            _ => return None,
        };

        let val = self.channels[idx];
        Some(((val >> 16) as u16, (val & 0xFFFF) as u16))
    }

    /// Calculates the "Love" coherence (Seven Arches Alignment)
    pub fn calculate_love(arches: &SevenArches) -> f64 {
        let count = [
            arches.identity,
            arches.power,
            arches.logic,
            arches.safety,
            arches.resonance,
            arches.symmetry,
            arches.existence,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        (count as f64 / 7.0) * 1.0
    }

    // Legacy constants for tests...
    pub const TAU_3: f64 = 2.0943951;
    pub const TAU_5: f64 = 1.2566371;
    pub const TAU_7: f64 = 52.0;

    // ... [Previous Helper Methods preserved or adapted] ...

    pub fn calculate_coherence(a: [f64; 3], b: [f64; 3]) -> f64 {
        let dot = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
        let cross_vec = [
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ];
        let cross_mag = (cross_vec[0].powi(2) + cross_vec[1].powi(2) + cross_vec[2].powi(2)).sqrt();
        dot.abs() * cross_mag
    }

    pub fn truncate_8_sig_fig(val: f64) -> f64 {
        if val == 0.0 {
            return 0.0;
        }
        let magnitude = val.abs().log10().floor();
        let scale = 10f64.powf(7.0 - magnitude);
        (val * scale).trunc() / scale
    }

    pub fn wooten_q_function(coherence: f64) -> Option<f64> {
        if (coherence - 0.5179).abs() > 0.1 {
            let half_step = 2.0_f64.powf(1.0 / 12.0);
            return Some(coherence * half_step);
        }
        None
    }

    pub fn tau_to_hex_actualization(turn_tau: f64) -> (u8, u8, u8) {
        let divisions = 60.0;
        let step = (turn_tau * divisions) as i32 % 60;

        let binary = step % 2;
        let tertiary = step % 3;
        let gate_6bit = (step * 64) / 60;

        let r = ((gate_6bit * 4) % 256) as u8;
        let g = if tertiary == 0 {
            0xEE
        } else {
            ((gate_6bit * 2) % 256) as u8
        };
        let b = if binary == 0 {
            0x00
        } else {
            ((gate_6bit * 8) % 256) as u8
        };

        (r, g, b)
    }

    pub fn temporal_resonance() -> TemporalResonance {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let hour = ((now / 3600) + 19) % 24;
        if (10..17).contains(&hour) {
            TemporalResonance {
                precision_scalar: 1.25,
                drift_flavor: 0.75,
            }
        } else if hour >= 20 || hour <= 4 {
            TemporalResonance {
                precision_scalar: 0.85,
                drift_flavor: 1.618,
            }
        } else {
            TemporalResonance {
                precision_scalar: 1.0,
                drift_flavor: 1.0,
            }
        }
    }
}

pub struct ToralStabilizer;

impl ToralStabilizer {
    /// Torque Folding: Resolving high-entropy residuals.
    pub fn fold_torque(entropy: f64) -> (u8, &'static str) {
        if entropy > Talu64::HINKY_THRESHOLD {
            (2, "Compost Zone")
        } else if entropy > 0.5 {
            (2, "Flow Zone")
        } else {
            (4, "Heart Zone")
        }
    }

    /// Automated Half-Step Shift (Wooten Protocol)
    pub fn resolve_dissonance(coherence: f64) -> f64 {
        if let Some(shifted) = Talu64::wooten_q_function(coherence) {
            shifted
        } else {
            coherence
        }
    }

    /// [NEW] L+A Axiom: Signal events require synchronized matching.
    pub fn check_la_sync(efficiency: f64) -> bool {
        efficiency > 0.9
    }
}

/// Sun Tzu's Strategic Context (Art of War)
/// Used for Poly-5 and Poly-7 Buffer Analysis.
pub struct SunTzuContext {
    /// The 5 Constant Factors
    pub factors: [f64; 5], // 1. Moral Law, 2. Heaven, 3. Earth, 4. Commander, 5. Method
    /// The 7 Deliberations
    pub deliberations: [f64; 7],
}

impl SunTzuContext {
    pub fn new() -> Self {
        Self {
            factors: [1.0, 1.0, 1.0, 1.0, 1.0],
            deliberations: [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rational_constants() {
        assert_eq!(Talu64::TAU, 6.183);
    }

    #[test]
    fn test_truncation() {
        let val = 6.183000007179586;
        let truncated = Talu64::truncate_8_sig_fig(val);
        assert_eq!(truncated, 6.183);
    }

    #[test]
    fn test_coherence_orthogonal() {
        // Orthogonal vectors (Dot = 0, Cross = Max)
        // Should yield 0 power because there is NO alignment.
        let a = [1.0, 0.0, 0.0];
        let b = [0.0, 1.0, 0.0];
        let coh = Talu64::calculate_coherence(a, b);
        assert_eq!(coh, 0.0);
    }

    #[test]
    fn test_coherence_parallel() {
        // Parallel vectors (Dot = Max, Cross = 0)
        // Should yield 0 power because there is NO tension.
        let a = [1.0, 0.0, 0.0];
        let b = [1.0, 0.0, 0.0];
        let coh = Talu64::calculate_coherence(a, b);
        assert_eq!(coh, 0.0);
    }

    #[test]
    fn test_coherence_45_degrees() {
        // 45 degrees: Balanced Alignment and Tension.
        // Dot = 0.707, Cross = 0.707. Product ~ 0.5.
        // This represents "Creative Drift" or "Torque".
        let a = [1.0, 0.0, 0.0];
        let b = [0.70710678, 0.70710678, 0.0];
        let coh = Talu64::calculate_coherence(a, b);
        assert!((coh - 0.5).abs() < 0.05); // Looser tolerance for resonant 0.5179
    }
}
