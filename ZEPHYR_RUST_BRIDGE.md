# Zephyr/Rust Cross-Core Bridge

## Architecture

**M33 Side** (Zephyr RTOS, C)
- Real-time spectral mass control
- Phase-locked loops, GPIO timing
- Event log (append-only, DMA-buffered)

**Dragonwing Side** (Debian/Linux, Rust)
- Spectral analysis, parameter optimization
- Network/storage/AI acceleration
- Monitoring & orchestration

## Interface

**Shared Memory Region** (64KB, dual-mapped)

```
Offset  Field               Size    Owner
--------------------
0x0000  Control Word        4B      Dragonwing (set), M33 (read)
0x0004  Status Word         4B      M33 (write), Dragonwing (read)
0x0008  Phase Parameters    32B     Dragonwing (write), M33 (read)
0x0028  Telemetry Ring      32KB    M33 (write), Dragonwing (read)
...
```

**RPCs via Inter-Processor Interrupt (IPI)**
- M33 → Dragonwing: "telemetry ready", "request reconfig"
- Dragonwing → M33: "apply phase update", "trigger log dump"

## Example: Deploy New Phase-Lock

```rust
// Dragonwing side (Rust)
fn deploy_phase_lock(tau_precision: f64, phase: f64) -> Result<()> {
    let params = PhaseParams {
        tau_8sig: 6.2831853,  // τ to 8 sig figs
        phase_offset: phase,
        integration_passes: 15,
    };
    
    // Write to shared memory (Dragonwing → M33)
    unsafe {
        std::ptr::copy_nonoverlapping(
            &params as *const _ as *const u8,
            SHARED_PHASE_PARAMS as *mut u8,
            std::mem::size_of::<PhaseParams>(),
        );
    }
    
    // Trigger M33 interrupt
    IPI::send_to_m33(IPI_PHASE_UPDATE)?;
    
    // Wait for M33 status ACK
    loop {
        if STATUS_WORD.load(Ordering::Acquire) & STATUS_ACK != 0 {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(1));
    }
}
```

This is the **living interface** between your sovereignty stack and the spectral experiments.
