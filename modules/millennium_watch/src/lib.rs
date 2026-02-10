pub mod agents;
use agents::{
    BSDAgent, HodgeAgent, MillenniumAgent, NavierStokesAgent, PoincareAgent, PvsNPAgent,
    RiemannAgent, YangMillsAgent,
};
use z_rr::Talu64; // Import from dependency

/// Voxel Synth: Dimensional Comet Toss Calculator
/// "The Geometry of Resonance"

pub fn audit_talu64(talu: &Talu64) {
    println!("\n--- Millennium Watch: 7-Agent Scan ---");
    let agents: Vec<Box<dyn MillenniumAgent>> = vec![
        Box::new(PvsNPAgent),
        Box::new(HodgeAgent),
        Box::new(RiemannAgent),
        Box::new(YangMillsAgent),
        Box::new(NavierStokesAgent),
        Box::new(BSDAgent),
        Box::new(PoincareAgent),
    ];

    println!("Scanning Talu64 State...");

    for agent in agents {
        if let Some(insight) = agent.scan(talu, 50.0) {
            println!("   🚨 [{}]: {}", agent.name(), insight);
        } else {
            // println!("   . [{}]: No anomalies.", agent.name());
        }
    }
}

fn analyze_topology(name: &str, divisors: &[u32], domain: u32, threshold: usize) {
    let mut points = Vec::new();

    // Include 0 as the Singularity/Origin
    for i in 0..=domain {
        if i == 0 {
            points.push(0);
            continue;
        }

        let energy: usize = divisors
            .iter()
            .map(|&d| if i % d == 0 { 1 } else { 0 })
            .sum();
        if energy >= threshold {
            points.push(i);
        }
    }

    println!("Topology: {}", name);
    println!("   Harmonics: {:?}", divisors);
    println!("   Energy Threshold: {} (Intersection Depth)", threshold);
    println!("   Resulting Voxel Size: {} points", points.len());

    // Density Analysis
    let density = (points.len() as f64 / domain as f64) * 100.0;
    println!(
        "   Space Reduction: {:.2}% of original cube Transmuted.",
        100.0 - density
    );
    println!("--------------------------------------------------");
}
