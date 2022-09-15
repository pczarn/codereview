use std::io;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::num::ParseFloatError;

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut height = String::new();
    let mut weight = String::new();
 
    println!("Kalkulator BMI, witamy!");
    thread::sleep(Duration::from_secs(1));
 
    print!("Wpisz swój wzrost: ");
    io::stdout().flush().unwrap(); // Make input in same line as print
    io::stdin().read_line(&mut height).context("Failed to read line")?;
    let height: f32 = height.trim().parse()?;
 
    print!("Wpisz teraz swoją wagę: ");
    io::stdout().flush().unwrap(); // Make input in same line as print
    io::stdin().read_line(&mut weight).context("Failed to read line")?;
    let weight: f32 = weight.trim().parse()?;
 
    let bmi: f32 = weight / (height / 100.0).powf(2.0) as f32;
    println!("Twoje BMI: {bmi:.1}");
    // zwracamy sukces
    Ok(())
}