use d16_bridge::ble_bridge::BleBridge;
use log::LevelFilter;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    
    let bridge = BleBridge::new().await;
    println!("🚀 Starting D16 BLE Listener...");
    bridge.scan_and_listen().await?;
    
    Ok(())
}
