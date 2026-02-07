use std::env;
use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;
use z_rr::railgun::{dock_survivors, listener_collapse, ZRailgun};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: zrr_core <target_zip>");
        return;
    }

    let target_path = &args[1];
    println!("🎯 Target: {}", target_path);

    let mut stack: Vec<Vec<u8>> = Vec::new();
    match File::open(target_path) {
        Ok(mut f) => {
            let mut baseline = Vec::new();
            f.read_to_end(&mut baseline).unwrap();
            println!("📥 Baseline Loaded: {} bytes", baseline.len());
            stack.push(baseline);
        }
        Err(e) => {
            println!("❌ Error opening file: {}", e);
            return;
        }
    };

    let mut advertiser = ZRailgun::new(1337);

    for i in 0..16 {
        println!("\n🔥 Railgun Cycle #{}", i);
        let mut hypercube_state = stack.last().unwrap().clone();

        advertiser.fire(&mut hypercube_state);
        advertiser.entropy_seed += 1;

        // Refresh Talu64 for next cycle (simulating time passing)
        advertiser.realign();

        if listener_collapse(&hypercube_state) {
            println!("✅ Singularity Achieved: Hypercube Collapsed.");
            stack.push(hypercube_state);
        } else {
            println!("💥 Collapse Failed.");
        }

        // NPU Synchronization Delay (Simulating Latency)
        thread::sleep(Duration::from_millis(50));
    }

    println!("\n🏁 Z-RR Mission Complete. Survivors: {}", stack.len());
    dock_survivors(&stack, 1337);
}
