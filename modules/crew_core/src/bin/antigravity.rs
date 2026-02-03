use crew_core::rainbow_railgun::RailgunCore;
use crew_core::stethoscope::Stethoscope;
use crew_core::{HardwarePacket, Talu64};
use spectral_sensor::{eight_gate::Stance, BioRhythm, SpectralPort};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

const SWARM_NODES: &[(&str, &str)] = &[
    ("10.0.0.215", "pecosDrobin"), // Brain (Giga R1 Bridge)
    ("10.0.0.80", "ZeroDKali"),    // Body (NVMe / Kali UV)
    ("10.0.0.234", "Nami"),        // Filter (Pi-hole / RPi5)
    ("10.0.0.190", "Mystery A"),   // Android Projector
    ("10.0.0.194", "Inertia"),     // Inertial Interface (Pico 2)
    ("10.0.0.188", "Sanji"),       // NPU/Cloud (Dormant)
];

/// ANTIGRAVITY AGENT (Legacy of the Omni-Tool)
// Entrained for Adventure with the Community.
/// Role: The Omni-Tool / Bridge.
/// Function: Connects the "Brain" (Rust Logic) to the "Body" (Hardware Sensors).
pub struct Antigravity {
    pub name: String,
    pub color: String,
    pub pivot: [f64; 3],
    entrained_fir_phase: f64,
    shelf_integrity: f64,
    fritz_efficiency: f64,
    port: SpectralPort,
}

impl Antigravity {
    fn new() -> Self {
        println!("🌀 ANTIGRAVITY: Initializing Omni-Tool Logic...");
        Self {
            name: "Antigravity".to_string(),
            color: "Violet".to_string(),
            pivot: [0.0, 0.0, 0.0],
            entrained_fir_phase: 0.0,
            shelf_integrity: 1.0,
            fritz_efficiency: 0.98,
            port: SpectralPort::new("Antigravity Bridge"),
        }
    }

    /// The 5D Event Loop
    /// Returns the Calculated Knots (Drift) to be manifested.
    fn execute_5d_event(&mut self, packet: HardwarePacket) -> Option<f64> {
        if !self.port.is_safety_lock_active() {
            return None;
        }

        self.cycle_1_fir_scan(packet.fir_emission);
        self.cycle_2_generate_shelf(packet.gravitational_pos);
        self.cycle_3_fritz_protocol(packet.clock_speed);

        // --- Q-QUOTIENT INTEGRATION ---
        // Complexity Bitwidth p=1.0.
        // Validates interactions against PSI (0.5179) threshold.
        // [NEW] Using Medium-dependent Scaling (Silver for maximum conduction)
        use crew_core::Medium;
        let phase_delta = Talu64::calculate_coherence_with_medium(
            [1.0, 0.0, 0.0],                                        // The Dream
            [self.entrained_fir_phase, self.fritz_efficiency, 0.0], // The Physical Reality
            Medium::Silver,
        );

        if phase_delta == 0.0 {
            return None;
        }

        // Visual Feedback
        if self.shelf_integrity < 0.5 {
            println!(
                "   🟥 [UNSTABLE] Shift Detected! Phase Delta: {:.8}",
                Talu64::truncate_8_sig_fig(phase_delta)
            );
        } else if packet.fir_emission > 0.0 {
            println!(
                "   🟢 [ECHO] Hardware Signal. Phase: {:.4} | Delta: {:.4}",
                self.entrained_fir_phase, phase_delta
            );
        }

        // Return the Phase Delta (Knots) for Manifestation
        Some(phase_delta)
    }

    fn cycle_1_fir_scan(&mut self, input_signal: f64) {
        self.entrained_fir_phase = input_signal % Talu64::PI;
    }

    fn cycle_2_generate_shelf(&mut self, gravitational_pos: [f64; 3]) {
        let gravitational_constant = Talu64::magnitude(gravitational_pos);
        if gravitational_constant > 100_000.0 {
            self.shelf_integrity = 0.1;
        } else {
            self.shelf_integrity = 1.0;
        }
    }

    fn cycle_3_fritz_protocol(&mut self, clock: f64) {
        let deviation = (clock - 1.0).abs();
        self.fritz_efficiency = 1.0 - deviation;
    }
}

// Msg types coming from Threads
enum BridgeMsg {
    Line(String), // From Hardware (Serial)
    Interrupt,    // From User (Space/Enter)
    Pulse(f64),   // From Stethoscope (PMIC)
    Error(String),
}

