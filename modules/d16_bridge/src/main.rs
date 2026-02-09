use d16_bridge::serial_bridge::SerialBridge;
use d16_bridge::ble_bridge::BleBridge;
use log::{info, error};
use simple_logger::SimpleLogger;
use std::thread;

#[tokio::main]
async fn main() {
    SimpleLogger::new().init().unwrap();

    info!("D16 Listener Starting (Sovereign Receiver)...");
    info!(">> Mode: Unified (Serial + BLE)");

    // 1. Spawn Serial Listener (Blocking IO)
    // We launch this on a dedicated thread because serialport is blocking
    let _serial_handle = thread::spawn(|| {
        let args: Vec<String> = std::env::args().collect();
        let port_name = if args.len() > 1 {
            &args[1]
        } else {
            "/dev/ttyACM0"
        };
        let baud_rate = 115200;
        
        loop {
            let bridge = SerialBridge::new(port_name, baud_rate);
            info!("🔌 Serial: Connecting to {}...", port_name);
            
            if let Err(e) = bridge.listen(|report| {
                info!(" [SERIAL] Stance: {:?} | Color: {}", report.stance, report.hex_color);
                info!("          Saturation: {:?} [Contrast:{:.2} RG:{:.2} BY:{:.2} UV:{:.2}]", 
                    report.saturation.state,
                    report.saturation.contrast_score, 
                    report.saturation.rg_stability,
                    report.saturation.by_stability,
                    report.saturation.uv_afterimage
                );

                // Write Voltage Differential (Contrast) to SHM for Hailo
                if let Ok(mut file) = std::fs::File::create("/dev/shm/d16_saturation_voltage") {
                     use std::io::Write;
                     let _ = write!(file, "{:.4}", report.saturation.contrast_score);
                }
            }) {
                error!("🔌 Serial: Connection Lost/Failed: {}", e);
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        }
    });

    // 2. Spawn BLE Listener (Async)
    let ble_handle = tokio::spawn(async {
        let bridge = BleBridge::new().await;
        info!("📡 BLE: Scanning for Hubble...");
        
        if let Err(e) = bridge.scan_and_listen().await {
            error!("📡 BLE: Scanner Failed: {}", e);
        }
    });
    
    // Wait for both (effectively forever)
    let _ = tokio::join!(ble_handle);
    // serial_handle.join().unwrap(); // We don't really join this unless we want to exit on serial death
}
