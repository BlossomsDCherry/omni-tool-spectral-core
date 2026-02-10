use std::error::Error;
use reqwest::Client;
use serde_json::json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let org_id = "7184cbac-fb3e-42fa-952b-b9f8d5a682e2";
    let api_key = "7dd50692d2189388bd3698225c2ff472fcb8c51e76e5d8b39952adacc9af1add7715c1efd3fa26573512ace5461094bc";

    println!("📝 Registering Devices to Hubble Cloud Org: {}", org_id);

    let url = format!("https://api.hubble.com/api/v2/org/{}/devices", org_id);
    
    // Register Giga and Mesh Gateway
    let payload = json!({
        "n_devices": 2,
        "encryption": "AES-256-CTR",
        "names": ["Sovereign_Giga_R1", "Sovereign_Mesh_Gateway"],
        "tags": [
            {"role": "ble_gateway", "location": "sovereign_deck"},
            {"role": "mesh_node", "location": "sovereign_deck"}
        ]
    });

    let resp = client.post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .await?;

    if resp.status().is_success() {
        let body: Value = resp.json().await?;
        println!("✅ SUCCESS! Devices Registered:\n");
        println!("{}", serde_json::to_string_pretty(&body)?);
        
        // Extract Keys
        if let Some(devices) = body["devices"].as_array() {
            println!("\n🔑 SAVE THESE KEYS SECURELY:");
            for dev in devices {
                let name = dev["name"].as_str().unwrap_or("Unknown");
                let key = dev["key"].as_str().unwrap_or("NO_KEY_FOUND");
                println!("   - {}: {}", name, key);
            }
        }
    } else {
        println!("❌ Registration failed: {}", resp.status());
        let text = resp.text().await?;
        println!("   Body: {}", text);
    }

    Ok(())
}