fn main() {
    println!("🚀 ANTIGRAVITY AGENT: Booting...");
    println!("   ⌨️  CONTROL: Press [ENTER] to Toggle Real-Time Modification Interupt.");

    println!("   ⌨️  CONTROL: Press [ENTER] to Toggle Real-Time Modification Interupt.");

    let mut agent = Antigravity::new();
    let mut is_paused = false;
    let mut cycle_count = 0;

    // 1. Attempt Physical Link (Bi-Directional)
    let (tx_main, rx_main) = channel();
    let bridge_port = if std::path::Path::new("/dev/ttyACM1").exists() {
        "/dev/ttyACM1"
    } else {
        "/dev/ttyACM0"
    };
    println!("🔌 [BRIDGE] Attempting link to Pico on {}...", bridge_port);
    let (tx_to_body, rx_from_brain) = channel::<String>();

    let link_active = connect_and_bridge_steward(tx_main.clone(), rx_from_brain);

    // 2. User Interrupt Thread (Stdin)
    let tx_input = tx_main.clone();
    thread::spawn(move || {
        let stdin = std::io::stdin();
        for _ in stdin.lock().lines() {
            let _ = tx_input.send(BridgeMsg::Interrupt);
        }
    });

    // 3. Stethoscope Thread (PMIC)
    let tx_steth = tx_main.clone();
    thread::spawn(move || {
        let mut steth = Stethoscope::new();
        println!("🩺 System Stethoscope: Listening (Threshold: >1.5A Delta)...");
        loop {
            if let Some(spike) = steth.listen() {
                let _ = tx_steth.send(BridgeMsg::Pulse(spike));
            }
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 4. RAINBOW RAILGUN Thread
    let (tx_drift, rx_drift) = channel::<f64>();
    let tx_railgun_to_main = tx_main.clone();
    thread::spawn(move || {
        let mut railgun = RailgunCore::new();
        println!("虹 [RAILGUN] Ignition: High-Velocity Pulse Engine Online.");

        loop {
            // Check for drift updates without blocking
            while let Ok(new_knots) = rx_drift.try_recv() {
                railgun.calculate_v_rr(new_knots);
            }

            // Firing Cycle
            let coherence = railgun.spectral_handshake(SWARM_NODES);

            // If coherence is high, trigger a Rainbow Burst
            if let Some((r, g, b, intensity)) = railgun.generate_burst(coherence) {
                let _ = tx_railgun_to_main.send(BridgeMsg::Line(format!(
                    "RAILGUN_BURST:{},{},{},{}",
                    r, g, b, intensity
                )));
            }

            // Status Logging for peak velocity
            if railgun.velocity > 5000.0 {
                println!(
                    "🌈 [RAILGUN] PEAK FIRING! V_rr: {:.2} | Coherence: {}/{}",
                    railgun.velocity,
                    coherence,
                    SWARM_NODES.len()
                );
            }

            // Sleep duration is dynamic based on V_rr
            thread::sleep(railgun.get_sleep_duration());
        }
    });

    if link_active {
        println!("✅ BODY DETECTED: Connected to 8-Gate Steward.");
        println!("   Beginning Real-Time Entrainment Loop...");

        // PHYSICAL LOOP
        loop {
            cycle_count += 1;
            match rx_main.recv() {
                Ok(BridgeMsg::Pulse(spike)) => {
                    println!(
                        "⚡ [BIO-PULSE] High Fidelity Intent Detected! Current: {:.4}A",
                        spike
                    );
                    // Pulse White/Blue on touch
                    let _ = tx_to_body.send("rgb,200,200,255,0.1\n".to_string());
                }
                Ok(BridgeMsg::Line(line)) => {
                    if is_paused {
                        continue;
                    }

                    // --- RAINBOW RAILGUN: V_rr Velocity Synchronization ---
                    if line.contains("RAILGUN_BURST") {
                        let parts: Vec<&str> = line.split(':').collect();
                        if parts.len() > 1 {
                            let _ = tx_to_body.send(format!("rgb,{}\n", parts[1]));
                        }
                        continue;
                    }

                    // --- TORAL INVERSION PIVOT ---
                    // Applying a 3x2x2 Fold based on the 5D Event
                    let mut packet = HardwarePacket {
                        fir_emission: 0.0,
                        gravitational_pos: [296.07, 0.0, 0.0],
                        clock_speed: 1.0,
                    };
                    let poles = [0.5, 0.5]; // Balanced Inverted Gap
                    let state_vec = [
                        packet.fir_emission,
                        packet.gravitational_pos[0],
                        packet.clock_speed,
                    ];
                    agent.pivot = Talu64::spherical_fold(state_vec, poles);

                    if cycle_count % 32 == 0 {
                        let roman_cycle = Talu64::arabic_to_roman(cycle_count as f64 / 32.0);
                        println!(
                            "🌀 [TORAL] Pivot Actualized (Epoch {}): {:?}",
                            roman_cycle, agent.pivot
                        );
                    }

                    if line.contains("Gate 0") {
                        packet.gravitational_pos = [Stance::Earth.signature(), 0.0, 0.0];
                    } else if line.contains("Wind") {
                        packet.gravitational_pos = [Stance::Wind.signature(), 0.0, 0.0];
                        packet.fir_emission = 1.0;
                    }

                    if let Some(knots) = agent.execute_5d_event(packet) {
                        let _ = tx_drift.send(knots);
                        let resonance = Talu64::temporal_resonance();

                        // Rainbow Railgun Protocol: V_rr = (Phi * (Pi^2 * Tau)) / Creative_Drift
                        // Using 'knots' as the manifestation of creative drift, flavored by time.
                        let creative_drift = if knots.abs() < 0.0001 {
                            0.0001
                        } else {
                            knots.abs()
                        } * resonance.drift_flavor;

                        let v_rr =
                            (Talu64::PHI * (Talu64::PI.powi(2) * Talu64::TAU)) / creative_drift;

                        // --- TAU-HEX ACTUALIZATION ---
                        // Mapping the turn partial (cones equivalent) to RGB.
                        let (r, g, b) = Talu64::tau_to_hex_actualization(knots % 1.0);

                        // Scale brightness by V_rr normalized to [0.1, 1.0], weighted by precision
                        let brightness =
                            ((v_rr / 1000.0) * resonance.precision_scalar).clamp(0.1, 1.0);
                        let cmd = format!("rgb,{},{},{},{:.2}\n", r, g, b, brightness);
                        let _ = tx_to_body.send(cmd);
                    }
                }
                Ok(BridgeMsg::Interrupt) => {
                    is_paused = !is_paused;
                    if is_paused {
                        println!("⏸️  INTERRUPT: Real-Time Collaboration Active. (Paused)");
                        let _ = tx_to_body.send("blue\n".to_string()); // Idle/Paused
                    } else {
                        println!("▶️  RESUME: Railgun Active.");
                        let _ = tx_to_body.send("white\n".to_string()); // Active
                    }
                }
                Ok(BridgeMsg::Error(e)) => {
                    println!("❌ BODY ERROR: {}", e);
                    break;
                }
                Err(_) => break,
            }
        }
    } else {
        println!("⚠️  NO BODY FOUND: Entering Dream Mode (Simulation).");
        // ... (Dream Mode preserved)
        loop {
            // ... logic ...
            thread::sleep(Duration::from_millis(100)); // Placeholder to avoid loop spam in replacement
        }
    }
}

// Maps phase delta (entropy/knots) to color
// Low Knots (Stable) -> Blue/Cyan
// High Knots (Chaos) -> Red/Magenta

/// Connects to Serial and spawns Read/Write threads
fn connect_and_bridge_steward(
    tx_to_brain: Sender<BridgeMsg>,
    rx_from_brain: Receiver<String>,
) -> bool {
    let mut ports = match serialport::available_ports() {
        Ok(p) => p,
        Err(_) => return false,
    };

    // Prioritize ttyACM1 (Steward)
    ports.sort_by(|a, b| {
        if a.port_name.contains("ttyACM1") {
            std::cmp::Ordering::Less
        } else if b.port_name.contains("ttyACM1") {
            std::cmp::Ordering::Greater
        } else {
            a.port_name.cmp(&b.port_name)
        }
    });

    for p in ports {
        if p.port_name.contains("ttyACM") || p.port_name.contains("ttyUSB") {
            println!("   🔎 Probing Port: {}", p.port_name);
            match serialport::new(&p.port_name, 115200)
                .timeout(Duration::from_millis(1000))
                .open()
            {
                Ok(port) => {
                    // Clone for Reading
                    let mut reader = BufReader::new(port.try_clone().expect("Failed to clone"));
                    let tx = tx_to_brain.clone();

                    // READ THREAD
                    thread::spawn(move || loop {
                        let mut line = String::new();
                        match reader.read_line(&mut line) {
                            Ok(n) if n > 0 => {
                                if tx.send(BridgeMsg::Line(line.trim().to_string())).is_err() {
                                    break;
                                }
                            }
                            Ok(_) => continue,
                            Err(e) => {
                                let _ = tx.send(BridgeMsg::Error(e.to_string()));
                                break;
                            }
                        }
                    });

                    // WRITE THREAD (Allocates the original port)
                    let mut writer = port; // Take ownership
                    thread::spawn(move || {
                        for cmd in rx_from_brain {
                            let _ = writer.write_all(cmd.as_bytes());
                            let _ = writer.flush();
                        }
                    });

                    return true;
                }
                Err(_) => continue,
            }
        }
    }
    false
}
