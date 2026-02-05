use crossbeam::channel::{unbounded, Receiver, Sender};
use memmap2::MmapMut;
use std::f64::consts::PI;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};

// --- CONSTANTS ---
const TAU: f64 = 6.2831853; // 8 Sig Figs
const PI: f64 = 3.1415927; // 8 Sig Figs
const PHI: f64 = 1.6180340; // 8 Sig Figs
const E: f64 = 2.7182818; // 8 Sig Figs
const GRID_SIZE: usize = 64;
const DAMPING: f64 = 0.99;
const C: f64 = 0.5; // Wave Speed
const DT: f64 = 0.1;
const DX: f64 = 1.0;
const SHM_PATH: &str = "/dev/shm/current_wave_coherence";

// --- Sobel-Feldman Kernels ---
const SOBEL_X: [[f64; 3]; 3] = [[-1.0, 0.0, 1.0], [-2.0, 0.0, 2.0], [-1.0, 0.0, 1.0]];

const SOBEL_Y: [[f64; 3]; 3] = [[-1.0, -2.0, -1.0], [0.0, 0.0, 0.0], [1.0, 2.0, 1.0]];

// --- Structures ---
#[derive(Clone)]
struct WaveState {
    u: Vec<Vec<f64>>,
    frame: u64,
}

impl WaveState {
    fn new() -> Self {
        Self {
            u: vec![vec![0.0; GRID_SIZE]; GRID_SIZE],
            frame: 0,
        }
    }
}

// --- BIG CORE: Wave Physics Engine ---
fn big_core_physics(state: Arc<RwLock<WaveState>>, tx_visual: Sender<WaveState>) {
    let mut u = vec![vec![0.0; GRID_SIZE]; GRID_SIZE];
    let mut u_prev = u.clone();
    let mut u_next = u.clone();
    let mut frame = 0;

    let start_time = Instant::now();

    loop {
        // 1. FDTD Laplacian Solve
        for y in 1..GRID_SIZE - 1 {
            for x in 1..GRID_SIZE - 1 {
                let laplacian =
                    u[y - 1][x] + u[y + 1][x] + u[y][x - 1] + u[y][x + 1] - 4.0 * u[y][x];

                u_next[y][x] = (2.0 * u[y][x]) - u_prev[y][x]
                    + (C.powi(2)) * (DT.powi(2) / DX.powi(2)) * laplacian;
                u_next[y][x] *= DAMPING;
            }
        }

        // 2. Source Injection (Crew Inputs)
        let t = frame as f64 * DT;

        // Source A (Center - Robin): Modulated by TAU
        let src_a_val = 2.0 * (t * 0.2 * TAU).sin();
        u_next[32][32] = src_a_val;

        // Source B (Interference): Offset
        let src_b_val = 1.0 * (t * 0.3 * TAU).sin();
        u_next[16][16] = src_b_val;

        // 3. Cycle Buffers
        u_prev = u.clone();
        u = u_next.clone();
        frame += 1;

        // 4. Update Shared State
        {
            let mut write_guard = state.write().unwrap();
            write_guard.u = u.clone();
            write_guard.frame = frame;
        }

        // 5. Send to Little Core (Visual/Analysis) every few frames
        if frame % 2 == 0 {
            tx_visual
                .send(WaveState {
                    u: u.clone(),
                    frame,
                })
                .ok();
        }

        thread::sleep(Duration::from_millis(50));
    }
}

// --- LITTLE CORE: Sobel-Feldman & Coherence ---
fn little_core_analysis(rx_visual: Receiver<WaveState>) {
    loop {
        if let Ok(state) = rx_visual.recv() {
            let u = state.u;

            // 1. Apply Sobel-Feldman Operator
            // This detects edges/gradients in the wave field, representing "energy fronts".
            let mut coherence_accum = 0.0;
            let mut energy_total = 0.0;

            for y in 1..GRID_SIZE - 1 {
                for x in 1..GRID_SIZE - 1 {
                    let mut gx = 0.0;
                    let mut gy = 0.0;

                    for ky in 0..3 {
                        for kx in 0..3 {
                            let val = u[y + ky - 1][x + kx - 1];
                            gx += val * SOBEL_X[ky][kx];
                            gy += val * SOBEL_Y[ky][kx];
                        }
                    }

                    let magnitude = (gx.powi(2) + gy.powi(2)).sqrt();
                    energy_total += magnitude;

                    // Coherence Metric: How much energy is concentrated vs geometric?
                    // Simple proxy: Center weighted coherence
                    let dx = (x as f64) - (GRID_SIZE as f64 / 2.0);
                    let dy = (y as f64) - (GRID_SIZE as f64 / 2.0);
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < 8.0 {
                        coherence_accum += magnitude;
                    }
                }
            }

            let coherence_metric = if energy_total > 0.0 {
                (coherence_accum / energy_total) * 10.0
            } else {
                0.0
            };

            // 2. Write to Shared Memory
            if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(SHM_PATH) {
                let _ = write!(file, "{:.4}", coherence_metric);
            }

            // 3. ASCII Render (Low Priority)
            print!("\x1B[2J\x1B[1;1H"); // Clear Screen
            println!(
                "--- ZINC-RUST RIPPLE TANK Frame: {} (Tau Mode) ---",
                state.frame
            );
            println!("Coherence: {:.4}", coherence_metric);

            let chars = ['.', ':', '-', '=', '+', '*', '#', '%', '@'];
            for y in (0..GRID_SIZE).step_by(2) {
                for x in (0..GRID_SIZE).step_by(2) {
                    let val = u[y][x];
                    let idx = ((val + 1.0) * 4.0) as usize;
                    let idx = idx.min(chars.len() - 1);
                    print!("{}", chars[idx]);
                }
                println!("");
            }
        }
    }
}

fn main() {
    println!("Initializing Native Physics Engine...");

    let shared_state = Arc::new(RwLock::new(WaveState::new()));
    let (tx, rx) = unbounded();

    // Spawn Little Core (Analysis)
    let state_clone = shared_state.clone();
    thread::spawn(move || {
        little_core_analysis(rx);
    });

    // Run Big Core (Physics) on Main Thread
    big_core_physics(shared_state, tx);
}
