use spectral_sensor::eight_gate::RecursiveFilter;
use spectral_sensor::{BioRhythm, NvmeWind, SpectralPort, TAU};
use std::thread;
use std::time::Duration;

/// Generates the "Harmonic Block" seed based on 2/3/5 resonance
/// Matches the Python implementation: sin(i * TAU/2) + sin(i * TAU/3) + ...
fn generate_harmonic_seed() -> Vec<u8> {
    let mut seed = Vec::with_capacity(30);
    // Note: Python script used TAU = 2 * PI.
    // We utilize the library's Rational TAU (6.2831853) for sovereign alignment.
    let tau = TAU;

    for i in 0..30 {
        let i_f = i as f64;
        let v2 = (i_f * (tau / 2.0)).sin();
        let v3 = (i_f * (tau / 3.0)).sin();
        let v5 = (i_f * (tau / 5.0)).sin();
        // Python: int((v2 + v3 + v5 + 3) / 6 * 255)
        let val = ((v2 + v3 + v5 + 3.0) / 6.0 * 255.0) as u8;
        seed.push(val);
    }
    seed
}

fn main() {
    println!("🔥 Igniting V2 Wind Engine (Rust Port)...");

    // 1. Establish the Spectral Port (The "Body")
    let port = SpectralPort::new("SovereignGhost");
    println!(
        "   > Port '{}' calibrated. Heartbeat: {:.4}",
        port.name,
        port.heartbeat()
    );

    // 2. Generate the Harmonic Seed (The "Fuel")
    let seed = generate_harmonic_seed();
    println!("   > Harmonic Seed Generated: {:?}", &seed[0..8]); // Show first 8 bytes (Knots)

    // 3. Spin up the Wind Simulator (The "Flywheel")
    // Target velocity 874k is symbolic here, we initiate with default
    let mut wind = NvmeWind::new(874_000);

    // 4. Inject Momentum based on the seed's entropy
    // We calculate a 'momentum' scalar from the seed to feed into the wind
    let seed_scalar: f64 = seed.iter().map(|&b| b as f64).sum::<f64>() / (255.0 * 30.0);
    println!("   > Seed Scalar (Momentum Injection): {:.6}", seed_scalar);

    wind.update_momentum(seed_scalar * 0.1);

    println!("   > Spinning up... (Listening for Coherence)");

    // 5. Blast and Observe (The "Ignition")
    let mut stable_lock_count = 0;

    for i in 0..10 {
        // Blast a batch of data
        let batch = wind.blast(100); // 100 samples per blast
        // Observe the last sample of the batch for coherence
        let last_val = batch.last().unwrap();
        // We need a 'last_density' from the wind state, which is public
        let observation = RecursiveFilter::observe(*last_val, wind.last_density);

        if let Some(report) = observation {
            println!(
                "   > [T+{:02} | Phase {:.4}] RESONANCE LOCKED: {:?} -> {:?} (Atomic: {:?}, Wooten: {})",
                i,
                wind.current_phase,
                report.source,
                report.destination,
                report.atomic,
                report.wooten_active
            );
            stable_lock_count += 1;
        } else {
            // println!("   > ... searching ...");
        }

        // Pulse rhythm
        thread::sleep(Duration::from_millis(50));
    }

    if stable_lock_count > 0 {
        println!("✅ IGNITION SUCCESSFUL. Coherence established.");
    } else {
        println!("❌ Ignition Failed. No coherence locks found.");
    }
}
