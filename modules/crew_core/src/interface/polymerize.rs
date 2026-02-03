use anyhow::Result;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize)]
struct Interaction {
    body: String,
    ontology: Option<String>,
}

pub fn synthesize() -> Result<()> {
    println!("🧪 Initiating Cognitive Polymerization...");

    let compost_path = "../saucy compost/interactions.jsonl";
    let file = File::open(compost_path)?;
    let reader = BufReader::new(file);

    let mut word_counts: HashMap<String, usize> = HashMap::new();
    let mut access_count = 0;

    // 1. Analyze "Access" Data
    for line in reader.lines() {
        let line = line?;
        if let Ok(interaction) = serde_json::from_str::<Interaction>(&line) {
            if interaction.ontology.as_deref() == Some("access") {
                access_count += 1;
                // Simple tokenization
                for word in interaction.body.split_whitespace() {
                    let clean_word = word
                        .to_lowercase()
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>();

                    if clean_word.len() > 4 {
                        // Filter noise
                        *word_counts.entry(clean_word).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    if access_count == 0 {
        println!("  No Access data to polymerize.");
        return Ok(());
    }

    // 2. Identify Patterns (The "Polymer")
    let mut top_words: Vec<_> = word_counts.into_iter().collect();
    top_words.sort_by(|a, b| b.1.cmp(&a.1));

    let top_5: Vec<String> = top_words
        .into_iter()
        .take(5)
        .map(|(w, c)| format!("{} ({})", w, c))
        .collect();
    let insight = format!(
        "Polymerization Complete. Dominant themes in Access layer: {}",
        top_5.join(", ")
    );

    println!("  Insight Generated: {}", insight);

    // 3. Generate "Equity" Event
    let equity_event = json!({
        "sender": "System",
        "role": "Polymerizer",
        "body": insight,
        "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        "verified": true,
        "ontology": "equity" // This balances the equation
    });

    // 4. Write to Compost
    let mut append_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(compost_path)?;

    writeln!(append_file, "{}", serde_json::to_string(&equity_event)?)?;

    println!("🧪 Equity injected into Compost.");
    Ok(())
}
