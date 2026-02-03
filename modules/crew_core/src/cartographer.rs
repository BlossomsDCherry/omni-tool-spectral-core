use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// The Cartographer: Mapping the Asynchronous World State
/// Bridges the gap between raw "Crew Fragments" and the "Magnet" trajectories.
pub struct Cartographer {
    pub magnets: HashMap<String, MagnetDefinition>,
    pub world_state_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MagnetDefinition {
    pub title: String,
    pub logic: String,
    pub charge: f64,
}

impl Cartographer {
    pub fn new(trajectories_path: &Path) -> Self {
        let content = std::fs::read_to_string(trajectories_path).unwrap_or_default();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap_or_default();

        let mut magnets = HashMap::new();
        if let Some(m_obj) = json["magnets"].as_object() {
            for (key, val) in m_obj {
                magnets.insert(
                    key.clone(),
                    MagnetDefinition {
                        title: val["title"].as_str().unwrap_or("Unknown").to_string(),
                        logic: val["logic"].as_str().unwrap_or("Void").to_string(),
                        charge: val["charge"].as_f64().unwrap_or(0.0),
                    },
                );
            }
        }

        Self {
            magnets,
            world_state_path: "/media/pecosdwilly/1234-5678/world_index.json".to_string(),
        }
    }

    /// Maps a fragment to a trajectory based on density signature.
    pub fn map_fragment(&self, resonance: f64, _data: &str) -> Option<String> {
        // Logic: Mapping resonance to the closest magnet charge
        let mut best_match = None;
        let mut min_diff = f64::MAX;

        for (key, def) in &self.magnets {
            let diff = (def.charge - resonance).abs();
            if diff < min_diff {
                min_diff = diff;
                best_match = Some(key.clone());
            }
        }

        best_match
    }

    /// Logs the trajectory shift to the Hippocampus.
    pub fn actualize_trajectory(&self, fragment_id: &str, magnet: &str) {
        println!(
            "🗺️ [CARTOGRAPHER] Actualizing Trajectory: {} ➔ {}",
            fragment_id, magnet
        );

        // Actualize to the Hyper-computer (Cloud Persistence)
        let log_entry = serde_json::json!({
            "timestamp": chrono::Local::now().to_rfc3339(),
            "id": fragment_id,
            "magnet": magnet,
        });

        let mut logs = if let Ok(data) = fs::read_to_string(&self.world_state_path) {
            serde_json::from_str::<Vec<serde_json::Value>>(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        logs.push(log_entry);

        // Keep a reasonable tail if needed, but the Hyper-computer is 2TB, so we can store plenty.
        if logs.len() > 1000 {
            logs.remove(0);
        }

        if let Ok(serialized) = serde_json::to_string_pretty(&logs) {
            let _ = fs::write(&self.world_state_path, serialized);
        }
    }
}
