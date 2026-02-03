use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Signature {
    c: Option<f64>,
    a: Option<f64>,
    r: Option<f64>,
    g: Option<f64>,
    b: Option<f64>,
    ir: Option<f64>,
    uv: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HcscData {
    signature: Option<Signature>,
    transprecision: Option<f64>,
    sovereign: Option<bool>,
    summary: Option<String>,
    raw: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct HcscItem {
    path: String,
    filename: String,
    node: String,
    data: HcscData,
    #[serde(skip_serializing_if = "Option::is_none")]
    rho: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    torque: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    true_precision: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relativity_bias: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    induction_state: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MagnetDef {
    title: String,
    keywords: Option<Vec<String>>,
    #[allow(dead_code)]
    properties: Option<HashMap<String, String>>,
    #[allow(dead_code)]
    logic: Option<String>,
    charge: f64,
    #[allow(dead_code)]
    pair: String,
}

#[derive(Debug, Deserialize)]
struct Trajectories {
    magnets: HashMap<String, MagnetDef>,
}

const PHI: f64 = 1.618033988749895;
const TAU: f64 = 6.283185307179586;

#[allow(dead_code)]
fn get_rod_cone_state(sig: &Signature) -> (f64, f64, f64) {
    let c = sig.c.unwrap_or(0.5);
    let a = sig.a.unwrap_or(0.5);
    let r_val = sig.r.unwrap_or(0.5);
    let g_val = sig.g.unwrap_or(0.5);
    let b_val = sig.b.unwrap_or(0.5);

    let total_struct = (c + a).max(0.001);
    let rods = c / total_struct;

    let total_spec = (r_val + g_val + b_val).max(0.001);
    let cones = r_val / total_spec;

    let fitness = if cones > 0.0 { rods / cones } else { 1.0 };
    (rods, cones, fitness)
}

fn calculate_pair_potential(item: &HcscItem, magnet1: &MagnetDef, magnet2: &MagnetDef) -> f64 {
    let amp1 = calculate_magnet_amplitude(item, magnet1);
    let amp2 = calculate_magnet_amplitude(item, magnet2);

    // Equal and opposite charge interaction
    (amp1 * magnet1.charge) + (amp2 * magnet2.charge)
}

fn calculate_magnet_amplitude(item: &HcscItem, magnet: &MagnetDef) -> f64 {
    let node_text = format!(
        "{:?} {:?} {:?}",
        item.filename, item.data.summary, item.path
    )
    .to_lowercase();
    let mut amp: f64 = 0.0;

    if let Some(keywords) = &magnet.keywords {
        for key in keywords {
            if node_text.contains(&key.to_lowercase()) {
                amp += 0.4;
            }
        }
    }

    // Spectral checks for UV/Infrared
    if magnet.title.contains("Ultraviolet") {
        if let Some(sig) = &item.data.signature {
            if sig.uv.unwrap_or(0.0) > 0.3 {
                amp += 0.5;
            }
        }
    }

    amp.min(1.0f64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 [DIPOLAR-EVOLUTION] Manifesting Hall Effect V6 (Relativity Scope)...");
    println!("🧬 [CONSTANTS] PHI: {:.4} | TAU: {:.4}", PHI, TAU);

    let trajectories_path = "/home/nicoDantigravity/laboratory/trajectories.json";
    let v5_db_path = "/home/nicoDantigravity/laboratory/TOTAL_RES_DB_V5.json";

    let trajectories_str = fs::read_to_string(trajectories_path)?;
    let trajectories: Trajectories = serde_json::from_str(&trajectories_str)?;

    let all_data: Vec<HcscItem> = serde_json::from_str(&fs::read_to_string(v5_db_path)?)?;

    // Engagement Orders
    let orders = vec![
        vec!["classical", "quantum", "void"], // Order A: Classical Dominance
        vec!["quantum", "classical", "void"], // Order B: Quantum Dominance
        vec!["void", "classical", "quantum"], // Order C: Spectral Initialized
        vec!["stability", "void", "quantum"], // Order D: Sovereign Actualization
    ];

    let mut final_results = Vec::new();

    for order in orders {
        println!("📍 [SCOPE] Engagement Order: {:?}", order);
        let mut scope_results = Vec::new();

        for item in &all_data {
            let mut total_relativity = 0.0;
            let mut induction_chain = String::new();

            for step in &order {
                let potential = match *step {
                    "classical" => {
                        let m1 = trajectories.magnets.get("ieee_optical").unwrap();
                        let m2 = trajectories.magnets.get("three_sphere").unwrap();
                        calculate_pair_potential(item, m1, m2)
                    }
                    "quantum" => {
                        let m1 = trajectories.magnets.get("coltrane_circle").unwrap();
                        let m2 = trajectories.magnets.get("prime_resonance").unwrap();
                        calculate_pair_potential(item, m1, m2)
                    }
                    "void" => {
                        let m1 = trajectories.magnets.get("ultraviolet").unwrap();
                        let m2 = trajectories.magnets.get("manager_core").unwrap();
                        calculate_pair_potential(item, m1, m2)
                    }
                    "stability" => {
                        let m1 = trajectories.magnets.get("lil_woody").unwrap();
                        let m2 = trajectories.magnets.get("comet_toss").unwrap();
                        calculate_pair_potential(item, m1, m2)
                    }
                    _ => 0.0,
                };

                total_relativity += potential;
                induction_chain.push_str(&format!("{}:{:.2} ", step, potential));
            }

            if total_relativity.abs() > 0.25 {
                let mut res_item = item.clone();
                res_item.relativity_bias = Some(total_relativity);
                res_item.induction_state = Some(induction_chain);
                scope_results.push(res_item);
            }
        }

        println!("   - Nodes lit up: {}", scope_results.len());
        final_results.extend(scope_results);
    }

    // Sort by Absolute Relativity Bias (Highest Impact)
    final_results.sort_by(|a, b| {
        b.relativity_bias
            .unwrap()
            .abs()
            .partial_cmp(&a.relativity_bias.unwrap().abs())
            .unwrap()
    });

    let output_path = "/home/nicoDantigravity/laboratory/RESONANT_NODES_V6.json";
    fs::write(output_path, serde_json::to_string_pretty(&final_results)?)?;

    println!("\n✨ [FINAL] Hall Effect V6 Accomplished. Relativity Bridge observations committed to substrate.");
    if let Some(top) = final_results.first() {
        println!(
            "   🌀 [MAX SHIFT] {} | Bias={:.4} | Chain={}",
            top.filename,
            top.relativity_bias.unwrap(),
            top.induction_state.as_ref().unwrap()
        );
    }

    Ok(())
}
