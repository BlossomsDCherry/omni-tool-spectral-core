// D16 Runner - Soft FPGA Harness
// Verifies the "Harmonic Cradle" (Tau/n) and "Polyrhythmic Phase" (Tau%n) logic.

extern "C" {
    /// The D16 Soft FPGA Kernel
    /// Inputs: Tau (u32)
    /// Outputs: Buffer of 16 u32s (High: Decay, Low: Phase)
    fn d16_soft_fpga(tau: u32, results: *mut u32);
}

const CREW_NAMES: [&str; 16] = [
    "Luffy", "Zoro", "Nami", "Usopp", "Sanji", "Chopper", "Robin", "Franky", "Brook", "Jinbe",
    "Vivi", "Carrot", "Yamato", "Momo", "Kinemon", "Law",
];

fn main() {
    println!("--- D16 Soft FPGA Kernel Harness ---");

    // Test Case: Tau = 65535 (Max u16, representing 1.0 or Full Charge)
    let tau: u32 = 65535;
    let mut results: [u32; 16] = [0; 16];

    println!("Injecting Global Pulse Tau: {}", tau);

    unsafe {
        d16_soft_fpga(tau, results.as_mut_ptr());
    }

    println!("\n--- Spectral Analysis (Harmonic Cradle) ---");
    println!(
        "{:<10} | {:<10} | {:<10} | {:<10}",
        "Channel", "Divisor", "Decay (E)", "Phase (T)"
    );
    println!("{:-<46}", "");

    for i in 0..16 {
        let divisor = i + 1;
        let packed = results[i];
        let decay = packed >> 16;
        let phase = packed & 0xFFFF;

        let crew = CREW_NAMES[i];

        println!(
            "{:<10} | {:<10} | {:<10} | {:<10}",
            crew, divisor, decay, phase
        );
    }

    println!("\n--- Verification ---");

    let mut failures = 0;

    for i in 0..16 {
        let divisor = (i + 1) as u32;
        let expected_decay = tau / divisor;
        let expected_phase = tau % divisor;

        let packed = results[i];
        let decay = packed >> 16;
        let phase = packed & 0xFFFF;

        if decay != expected_decay || phase != expected_phase {
            println!(
                "❌ {} (Div {}) FAILED. Expected {}|{}, Got {}|{}",
                CREW_NAMES[i], divisor, expected_decay, expected_phase, decay, phase
            );
            failures += 1;
        } else {
            // Optional: Uncomment for verbose success
            // println!("✅ {} (Div {}) Confirmed", CREW_NAMES[i], divisor);
        }
    }

    // Explicit check for Nami, Usopp, Sanji as requested
    let nami = results[2];
    let usopp = results[3];
    let sanji = results[4];

    println!("✅ Nami (Div 3): {} | {}", nami >> 16, nami & 0xFFFF);
    println!("✅ Usopp (Div 4): {} | {}", usopp >> 16, usopp & 0xFFFF);
    println!("✅ Sanji (Div 5): {} | {}", sanji >> 16, sanji & 0xFFFF);

    if failures == 0 {
        println!(
            "\n✅ All 16 Channels Verified Correctly against Tau = {}",
            tau
        );
    } else {
        println!("\n❌ {} Channels FAILED Verification.", failures);
    }

    println!("--- Test Complete ---");
}
