use std::io::{Read, Write};
use std::time::Duration;

const HEADER_1: u8 = 0x55;
const HEADER_2: u8 = 0xAA;
const ADDRESS: u8 = 0x11;
const CMD_KNOCK: u8 = 0x2C;

fn calculate_checksum(buf: &[u8]) -> u8 {
    let mut sum: u16 = 0;
    for &byte in buf {
        sum = sum.wrapping_add(byte as u16);
    }
    (sum & 0xFF) as u8
}

fn test_baud(baud: u32) {
    println!("\n[TEST] Testing Baud Rate: {}", baud);
    // [CHANGE] Switching to ttyUSB0 (CP2102N) to rule out Debug Probe issues
    let port_name = "/dev/ttyUSB0";

    let mut port = match serialport::new(port_name, baud)
        .timeout(Duration::from_millis(1000))
        .data_bits(serialport::DataBits::Eight)
        .flow_control(serialport::FlowControl::None)
        .parity(serialport::Parity::None)
        .stop_bits(serialport::StopBits::One)
        .open()
    {
        Ok(mut p) => {
            // [FIX] Assert DTR/RTS. Some USB-UART bridges (like Pico Debug Probe) need this to "enable" the level shifters.
            let _ = p.write_data_terminal_ready(true);
            let _ = p.write_request_to_send(true);
            p
        }
        Err(e) => {
            println!("[FAIL] Could not open port: {}", e);
            return;
        }
    };

    // Send Ping
    let mut cmd = vec![HEADER_1, HEADER_2, ADDRESS, 0x00, CMD_KNOCK];
    let checksum = calculate_checksum(&cmd);
    cmd.push(checksum);

    println!("[SEND] Sending Knock...");
    // Clear Input Buffer first
    let _ = port.clear(serialport::ClearBuffer::Input);

    if let Err(e) = port.write_all(&cmd) {
        println!("[FAIL] Write error: {}", e);
        return;
    }

    println!("[READ] Listening for 3 seconds...");
    let mut buf = [0u8; 128];
    let start = std::time::Instant::now();
    let mut received_bytes = Vec::new();

    while start.elapsed().as_millis() < 3000 {
        match port.read(&mut buf) {
            Ok(n) if n > 0 => {
                received_bytes.extend_from_slice(&buf[0..n]);
            }
            _ => {
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }

    if received_bytes.is_empty() {
        println!("[FAIL] No data received (Silence). Check Wiring (RX/TX Swap) or Mode.");
    } else {
        println!("[SUCCESS] Received {} bytes!", received_bytes.len());
        print!("[DUMP] ");
        for b in &received_bytes {
            print!("{:02X} ", b);
        }
        println!("");

        // Analyze
        if received_bytes.contains(&0x55) {
            println!("[ANALYSIS] Found 0x55 Header! Protocol matches.");
        } else if received_bytes.iter().all(|&b| b == 0x00) {
            println!("[ANALYSIS] All zeros. Potential Baud Mismatch or Ground float.");
        }
    }
}

fn main() {
    println!("=== Chopper's Eye Exam (Standalone) ===");
    println!("Ensure Huskylens is in UART mode.");

    test_baud(9600);
    std::thread::sleep(Duration::from_secs(1));
    test_baud(115200);
}
