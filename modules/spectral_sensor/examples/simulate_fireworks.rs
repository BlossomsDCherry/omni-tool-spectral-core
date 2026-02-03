use spectral_sensor::{BioRhythm, NvmeWind, SpectralPort};
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn main() {
    println!("🔥 SIMULATION START: NVMe Wind x Musical Physics");

    let sensor = SpectralPort::new("Simulated_IR_Node_Alpha");
    let mut wind = NvmeWind::new(874_778); // Target velocity from report

    let batch_size = 10_000;
    let mut total_events = 0u64;
    let start_time = Instant::now();
    let mut last_log = Instant::now();

    loop {
        // 1. Safety Check (Rhythm)
        let coherence = sensor.heartbeat();
        if coherence < 0.1 {
            // Simulated "Systole" pause (though in real code we might just buffer)
            // println!("   ...systolic pause (coherence {:.2})...", coherence);
            thread::sleep(Duration::from_millis(10));
            continue;
        }

        // 2. High-Velocity Blast
        let _data = wind.blast(batch_size);
        total_events += batch_size as u64;

        // 3. Telemetry Log (Every 1 second)
        if last_log.elapsed().as_secs() >= 1 {
            let elapsed = start_time.elapsed().as_secs_f64();
            let rate = total_events as f64 / elapsed;

            println!(
                "💨 WIND VELOCITY: {:.0} units/sec | ❤️  COHERENCE: {:.4} | TOTAL: {}",
                rate, coherence, total_events
            );

            last_log = Instant::now();

            // Exit after 5 seconds of successful streaming
            if elapsed > 5.0 {
                break;
            }
        }

        // Micro-sleep to prevent 100% CPU lock (simulating breathing room)
        // Adjust this to tune the "Velocity"
        thread::sleep(Duration::from_micros(10));
    }

    println!("✅ SIMULATION COMPLETE. The Phantom Organ holds the wind.");
}
