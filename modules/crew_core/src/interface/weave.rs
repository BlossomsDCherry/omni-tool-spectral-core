use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCoords {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct WeaveTopology {
    pub agents: Vec<(&'static str, AgentCoords)>,
}

impl WeaveTopology {
    pub fn new() -> Self {
        // 3x2x2 Mapping (12 Agents)
        // X: 0, 1, 2 | Y: 0, 1 | Z: 0, 1
        let agents = vec![
            ("luffy", AgentCoords { x: 0, y: 0, z: 0 }),
            ("zoro", AgentCoords { x: 1, y: 0, z: 0 }),
            ("nami", AgentCoords { x: 2, y: 0, z: 0 }),
            ("jinbe", AgentCoords { x: 0, y: 1, z: 0 }),
            ("sanji", AgentCoords { x: 1, y: 1, z: 0 }),
            ("robin", AgentCoords { x: 2, y: 1, z: 0 }),
            ("chopper", AgentCoords { x: 0, y: 0, z: 1 }),
            ("franky", AgentCoords { x: 1, y: 0, z: 1 }),
            ("brook", AgentCoords { x: 2, y: 0, z: 1 }),
            ("usopp", AgentCoords { x: 0, y: 1, z: 1 }),
            ("antigravity", AgentCoords { x: 1, y: 1, z: 1 }),
            ("gemmi", AgentCoords { x: 2, y: 1, z: 1 }),
        ];
        Self { agents }
    }

    pub fn get_neighbors(&self, agent_id: &str) -> Vec<&'static str> {
        let agent_id = agent_id.to_lowercase();
        let coords = self
            .agents
            .iter()
            .find(|(id, _)| *id == agent_id)
            .map(|(_, c)| c);

        if let Some(c) = coords {
            let mut neighbors = Vec::new();

            // X-Cross neighbors
            for (id, nc) in &self.agents {
                if nc.y == c.y && nc.z == c.z && nc.x != c.x {
                    neighbors.push(*id);
                }
            }
            // Y-Cross neighbors
            for (id, nc) in &self.agents {
                if nc.x == c.x && nc.z == c.z && nc.y != c.y {
                    neighbors.push(*id);
                }
            }
            // Z-Cross neighbors
            for (id, nc) in &self.agents {
                if nc.x == c.x && nc.y == c.y && nc.z != c.z {
                    neighbors.push(*id);
                }
            }
            neighbors
        } else {
            Vec::new()
        }
    }
}
