# Sovereign Node Factory: RPi 5 + D16 + TPP

## Architecture


## Why This Matters

1. **No vendor lock**: You control the entire build pipeline.
2. **No cloud CI**: Everything runs locally on RPi 5.
3. **Heterogeneous deployment**: Both RT + Linux sides rebuilt together, atomically.
4. **Sovereign attestation**: D16 ensures integrity across both cores.

## Build Flow

1. RPi 5 pulls your specs (spectral masses config, τ precision, etc.).
2. D16 cross-compiles M33 RT kernel + Dragonwing userspace.
3. TPP deploys both over a single channel to UNO Q.
4. Zephyr/Rust bridge on UNO Q spins up both cores coherently.
5. Feedback loop: M33 telemetry → Dragonwing → RPi 5.

## Next Steps

- Document D16 protocol (how it orchestrates M33 + Dragonwing).
- Show TPP as the liveness channel (proof the board is responsive).
- Prove this works for the spectral experiments (buzzer, pendula, cradle).
