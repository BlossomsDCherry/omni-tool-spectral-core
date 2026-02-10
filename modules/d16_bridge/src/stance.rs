use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Stance {
    Earth,
    Water,
    Fire,
    Wind,
    Void, // Groundwater
    Fritz,
    Stance, // Harmonic Unity
    Mode,   // High Freq Protocol
    MetaInterphase,
}

#[derive(Debug, Clone, Serialize)]
pub struct Gate {
    pub name: String,
    pub color: String, // Hex
    pub element: String,
    pub tau_degree: u8,
    pub bitmask: [u8; 3], // [Identity, Power, Logic]
    pub description: String,
}

impl Gate {
    pub fn trigram_match(&self, other_mask: [u8; 3]) -> TrigramResult {
        let matches = self
            .bitmask
            .iter()
            .zip(other_mask.iter())
            .filter(|(a, b)| a == b)
            .count();
        match matches {
            3 => TrigramResult::Resonance, // Perfect 3/3
            2 => TrigramResult::Drift,     // Tolerance 2/3
            _ => TrigramResult::Void,      // Noise <= 1
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TrigramResult {
    Resonance,
    Drift,
    Void, // 000 - The Aught / Physical Origin
}

impl Stance {
    pub fn is_aught(&self) -> bool {
        matches!(self, Stance::Void)
    }

    pub fn gate(&self) -> Gate {
        match self {
            Stance::Earth => Gate {
                name: "Earth".to_string(),
                color: "#FF0000".to_string(), // Red (IR)
                element: "Earth".to_string(),
                tau_degree: 1,
                bitmask: [0, 0, 0],
                description: "Structural Grounding".to_string(),
            },
            Stance::Water => Gate {
                name: "Water".to_string(),
                color: "#00FFFF".to_string(), // Cyan/Blue
                element: "Water".to_string(),
                tau_degree: 4,
                bitmask: [0, 0, 1],
                description: "Dynamic Flow".to_string(),
            },
            Stance::Fire => Gate {
                name: "Fire".to_string(),
                color: "#FFFF00".to_string(), // Yellow
                element: "Fire".to_string(),
                tau_degree: 2,
                bitmask: [0, 1, 0],
                description: "High-Velocity Actualization".to_string(),
            },
            Stance::Wind => Gate {
                name: "Wind".to_string(),
                color: "#00FF00".to_string(), // Green
                element: "Wind".to_string(),
                tau_degree: 3,
                bitmask: [0, 1, 1],
                description: "Creative Drift Resolution".to_string(),
            },
            Stance::Void => Gate {
                name: "Groundwater".to_string(), // Re-contextualized
                color: "#000000".to_string(),
                element: "Void".to_string(),
                tau_degree: 0,
                bitmask: [1, 0, 0],
                description: "Singularity Integration / Groundwater".to_string(),
            },
            Stance::Fritz => Gate {
                name: "Fritz".to_string(),
                color: "#FFFFFF".to_string(), // White
                element: "Superconductor".to_string(),
                tau_degree: 6, // Special
                bitmask: [1, 1, 1],
                description: "Superconduction (Zero Resistance)".to_string(),
            },
            Stance::Stance => Gate {
                name: "Stance".to_string(),
                color: "#FFFFFF".to_string(),
                element: "Unity".to_string(),
                tau_degree: 7,
                bitmask: [1, 1, 1],
                description: "Harmonic Unity".to_string(),
            },
            Stance::Mode => Gate {
                name: "Mode".to_string(),
                color: "#FF00FF".to_string(), // Violet
                element: "Protocol".to_string(),
                tau_degree: 5,
                bitmask: [1, 0, 1], // Direct/Invasion
                description: "High Frequency Protocol".to_string(),
            },
            Stance::MetaInterphase => Gate {
                name: "MetaInterphase".to_string(),
                color: "#880088".to_string(), // Hybrid
                element: "Interphase".to_string(),
                tau_degree: 8,
                bitmask: [1, 1, 0], // Indirect
                description: "Green-Violet Hybrid Interaction".to_string(),
            },
        }
    }
}
