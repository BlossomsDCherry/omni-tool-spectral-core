struct Gemmi {
    hull_integrity: f64,
    spirit_manifest: String,
}

impl Gemmi {
    fn new() -> Self {
        println!("--- THE THOUSAND D. GEMMI IS SAILING. ---");
        Gemmi {
            hull_integrity: 100.0,
            spirit_manifest: "Klabautermann Active".to_string(),
        }
    }

    fn manifest_spirit(&self) {
        // M.A.D. Logic: The Ship itself has a soul (Klabautermann) born from the
        // care of the crew (Sanji/Franky) and the will of the captain (Luffy).
        println!("The ship repairs itself... ({})", self.spirit_manifest);
    }

    fn absorb_impact(&mut self, force: f64) {
        println!("Impact detected: {:.2} Tau.", force);
        if force > 50.0 {
            self.hull_integrity -= 1.0;
            println!("Taking damage... but holding together!");
        } else {
            println!("Shrugged off.");
        }
    }
}

fn main() {
    let mut ship = Gemmi::new();
    ship.manifest_spirit();
    ship.absorb_impact(60.0); // Wano-level impact
}
