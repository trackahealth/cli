use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("🩺 Welcome to TrackaHealth – Offline Vitals Logger");
    println!("------------------------------------------------");

    // Simulate vitals
    let heart_rate = simulate_heart_rate();
    let temperature = simulate_temperature();
    let breathing_rate = simulate_breathing_rate();
    let (systolic, diastolic) = simulate_blood_pressure();

    // Show results
    println!("💓 Heart Rate: {} bpm", heart_rate);
    println!("🌡️ Temperature: {:.1}°C", temperature);
    println!("🌬️ Breathing Rate: {} breaths/min", breathing_rate);
    println!("💉 Blood Pressure: {}/{} mmHg", systolic, diastolic);

    // Save to file
    if let Err(e) = save_vitals(heart_rate, temperature, breathing_rate, systolic, diastolic) {
        eprintln!("❌ Failed to save vitals: {}", e);
    } else {
        println!("✅ Vitals saved to 'vitals_log.txt'");
    }
}

// Simulate heart rate between 60 - 100 bpm
fn simulate_heart_rate() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    ((now % 41) + 60) as u32
}

// Simulate temperature between 36.5 - 37.5°C
fn simulate_temperature() -> f32 {
    let now_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    let fractional = (now_nanos % 1000) as f32 / 1000.0;
    36.5 + fractional
}

// Simulate breathing rate between 12 - 20 breaths per minute
fn simulate_breathing_rate() -> u32 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u32;

    now % 9 + 12
}

// Simulate blood pressure between 90/60 to 140/90 mmHg
fn simulate_blood_pressure() -> (u32, u32) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as u32;

    let systolic = 90 + (now % 51);   // 90 - 140 mmHg
    let diastolic = 60 + (now % 31); // 60 - 90 mmHg

    (systolic, diastolic)
}

// Save vitals to local file
fn save_vitals(
    heart_rate: u32,
    temperature: f32,
    breathing_rate: u32,
    systolic: u32,
    diastolic: u32,
) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_entry = format!(
        "[{}] Heart Rate: {} bpm, Temperature: {:.1}°C, Breathing Rate: {} breaths/min, Blood Pressure: {}/{} mmHg\n",
        timestamp, heart_rate, temperature, breathing_rate, systolic, diastolic
    );

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("vitals_log.txt")?;

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}
