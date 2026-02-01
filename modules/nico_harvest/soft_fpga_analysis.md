# Analysis: D16 Soft FPGA Kernel (`soft_fpga.rs`)

**Artifact**: `modules/nico_harvest/soft_fpga.rs`
**Origin**: Laboratory/Bin_7 (Compost Harvest)
**Significance**: "Millennium Solution" Artifact (Zemon Realm)

## Core Philosophy
The kernel implements a "Soft FPGA" where logic gates are not silicon paths but **Navigation States** derived from **Spectral Density** and **Relative Entropy**.

### 1. Atomic Coherence ("Why 8?")
The code explicitly explicitly maps density to electron shells (Lines 202-216):
```rust
// Stable shells: 2 (He), 10 (Ne), 18 (Ar), 26 (Fe - Iron Core).
let shell_pos = (density * 60.0) as i32;
let is_atomic_stable = shell_pos == 2 || shell_pos == 10 || shell_pos == 18 || shell_pos == 26;
```
This suggests the "Why 8?" comes from the **Noble Gas Stability** (2, 10, 18) where the valence shell is full (8 electrons for Ne/Ar). The code reduces entropy (`*= 0.5`) when these nodes are hit.

### 2. Navigation States
The system steers through:
- **VoidExtraction**: Escaping 0-density.
- **GestaltActualization**: Stable operation.
- **CometSlingshot**: High-velocity correction ("Hugging the Void").
- **RainbowRailgun**: High-order orbit (recalling the "Rainbow Railgun" script).

### 3. The Math Systems
- **Roman**: Starts at I (1.0). Existence Exists. Used when near Void.
- **Arabic**: Includes Zero. Phase-Tracking. Used for high-precision navigation.

## Connection to Experiments
The "Piezo Test" (piko_buzzer_ac.md) confirms the **physical** manifestation of this logic:
- **LED (Electro-Kinetic)**: Non-reversible, stable light (like the `GestaltActualization`).
- **Piezo (Magneti-Kinetic)**: Spectral torque (like the `CometSlingshot` or `TorsionalPendulum`).
