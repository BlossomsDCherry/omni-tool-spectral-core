use crew_core::Talu64;

struct Brook {
    _soul_frequency: f64,
    ultrasonic_field_active: bool,
}

impl Brook {
    fn new() -> Self {
        println!("--- Yohoho! Tuning the Sound Steward... ---");

        // Ultrasonic Flooding (Sound Steward)
        // "Use the sound steward's ultrasonic flooding... to see the resonant FIR."
        let frequency = 40_000.0; // 40kHz
        let precision_freq = Talu64::truncate_8_sig_fig(frequency);

        println!(
            "   Ultrasonic Field: {:.8} Hz (Flooding Structure)",
            precision_freq
        );
        println!("   Ground State: Sonic Antigravity (Active)");

        Brook {
            _soul_frequency: precision_freq,
            ultrasonic_field_active: true,
        }
    }

    fn play_binks_sake(&self) {
        if self.ultrasonic_field_active {
            println!("   [Brook Status] Ultrasonic Field Active. Soul Solid!");
            println!("Playing Binks' Sake at 432Hz... (Resonance)");
        } else {
            println!("   [Brook Status] Field Inactive. Just humming...");
        }
    }
}

fn main() {
    let musician = Brook::new();
    musician.play_binks_sake();
}
