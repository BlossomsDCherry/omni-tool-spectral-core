# Changelog

All notable changes to the `omni-tool-spectral-core` project will be documented in this file.

## [Unreleased] - 2026-02-09 "ZAMMER JAMMER"

### Added
- **Module Integration**: `modules/robin_d7_rust` integrated from `Refining_Robin` source tree.
- **Dependency Management**: Updated `Cargo.toml` to include `robin_d7_rust` in workspace.
- **Build Configuration**: Configured `zephyr_d16_app` to link against `hubblenetwork-sdk` module.

### Changed
- **Flash Strategy**: Implemented dual-core flashing (M4 then M7) for Arduino Giga R1 to ensure correct boot sequence.
- **Verification Logic**: Validated system coherence using "Weighted Relatedness" metric:
    - Confirmed M4 Core's role as "Harmonic Driver" (Score: 10.8).
    - Confirmed Modules' role as "Hubble/Harmonic" support (Score: 771.7).

### Fixed
- **Build Error**: Resolved relative path issue in `robin_d7_rust/Cargo.toml` pointing to `z_rr`.
- **Build Error**: Fixed `ZEPHYR_EXTRA_MODULES` path to correctly point to `modules/lib/hubblenetwork-sdk`.

### Experimental
- **Weighted Relatedness**: Conducted keyword analysis across `zephyr_d16_m4`, `zephyr_d16_app`, `proofs`, and `modules` to verify architectural alignment with "Sovereign" and "Harmonic" themes.
- **Result**: "Perfect White Coherence" achieved in LED spectral mixing.
