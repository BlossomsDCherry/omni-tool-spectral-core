use std::process::Command;
use std::thread;
use std::time::Duration;

/// Chopper Agent: Diagnostics & Biology (System Health)
/// Role: Monitoring system logs and resource exhaustion.
struct Chopper {
    memory_threshold: f64,
}

impl Chopper {
    fn new() -> Self {
        println!("🦌 CHOPPER: I'm not a raccoon dog! Initializing Diagnostics...");
        Self {
            memory_threshold: 0.85, // 85% threshold
        }
    }

    fn diagnose_system(&self) {
        println!("🔭 [DIAGNOSIS] Scanning system vitals...");

        // Mocking log scan for kernel panics/errors
        let output = Command::new("dmesg")
            .args(["-l", "err,crit,alert,emerg"])
            .output()
            .ok();

        if let Some(out) = output {
            if !out.stdout.is_empty() {
                println!("🚨 [CHOPPER ALERT] Biological anomalies (Errors) detected in dmesg!");
            } else {
                println!("💖 [VITALS] No critical system errors found.");
            }
        }

        // Mocking memory check
        let load = 0.45; // Simulated load
        println!("💊 [RUMBLE BALL] Memory Load: {:.2}%", load * 100.0);

        if load > self.memory_threshold {
            println!("🧪 [EMERGENCY] Resource depletion detected! Applying medical bypass...");
        }
    }
}

fn main() {
    let doctor = Chopper::new();
    loop {
        doctor.diagnose_system();
        thread::sleep(Duration::from_secs(10));
    }
}
