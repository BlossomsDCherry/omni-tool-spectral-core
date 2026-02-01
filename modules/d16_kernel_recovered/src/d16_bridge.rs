extern "C" {
    /// The D16 Soft FPGA Kernel (Harmonic Cradle)
    ///
    /// # Arguments
    /// * `tau` - Global Clock Pulse (u32)
    /// * `results` - Ptr to buffer for 16 u32 results (High: Decay, Low: Phase)
    pub fn d16_soft_fpga(tau: u32, results: *mut u32);
}

#[derive(Debug, Clone, Copy)]
pub struct CrewmateSignal {
    pub name: &'static str,
    pub decay_energy: u16, // Tau / n
    pub phase_gear: u16,   // Tau % n
}

pub struct D16Spectrum {
    pub signals: [CrewmateSignal; 16],
}

/// The Crewmate Channel Map
const CREW_NAMES: [&str; 16] = [
    "Luffy", "Zoro", "Nami", "Usopp", "Sanji", "Chopper", "Robin", "Franky", "Brook", "Jinbe",
    "Vivi", "Carrot", "Yamato", "Momo", "Kinemon", "Law",
];

/// Process a single Global Moment (Tau) through the 16-channel Soft FPGA
pub fn process_soft_fpga(tau: u32) -> D16Spectrum {
    let mut raw_buffer: [u32; 16] = [0; 16];

    unsafe {
        // CALL THE ASSEMBLY KERNEL
        d16_soft_fpga(tau, raw_buffer.as_mut_ptr());
    }

    // Unpack the packed u32s back into Signals
    let mut signals = [CrewmateSignal {
        name: "",
        decay_energy: 0,
        phase_gear: 0,
    }; 16];

    for i in 0..16 {
        let packed = raw_buffer[i];
        signals[i] = CrewmateSignal {
            name: CREW_NAMES[i],
            decay_energy: (packed >> 16) as u16,
            phase_gear: (packed & 0xFFFF) as u16,
        };
    }

    D16Spectrum { signals }
}
