// D16 Runner - Soft FPGA Harness
// Verifies the "Harmonic Cradle" (Tau/n) and "Polyrhythmic Phase" (Tau%n) logic.

extern "C" {
    /// The D16 Soft FPGA Kernel
    /// Inputs: Tau (u32)
    /// Outputs: Buffer of 16 u32s (High: Decay, Low: Phase)
    fn d16_soft_fpga(tau: u32, results: *mut u32);
}

const CREW_NAMES: [&str; 16] = [
    "Luffy", "Zoro", "Nami", "Usopp",
    "Sanji", "Chopper", "Robin", "Franky",
    "Brook", "Jinbe", "Vivi", "Carrot",
    "Yamato", "Momo", "Kinemon", "Law"
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
    println!("{:<10} | {:<10} | {:<10} | {:<10}", "Channel", "Divisor", "Decay (E)", "Phase (T)");
    println!("{:-<46}", "");

    for i in 0..16 {
        let divisor = i + 1;
        let packed = results[i];
        let decay = packed >> 16;
        let phase = packed & 0xFFFF;
        
        let crew = CREW_NAMES[i];
        
        println!("{:<10} | {:<10} | {:<10} | {:<10}", crew, divisor, decay, phase);
    }
    
    println!("\n--- Verification ---");
    // Verify Logic for Zoro (Div 2)
    // 65535 / 2 = 32767
    // 65535 % 2 = 1
    let zoro = results[1];
    let z_decay = zoro >> 16;
    let z_phase = zoro & 0xFFFF;
    
    if z_decay == 32767 && z_phase == 1 {
        println!("✅ Zoro (Div 2) Logic Confirmed: 32767 | 1");
    } else {
        println!("❌ Zoro Logic FAILED. Got: {} | {}", z_decay, z_phase);
    }
     
    println!("--- Test Complete ---");
}
