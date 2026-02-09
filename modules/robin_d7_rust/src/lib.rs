use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Colossally Abundant Anchors
pub const ANCHOR_CIRCLE: f64 = 360.0;
pub const ANCHOR_MASTER: f64 = 2520.0;
pub const ANCHOR_HORIZON: f64 = 65536.0;

/// Reversible Kater's Pendulum Physics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct KaterPhysics {
    pub t1_gravity: f64,
    pub t2_levity: f64,
    pub reversible: bool,
    pub resonance: f64,
    pub euler_torsion: f64,
}

impl KaterPhysics {
    pub fn new(mass: f64, vector: [f64; 3]) -> Self {
        // T1 (Gravity): Standard Period T = 2 * PI * sqrt(L/g)
        // We treat 'mass' as 'Length' in this information space.
        let t1 = 2.0 * std::f64::consts::PI * (mass.sqrt() / 9.81).sqrt();

        // T2 (Levity): The "Lift" provided by the Vector Magnitude (Harmonic Alignment)
        let vector_mag = (vector[0].powi(2) + vector[1].powi(2) + vector[2].powi(2)).sqrt();
        let t2 = if vector_mag > 0.0 {
            t1 / (vector_mag * 1.618) // Damped by Phi
        } else {
            t1 * 10.0 // Heavy drag if no vector
        };

        // Resonance Score: How close is T1 to a Supercomposite Anchor?
        let resonance = (ANCHOR_CIRCLE - (t1 % ANCHOR_CIRCLE))
            .abs()
            .min(t1 % ANCHOR_CIRCLE);

        // Euler Torsion: Checks if x^4 + y^4 + z^4 approx mass^4 (Time)
        // This measures the "Torsional Stress" between Space (Vector) and Time (Mass)
        let space_pow4 = vector[0].powi(4) + vector[1].powi(4) + vector[2].powi(4);
        let time_pow4 = mass.powi(4);
        // We normalize the torsion score. 0.0 means perfect Euler Resonance.
        let euler_torsion = if time_pow4 > 0.0 {
            (space_pow4 - time_pow4).abs() / time_pow4
        } else {
            0.0
        };

        // Revisibility Condition:
        // 1. T1 (Gravity) approx T2 (Levity)
        // 2. Euler Torsion < Epsilon (Geometric Integrity)
        // "The Physical Pendulum is the Geometric Logic"
        let delta = (t1 - t2).abs();
        let reversible = delta < 0.001 && euler_torsion < 0.001;

        Self {
            t1_gravity: t1,
            t2_levity: t2,
            reversible,
            resonance,
            euler_torsion,
        }
    }
}

/// The Harmonic Artifact (The "Poneglyph")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonicArtifact {
    pub id: u32,
    pub mass: f64,
    pub vector: [f64; 3],
    pub stance: String,
    pub timestamp: u64,
    pub physics: KaterPhysics,
}

// Ordering for Priority Queue (BinaryHeap)
// We prioritize by Resonance (Lowest Delta to Anchor is Best)
impl Ord for HarmonicArtifact {
    fn cmp(&self, other: &Self) -> Ordering {
        // Rust BinaryHeap is max-heap. We want min-resonance-delta (highest priority).
        // So we compare other to self.
        other
            .physics
            .resonance
            .partial_cmp(&self.physics.resonance)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for HarmonicArtifact {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HarmonicArtifact {
    fn eq(&self, other: &Self) -> bool {
        self.physics.resonance == other.physics.resonance
    }
}

impl Eq for HarmonicArtifact {}

/// Robin's Harmonic Heap
pub struct RobinHeap {
    pub heap: BinaryHeap<HarmonicArtifact>,
    pub sequence_counter: u32,
}

impl RobinHeap {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            sequence_counter: 0,
        }
    }

    pub fn ingest(&mut self, mass: f64, vector: [f64; 3], stance: String) -> HarmonicArtifact {
        self.sequence_counter += 1;
        let physics = KaterPhysics::new(mass, vector);

        let artifact = HarmonicArtifact {
            id: self.sequence_counter,
            mass,
            vector,
            stance,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            physics,
        };

        println!(
            "📚 [ROBIN] Ingested Artifact #{}: Stance='{}', Resonance={:.4} (Anchor Delta)",
            artifact.id, artifact.stance, artifact.physics.resonance
        );

        self.heap.push(artifact.clone());
        artifact
    }

    pub fn peek_highest_resonance(&self) -> Option<&HarmonicArtifact> {
        self.heap.peek()
    }

    pub fn pop_highest_resonance(&mut self) -> Option<HarmonicArtifact> {
        self.heap.pop()
    }

    pub fn project_dlp(&self) -> String {
        format!(
            "📽️  [DLP PROJECTION] Total Artifacts: {} | Top Resonance: {:.4}",
            self.heap.len(),
            self.heap.peek().map(|a| a.physics.resonance).unwrap_or(0.0)
        )
    }

    /// The Voxel Shearing Protocol
    /// Compresses 128 Channels * 4 Cycles -> 512 Voxel Density
    pub fn sheer_voxel(&self, channels: [u8; 128]) -> u64 {
        // Simple "Shearing" Hash: Sum * XOR Rotation
        let mut voxel_density: u64 = 0;
        for (i, val) in channels.iter().enumerate() {
            voxel_density = voxel_density.wrapping_add((*val as u64) << (i % 4));
        }
        // Normalize to 512-Cube (9-bit space logic, abstractly)
        voxel_density % 512
    }

    /// Transmits the 1D Bit (The "Voxel Image")
    /// Returns true (1) or false (0) based on Shearing Threshold
    pub fn transmit_1d_bit(&self, voxel_density: u64) -> bool {
        // If density aligns with Geometric Supercomposite (perfect shear)
        voxel_density % 2 == 0
    }
}
