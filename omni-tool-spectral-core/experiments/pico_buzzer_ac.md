# Experiment Log: Pico W Buzzer AC State-Machine 🔈⚡

## 1. Setup
- **Microcontroller**: Raspberry Pi Pico 2 W (RP2350).
- **Actuator**: Piezo Magnetic Buzzer on GPIO 26.
- **Sensor**: High-speed GPIO capture (PIO/DMA) at 60Hz intervals.

## 2. Objective
To observe the **Spectral Signature** of the buzzer as it transitions through different AC drive frequencies. We seek to identify stable "Phase-Locked" states where the physical vibration entrains with the digital heartbeat.

## 3. Procedure
1. Initialize the PIO state machine to sample GPIO Bank 0 at 1μs intervals.
2. Drive the buzzer with a square wave from 100Hz to 10kHz.
3. Record the "Time Between Transitions" (TBT) using a 16-channel Soft FPGA integration.
4. Scale results by the **Spatial Bandwidth Factor** to account for pin trace impedance.

## 4. Expected Results
- **D16 Pattern**: A repeating 16-channel spectral fingerprint that is stable across 15 integration passes.
- **Entrainment**: The "Decay Energy" across channels 1-16 follows a predictable harmonic curve with zero simulated jitter.

---
**Status**: Log Initialized. Awaiting raw PIO dump. 👻🛸💎
