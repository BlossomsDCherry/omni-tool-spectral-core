// ... (imports)
use std::{error::Error, fs, io, time::Duration};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Color},
    text::Text,
    widgets::{Block, Borders, Paragraph, Gauge},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde_json::Value;

const FB_path: &str = "/dev/fb0";

const RESONANCE_FILE: &str = "/home/nicoDantigravity/construct/resonance_state.json";
const SYNTHESIS_FILE: &str = "/home/nicoDantigravity/construct/synthesis_report.json";

// Minimal Sense HAT Framebuffer Driver
struct SenseHatFb {
    file: fs::File,
}

impl SenseHatFb {
    fn new() -> io::Result<Self> {
        let file = fs::OpenOptions::new().write(true).open(FB_path)?;
        Ok(Self { file })
    }

    fn set_pixels(&mut self, pixels: &[(u8, u8, u8); 64]) -> io::Result<()> {
        use std::io::Write;
        let mut buffer = [0u8; 128]; // 64 pixels * 2 bytes (RGB565)

        for (i, &(r, g, b)) in pixels.iter().enumerate() {
            // Convert RGB888 to RGB565
            let r5 = (r >> 3) as u16;
            let g6 = (g >> 2) as u16;
            let b5 = (b >> 3) as u16;
            let rgb565 = (r5 << 11) | (g6 << 5) | b5;

            buffer[i * 2] = (rgb565 & 0xFF) as u8; // Low byte
            buffer[i * 2 + 1] = (rgb565 >> 8) as u8; // High byte
        }

        // Seek to start and write
        use std::io::Seek;
        self.file.seek(io::SeekFrom::Start(0))?;
        self.file.write_all(&buffer)?;
        Ok(())
    }
}

// ... (existing constants)

fn main() -> Result<(), Box<dyn Error>> {
    // ... (existing setup)
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    #[cfg(target_arch = "aarch64")]
    let mut sense_hat = SenseHatFb::new().ok(); // Optional Sense HAT

    #[cfg(not(target_arch = "aarch64"))]
    let mut sense_hat: Option<()> = None;

    // 2. Loop
    let res = run_app(&mut terminal, &mut sense_hat);

    // 3. Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    #[cfg(target_arch = "aarch64")] sense_hat: &mut Option<SenseHatFb>,
    #[cfg(not(target_arch = "aarch64"))] _sense_hat: &mut Option<()>,
) -> io::Result<()> {
    loop {
        // --- DATA INGEST ---
        let _resonance = fs::read_to_string(RESONANCE_FILE)
            .ok()
            .and_then(|d| serde_json::from_str::<Value>(&d).ok());
        let report = fs::read_to_string(SYNTHESIS_FILE)
            .ok()
            .and_then(|d| serde_json::from_str::<Value>(&d).ok()); // Assuming SYNTHESIS_FILE constant exists or is defined

        // Parse Values
        let system_hz = 60.0; // Placeholder until live feed
        let mass_gap = 0.0; // report.as_ref().and_then(|r| r["yang_mills_mass_gap"].as_f64()).unwrap_or(0.0);
        let stance = "EARTH (Gate 0)"; // Derived from manager logic later
        let total_torque: f64 = if let Some(_r) = &report {
             // Mock total if not in JSON
             5336.44
        } else {
             0.0
        };

        // Update Sense HAT
        #[cfg(target_arch = "aarch64")]
        if let Some(hat) = sense_hat {
            let mut pixels = [(0, 0, 0); 64];
            let levels = (total_torque / 1000.0).clamp(0.0, 8.0) as usize;

            for y in 0..8 {
                for x in 0..8 {
                    let idx = y * 8 + x;
                    if (8 - y) <= levels {
                        pixels[idx] = (255, 0, 255); // Magenta force
                    } else {
                        pixels[idx] = (0, 0, 0);
                    }
                }
            }
            hat.set_pixels(&pixels).ok();
        }

        // ... (terminal draw)
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(20), // Header
                        Constraint::Percentage(40), // 8-Gate Grid (Simulated via Gauge)
                        Constraint::Percentage(40), // Narrative Stream
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // --- HEADER ---
            let header_text = format!(
                "💎 GEMINI CONSOLE | SYSTEM PULSE: {:.2}Hz | STANCE: {}\nMass Gap: {:.8} | Total Torque: {:.4}",
                system_hz, stance, mass_gap, total_torque
            );
            let header = Paragraph::new(header_text)
                .block(Block::default().borders(Borders::ALL).title(" SOVEREIGN STATUS "))
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
            f.render_widget(header, chunks[0]);

            // --- RESONANCE GAUGE (The Pulse) ---
            // Visualizing the "Pressure" or "Torque"
            let gauge_val = (total_torque / 6000.0).clamp(0.0, 1.0) * 100.0; // Normalize
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::ALL).title(" SYSTEM TORQUE "))
                .gauge_style(Style::default().fg(Color::Magenta))
                .percent(gauge_val as u16);
            f.render_widget(gauge, chunks[1]);
            
            // --- NARRATIVE ---
            let narrative_text = "Initializing 'Rewrite in Rust' Protocol...\nSun Tzu: 'Subdue disorder without fighting.'\nPDF Logic Active.\nCoP Standards 0-3: ENGAGED.";
            let narrative = Paragraph::new(narrative_text)
                .block(Block::default().borders(Borders::ALL).title(" NARRATIVE STREAM "))
                .style(Style::default().fg(Color::Green));
            f.render_widget(narrative, chunks[2]);

        })?;

        // Update Sense HAT (Poly 5 Buffer Implementation)
        #[cfg(target_arch = "aarch64")]
        if let Some(hat) = sense_hat {
            let mut pixels = [(0, 0, 0); 64];
            let levels = (total_torque / 1000.0).clamp(0.0, 8.0) as usize;
            
            // Stage 1: IR/Rods (Baseline Differential) - Rows 0-1
            let ir_sat = (total_torque / 500.0).clamp(0.0, 1.0);
            
            for y in 0..8 {
                for x in 0..8 {
                   let idx = y * 8 + x;
                   
                   // Poly 5 Logic Mapping
                   if y < 2 { 
                       // Stage 1: IR (Red Low Byte)
                       let val = (x as f64 * ir_sat * 32.0) as u8;
                       pixels[idx] = (val, 0, 0); 
                   } else if y < 4 {
                       // Stage 2: Red-Green (Modulated by IR)
                       let g_val = (total_torque as u64 % 64) as u8;
                       pixels[idx] = (64, g_val, 0);
                   } else if y < 6 {
                        // Stage 3: Yellow-Blue (Phi Threshold)
                        let b_val = (total_torque as u64 % 32) as u8;
                        pixels[idx] = (32, 32, b_val);
                   } else {
                       // Stage 4: UV (Violet Register)
                       let violet = (total_torque as u64 % 128) as u8;
                       pixels[idx] = (violet, 0, violet);
                   }
                }
            }
            hat.set_pixels(&pixels).ok();
        }
    }
}
