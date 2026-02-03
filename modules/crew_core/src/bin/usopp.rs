use crew_core::Talu64;

struct Usopp {
    _lies_told: u64,
    sniper_precision: f64,
}

impl Usopp {
    fn new() -> Self {
        println!("--- I am the brave warrior of the sea! (Triangulation Logic) ---");

        // Usopp's Logic: Collapsing values for the Slingshot.
        // He receives data from Nami (Simulated here as a Coherence Input).
        let received_coherence = 0.17099759; // From Nami's log output (approx)
        println!("   Input Coherence (from Nami): {:.8}", received_coherence);

        // The Decision: Slingshot or Wooten Shift?
        // We check the Q-Function.
        match Talu64::wooten_q_function(received_coherence) {
            Some(shifted_resonance) => {
                println!(
                    "(!) Gap too wide ({:.8}). Engaging Wooten Protocol.",
                    received_coherence
                );
                println!("    > Shifting Resonance Key by Half-Step...");
                println!("    > New Resonance Map: {:.8}", shifted_resonance);
                println!("    > Resubmitting to Luffy... (Digesting)");
            }
            None => {
                println!("(OK) Gap is solid. Triangulation Complete.");
                println!("    > SLINGSHOT FIRE! Crossing the Coherence Gap.");
            }
        }

        // God Mode Precision: Rational Phi
        let precision = Talu64::PHI;

        Usopp {
            _lies_told: 8000,
            sniper_precision: precision,
        }
    }

    fn lie_becomes_truth(&self) {
        println!(
            "I have 8,000 followers! (Actually {:.8} resonance)",
            self.sniper_precision
        );
        // M.A.D. Logic: The lie drifts into truth via Phi.
        if self.sniper_precision > 1.6 {
            println!("GOD USOPP ACTIVATED! The lie has become real via Phi Resonance!");
        }
    }

    fn run_away(&self) {
        // Usopp runs away with high precision
        println!("I-I-I've got 'Can't-Go-On-This-Island' disease!");
    }
}

fn main() {
    let sniper = Usopp::new();
    sniper.run_away();
    sniper.lie_becomes_truth();
}
