# D16 Harmonic Topology Dashboard 🌊

A sovereign, single-file visualization interface for the D16 Spectral Core.

## Features
- **Web Serial API**: Connects directly to the Pico 2 W without middleware.
- **Talu64 Logic**: Visualizes the 8 Gates and 64 Positions of the Harmonic Manifold.
- **Emergent Topology**: Renders a "Breathing Torus" driven by real-time `tau` and `ECG` data.

## How to Run
1. **Serve the file**:
   Web Serial requires a secure context (HTTPS or localhost).
   ```bash
   cd dashboard
   python3 -m http.server 8000
   ```
2. **Open in Browser**:
   Navigate to `http://localhost:8000`.
3. **Connect**:
   - Click **Connect D16**.
   - Select your Pico 2 W (e.g., `/dev/ttyACM0`).
   - Watch the geometry breathe.

## Demo Mode
If no hardware is available, click **Demo Mode** to simulate the harmonic waves.
