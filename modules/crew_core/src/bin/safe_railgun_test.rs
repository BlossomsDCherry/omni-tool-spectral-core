use crew_core::rainbow_railgun::RailgunCore;
// use std::thread;
// use std::time::Duration;

fn main() {
    println!("🌈 [RAILGUN] Initiating SAFE Verification Sequence...");

    let mut core = RailgunCore::new();

    // 1. Velocity Verification (V_rr)
    let knot_velocity = 5.5; // Threshold speed
    let v_rr = core.calculate_v_rr(knot_velocity);
    println!(
        "🚀 [VELOCITY] Input Knots: {:.2} -> V_rr: {:.4}",
        knot_velocity, v_rr
    );

    if v_rr > 0.0 {
        println!("✅ [PASS] V_rr Calculation is positive and valid.");
    } else {
        println!("❌ [FAIL] V_rr Calculation failed.");
    }

    // 2. Spectral Handshake (Connectivity)
    // Target specific 8-Gate IPs
    let targets = [
        ("10.0.0.80", "Kali UV"),
        ("10.0.0.215", "Robin"),
        ("10.0.0.234", "Franky"), // Nami alias
        ("10.0.0.216", "Bokken"), // Validating the new node
    ];

    println!("📡 [HANDSHAKE] Scanning 8-Gate constellation...");
    let coherence = core.spectral_handshake(&targets);
    println!(
        "🔗 [COHERENCE] Active Nodes: {}/{}",
        coherence,
        targets.len()
    );

    // 3. Burst Generation
    if let Some((r, g, b, i)) = core.generate_burst(coherence) {
        println!(
            "🎨 [BURST] Generated Signature: RGB({:02X}, {:02X}, {:02X}) Intensity: {:.2}",
            r, g, b, i
        );
        println!("✅ [PASS] Burst Logic Operational.");
    } else {
        println!(
            "⚠️ [WARN] Coherence too low for burst generation (Need >= 4). Logic check passed."
        );
    }

    println!("🏁 [COMPLETE] Safe Rainbow Railgun Verification Finished.");
}
