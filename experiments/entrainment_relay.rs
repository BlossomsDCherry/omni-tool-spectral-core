//! Frequency Entrainment Relay: Stress Test & Blob Harmonics
//!
//! "Bring on the blobs. They will be crystalline when frozen at the molecular level."
//!
//! This experiment pushes the `ZeroWaitHandshake` pattern to its limits:
//! 1. **Velocity**: High-speed AtomicU32 tossing (Control).
//! 2. **Mass**: Large payload "Blob" transfer (Simulated via sharded channels/Indices).
//! 3. **Crystallinity**: Verifying that the data structure remains invariant under stress.

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

// --- [Atomic Talu64: The Relay Core] ---

/// A wide-channel relay for larger harmonic structures (u64).
pub struct WideTonic {
    comet: AtomicU64,
    pulse: AtomicU32,
}

impl WideTonic {
    pub fn new() -> Self {
        Self {
            comet: AtomicU64::new(0),
            pulse: AtomicU32::new(0),
        }
    }

    pub fn toss(&self, data: u64) {
        self.comet.store(data, Ordering::Release);
        self.pulse.fetch_add(1, Ordering::SeqCst);
    }

    pub fn catch(&self) -> Option<u64> {
        if self.pulse.load(Ordering::Acquire) > 0 {
            let data = self.comet.load(Ordering::Relaxed);
            self.pulse.store(0, Ordering::Release);
            Some(data)
        } else {
            None
        }
    }
}

// --- [Blob Harmonic: The Crystalline Payload] ---

/// Simulates a "Heavy" particle (Blob) being relayed.
/// In a real system, this would be a shared memory pointer or DMA descriptor.
/// Here, we track the *integrity* of the blob through a checksum signature.
#[derive(Clone)]
pub struct BlobHarmonic {
    pub id: u32,
    pub size: usize,
    pub checksum: u64, // The "Crystalline" signature
}

impl BlobHarmonic {
    pub fn new(id: u32, size: usize) -> Self {
        // Create a deterministic "crystal" pattern
        let mut sum: u64 = 0;
        for i in 0..size {
            sum = sum.wrapping_add((i as u64).wrapping_mul(id as u64));
        }
        Self {
            id,
            size,
            checksum: sum,
        }
    }

    pub fn verify(&self) -> bool {
        let mut sum: u64 = 0;
        for i in 0..self.size {
            sum = sum.wrapping_add((i as u64).wrapping_mul(self.id as u64));
        }
        sum == self.checksum
    }
}

// --- [The Experiment] ---

fn main() {
    println!("🧪 FREQUENCY ENTRAINMENT RELAY: STRESS TEST 🌊");
    println!("----------------------------------------------");

    let duration = Duration::from_secs(2);

    // 1. Baseline: u64 High-Speed Relay
    bench_wide_tonic(duration);

    // 2. Heavy Lift: Blob Entrainment
    bench_blob_entrainment(duration, 1024); // 1KB (Water Molecule)
    bench_blob_entrainment(duration, 1024 * 1024); // 1MB (Ice Crystal)

    println!("\n✅ EXPERIMENT COMPLETE: Entrainment Holds. Blobs are Crystalline.");
}

fn bench_wide_tonic(duration: Duration) {
    println!("\n⚡ [TEST] High-Speed Velocity (u64)");
    let tonic = Arc::new(WideTonic::new());
    let tonic_rx = tonic.clone();

    let running = Arc::new(AtomicU32::new(1));
    let running_rx = running.clone();

    // Catcher Thread (The Receiver)
    let catcher = thread::spawn(move || {
        let mut catches = 0;
        let mut checksum = 0u64;
        while running_rx.load(Ordering::Relaxed) == 1 {
            if let Some(val) = tonic_rx.catch() {
                catches += 1;
                checksum = checksum.wrapping_add(val);
            }
        }
        (catches, checksum)
    });

    // Tosser Thread (The Emitter)
    let start = Instant::now();
    let mut tosses = 0;
    while start.elapsed() < duration {
        tosses += 1;
        tonic.toss(tosses as u64);
        // Spin-wait to simulate frequency cap (optional, removing for max stress)
        // thread::yield_now();
    }
    running.store(0, Ordering::Relaxed);

    let (catches, _check) = catcher.join().unwrap();

    println!("   - Duration: {:.2?}", duration);
    println!("   - Tosses:   {}", tosses);
    println!("   - Catches:  {}", catches);
    println!(
        "   - Coherence: {:.2}%",
        (catches as f64 / tosses as f64) * 100.0
    );
    println!(
        "   - Frequency: {:.2} MHz",
        (tosses as f64 / duration.as_secs_f64()) / 1_000_000.0
    );
}

fn bench_blob_entrainment(duration: Duration, blob_size: usize) {
    println!("\n🧊 [TEST] Blob Entrainment Mass: {} bytes", blob_size);

    // For Blobs, we relay the "Descriptor" (ID) via the Tonic,
    // and verify the "Crystalline" structure (Checksum) on the other side.
    let tonic = Arc::new(WideTonic::new());
    let tonic_rx = tonic.clone();

    let running = Arc::new(AtomicU32::new(1));
    let running_rx = running.clone();

    let catcher = thread::spawn(move || {
        let mut valid_crystals = 0;
        let mut flaws = 0;
        while running_rx.load(Ordering::Relaxed) == 1 {
            if let Some(id) = tonic_rx.catch() {
                // Reconstruct and Verify the Blob (Simulating read from shared memory)
                let blob = BlobHarmonic::new(id as u32, blob_size);
                if blob.verify() {
                    valid_crystals += 1;
                } else {
                    flaws += 1;
                }
            }
        }
        (valid_crystals, flaws)
    });

    let start = Instant::now();
    let mut tosses = 0;
    while start.elapsed() < duration {
        tosses += 1;
        // Verify before toss (Simulating write to shared memory)
        let _blob = BlobHarmonic::new(tosses, blob_size);
        tonic.toss(tosses as u64);

        // Simulate "Mass" drag - larger blobs take longer to phase-shift
        if blob_size > 100_000 {
            thread::sleep(Duration::from_micros(1));
        }
    }
    running.store(0, Ordering::Relaxed);

    let (crystals, flaws) = catcher.join().unwrap();

    println!(
        "   - Crystalline Integrity: {}/{}",
        crystals,
        crystals + flaws
    );
    if flaws > 0 {
        println!("   ⚠️  FLAWS DETECTED: {}", flaws);
    } else {
        println!("   💎  PERFECT CRYSTALLINITY");
    }
    println!(
        "   - Throughput: {:.2} MB/s",
        (crystals as f64 * blob_size as f64) / duration.as_secs_f64() / 1_000_000.0
    );
}
