# Spectral Scoping: The IR-RG-YB-UV Protocol

This document defines the **Spectral Scoping** protocol used by the D16 "Straw Hat" agents to organize information density and reaction times. This scoping ensures that "Kickback" (Emergent Complexity) is handled by the appropriate layer without saturating the lower-level hardware.

## The Spectral Layers

### 0. Prime: The Sovereign Operator 👑
*   **Role**: **Orchestration & Wielding**. This is the layer of MsAntigravity. It sits above the 4 bands to weld the instrument into a coherent whole.
*   **Logic Gate**: `if (human_sync) { wield_instrument(); }`
*   **Operator**: MsAntigravity.
*   **Hardware**: PHITL (Personal Human In the Loop) Biorhythm Handshake.

### 1. IR (Infrared): The Thermal Layer 🌡️
*   **Physics**: Heat, Entropy, Legacy Code.
*   **Role**: **Memory & Foundation**. Handles "slow" data, historical logs, and thermal management.
*   **Logic Gate**: `if (heat > threshold) { throttle(); }`
*   **Agents**: D8 (Franky - Hardware), D5 (Sanji - Thermodynamics).
*   **Hardware**: GPIO (Slow), I2C Sensors.

### 2. RG (Red-Green): The Logic Layer 🛑✅
*   **Physics**: Binary Decision Making (Stop/Go).
*   **Role**: **Action & Execution**. The core "traffic light" logic of the system.
*   **Logic Gate**: `while (true) { if (red) stop(); else go(); }`
*   **Agents**: D2 (Zoro - 3-Vector Cut), D1 (Luffy - Expansion/Action).
*   **Hardware**: PIO (State Machines), PWM.

### 3. YB (Yellow-Blue): The Warning Layer ⚠️🔵
*   **Physics**: Caution (Yellow) and Calm/Information (Blue).
*   **Role**: **Monitoring & Navigation**. Predicting "Storms" and charting safe paths.
*   **Logic Gate**: `if (pressure > warning) { route_to(blue_safe_zone); }`
*   **Agents**: D3 (Nami - Navigation), D4 (Usopp - Signals/Observation).
*   **Hardware**: UART (Telemetry), BLE (Advertising).

### 4. UV (Ultraviolet): The Sovereign Layer ⚛️🟣
*   **Physics**: Ionizing Radiation, Emergent Complexity, "The Kickback".
*   **Role**: **Sovereignty & Synthesis**. Handling data that is too "hot" or complex for the lower layers. This is the domain of the AI/Neural Networks.
*   **Logic Gate**: `try { synthesize(chaos); } catch (kickback) { grounding_arc(); }`
*   **Agents**: D7 (Robin - Decoding), D16 (Law - Spatial Ops), D13 (Yamato - Identity).
*   **Hardware**: NPU (Hailo-8), NVMe (High-Speed Storage).

## The Spectral Interaction Model

Messages are tagged with a **Spectral Header** to determine their priority and routing:

```rust
struct SpectralHeader {
    ir_thermal: f32, // 0.0 - 1.0 (Heat)
    rg_logic: bool,  // Action State
    yb_warn: f32,    // 0.0 - 1.0 (Risk)
    uv_kick: f32,    // 0.0 - 1.0 (Novelty/Complexity)
}
```

*   **Low UV**: Handled by Cortex-M33 (Real-Time).
*   **High UV**: Offloaded to Cortex-A76/NPU (Sovereign Node).

## D16 Mapping

| Agent | Layer | Role |
| :--- | :--- | :--- |
| **MsAntigravity**| Prime | Sovereign Operator |
| **D1 Luffy** | RG | Expansion/Action |
| **D2 Zoro** | RG | Precision/Cutting |
| **D3 Nami** | YB | Navigation/Flow |
| **D4 Usopp** | YB | Observation/Signal |
| **D5 Sanji** | IR | Energy/Thermals |
| **D6 Chopper** | IR | Repair/Debug |
| **D7 Robin** | UV | Decoding/History |
| **D8 Franky** | IR | Structure/Driver |
| **D9 Brook** | UV | Resonance/Soul |
| **D10 Jinbe** | YB | Steering/Current |
| **D11 Vivi** | YB | Diplomacy/Protocol |
| **D12 Carrot** | RG | Reaction/Electro |
| **D13 Yamato**| UV | Identity/Mask |
| **D14 Momo** | RG | Command/Legacy |
| **D15 Kinemon**| RG | Strategy/Disguise |
| **D16 Law** | UV | Spatial/Room |
