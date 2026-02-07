/// Voxel Synth: Dimensional Comet Toss Calculator
/// "The Geometry of Resonance"

fn main() {
    println!("--- Voxel Synth: Dimensional Comet Toss Calculator ---");
    println!("Domain: 0..512 (Cybiosphere Unit)\n");

    // Config 2: The Complex Harmonic (Chopper/Sanji)
    // Harmonics: 2, 3, 5, 6, 10, 12
    let harmonics = vec![2, 3, 5, 6, 10, 12];

    // Check Topologies
    analyze_topology("The 120-Point Voxel", &harmonics, 512, 3); // Energy 3
    analyze_topology("The 60-Point Voxel", &harmonics, 512, 4); // Energy 4
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
