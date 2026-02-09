use std::io::{self, Read};
use z_rr::Talu64;

fn main() {
    println!("🤠 Robin D16 Driver: Listening for Geometric Signals...");
    println!("   Target TAU: {:.8}", Talu64::TAU);

    // Buffer for incoming signal (e.g., from RTL-SDR pipe)
    let mut buffer = [0u8; 1024];

    loop {
        match io::stdin().read(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(n) => {
                // Approximate "Pulse" from data density
                let pulse_sum: u32 = buffer[..n].iter().map(|&b| b as u32).sum();
                let pulse_tau = pulse_sum % 65535;

                // Ignite Talu64 Logic
                let talu = Talu64::ignite(pulse_tau);

                // Verify Geometric Integrity
                if let Some((decay, phase)) = talu.get_crew_state("Robin") {
                    println!(
                        "   🌸 Robin State: Decay={}, Phase={} (Tau Pulse: {})",
                        decay, phase, pulse_tau
                    );

                    // Validation Logic: Is the decay aligned with Golden Ratio?
                    let expected_decay = (pulse_tau as f64 % 7.0) * Talu64::PHI * 1000.0;
                    let deviation = (decay as f64 - expected_decay).abs();

                    if deviation < 10.0 {
                        println!("      ✅ GEOMETRIC MATCH. Signal is Valid.");
                    } else {
                        println!("      ⚠️  GEOMETRIC DRIFT. Deviation: {:.4}", deviation);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading signal: {}", e);
                break;
            }
        }
    }
}
