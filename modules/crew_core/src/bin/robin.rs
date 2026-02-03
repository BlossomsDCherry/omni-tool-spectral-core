use serialport::SerialPortType;
use std::io::{Read, Write};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

// Robin Bridge v6: UDP <-> Serial
// Listens on UDP 12345 (for Void-to-Vision app)
// Forwards commands to Steward (Pico 2) via USB Serial
// Streams Steward Telemetry to Stdout

const LISTEN_ADDR: &str = "0.0.0.0:12345";
const BAUD_RATE: u32 = 115200;

fn main() {
    println!("🦜 Robin Bridge (v6): Awakening...");
    println!("   USAGE: Run 'void_to_vision.py' targeting this IP.");

    // 1. Find Serial Port
    let ports = serialport::available_ports().expect("No serial ports found!");
    let mut target_port_name = String::new();

    // Auto-discovery logic
    for p in ports {
        if let SerialPortType::UsbPort(info) = &p.port_type {
            // Filter out Debug Probe explicitly if needed, or accept it if it's the only path
            // For now, we prefer anything that isn't the Probe, but take the Probe if desperate?
            // Let's stick to the "Steward" preference.

            // If MicroPython is running (VID 2e8a, PID 0005), it's a match.
            // If Rust Firmware is running (Custom VID/PID or Generic), it's a match.
            // Debug Probe is 2e8a:000c.

            if info.vid == 0x2e8a && info.pid == 0x000c {
                println!(
                    "   ⚠️ Found Debug Probe at {}. Skipping for now (preferring Target USB).",
                    p.port_name
                );
                continue;
            }

            println!(
                "   ✅ Found Candidate: {} ({:04x}:{:04x})",
                p.port_name, info.vid, info.pid
            );
            target_port_name = p.port_name.clone();
            break;
        }
    }

    if target_port_name.is_empty() {
        println!(
            "❌ No dedicated Steward USB detected. (Is firmware running? Is MicroPython active?)"
        );
        println!("   ⚠️ NOTE: If using Rust Firmware without USB stack, you must use the Debug Probe UART.");
        println!("   Exiting to prevent false locking.");
        return;
    }

    // 2. Open Serial Port
    println!("   🔗 Connecting to Steward at {}...", target_port_name);
    let serial_port = serialport::new(&target_port_name, BAUD_RATE)
        .timeout(Duration::from_millis(10))
        .open();

    match serial_port {
        Ok(mut port) => {
            println!("   🟢 SERIAL LINK ESTABLISHED.");

            let mut port_clone = port.try_clone().expect("Failed to clone PTY");

            // 3. UDP Listener Thread (Commands -> Serial)
            thread::spawn(move || {
                println!("   👂 UDP Listener active on {}", LISTEN_ADDR);
                let socket = UdpSocket::bind(LISTEN_ADDR).expect("Failed to bind UDP socket");
                let mut buf = [0u8; 1024];

                loop {
                    match socket.recv_from(&mut buf) {
                        Ok((amt, _src)) => {
                            let cmd = &buf[..amt];
                            let cmd_str = String::from_utf8_lossy(cmd).trim().to_string();
                            println!("   📥 UDP RX: [{}] -> Forwarding to Serial", cmd_str);

                            // Forward to Serial
                            if let Err(e) = port_clone.write_all(cmd) {
                                eprintln!("   ❌ Serial Write Error: {}", e);
                            }
                            if let Err(e) = port_clone.write_all(b"\n") {
                                // Ensure newline
                                eprintln!("   ❌ Serial Write Error: {}", e);
                            }
                        }
                        Err(e) => eprintln!("   ❌ UDP Recv Error: {}", e),
                    }
                }
            });

            // 4. Serial Reader Loop (Telemetry -> Stdout)
            println!("   📤 Forwarding Serial Telemetry to Console...");
            let mut serial_buf = [0u8; 1024];
            loop {
                match port.read(&mut serial_buf) {
                    Ok(n) if n > 0 => {
                        let s = String::from_utf8_lossy(&serial_buf[..n]);
                        print!("{}", s); // Stream raw telemetry
                                         // TODO: Parse JSON and optimize display?
                    }
                    Ok(_) => {}
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {}
                    Err(e) => {
                        println!("   ❌ Serial Read Error: {}, retrying...", e);
                        thread::sleep(Duration::from_millis(500));
                    }
                }
                thread::sleep(Duration::from_millis(1));
            }
        }
        Err(e) => {
            eprintln!("   ❌ Failed to Open Serial Port: {}", e);
        }
    }
}
