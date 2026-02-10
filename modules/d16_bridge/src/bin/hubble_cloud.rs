use std::error::Error;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let org_id = "7184cbac-fb3e-42fa-952b-b9f8d5a682e2";
    let api_key = "7dd50692d2189388bd3698225c2ff472fcb8c51e76e5d8b39952adacc9af1add7715c1efd3fa26573512ace5461094bc";

    println!("📡 Querying Hubble Cloud for Organization: {}", org_id);

    let url = format!("https://api.hubble.com/api/org/{}/devices", org_id);
    
    let resp = client.get(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Accept", "application/json")
        .send()
        .await?;

    if resp.status().is_success() {
        let body: Value = resp.json().await?;
        // println!("✅ Response: {}", serde_json::to_string_pretty(&body)?);

        if let Some(devices) = body["devices"].as_array() {
            println!("✅ Found {} Registered Devices:", devices.len());
            for dev in devices {
                let id = dev["id"].as_str().unwrap_or("Unknown");
                let name = dev["name"].as_str().unwrap_or("Unnamed");
                let created = dev["created_ts"].as_i64().unwrap_or(0);
                println!("   - [{}] '{}' (Created: {})", id, name, created);
                
                if let Some(packet) = dev.get("most_recent_packet") {
                     println!("     Last Packet: {:?}", packet);
                }
            }
            
            // Fetch validation details for the first device to see if KEY is exposed
            if let Some(first_dev) = devices.first() {
                let id = first_dev["id"].as_str().unwrap();
                println!("\n🔍 Fetching Details for Device: {}", id);
                let detail_url = format!("https://api.hubble.com/api/org/{}/devices/{}", org_id, id);
                let detail_resp = client.get(&detail_url)
                    .header("Authorization", format!("Bearer {}", api_key))
                    .header("Accept", "application/json")
                    .send()
                    .await?;
                
                if detail_resp.status().is_success() {
                    let detail_body: Value = detail_resp.json().await?;
                    println!("📄 Device Detail:\n{}", serde_json::to_string_pretty(&detail_body)?);
                } else {
                    println!("❌ Failed to fetch device detail: {}", detail_resp.status());
                }
            }

        } else {
            println!("⚠️ No devices array found in response.");
        }
    } else {
        println!("❌ Request failed: {}", resp.status());
        let text = resp.text().await?;
        println!("   Body: {}", text);
    }

    Ok(())
}
