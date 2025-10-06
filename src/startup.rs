use std::io::{self, Write};
use std::thread;
use std::time::Duration;

// Alternative: Animated startup
pub fn starting_up_animated() {
    clearscreen::clear().expect("failed to clear screen");
    
    let lines = vec![
                    r"████                              ",
                   r"░░███                              ",
  r"██████   ██████   ░███             ████████   █████ ",
 r"███░░███ ░░░░░███  ░███  ██████████░░███░░███ ███░░  ",
r"░███ ░░░   ███████  ░███ ░░░░░░░░░░  ░███ ░░░ ░░█████ ",
r"░███  ███ ███░░███  ░███             ░███      ░░░░███",
r"░░██████ ░░████████ █████            █████     ██████ ",
 r"░░░░░░   ░░░░░░░░ ░░░░░            ░░░░░     ░░░░░░  ",                                              
                                                                                                
    ];
    
    // Animate each line
    for line in &lines {
        print!("{}", colorize(line, Color::White));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(80));
        println!();
    }
    
    thread::sleep(Duration::from_millis(200));
    
    // Typewriter effect for tagline
    let tagline = "  A powerful expression calculator with tree visualization";
    for ch in tagline.chars() {
        print!("{}", colorize(&ch.to_string(), Color::Yellow));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }
    println!("\n");
    
    thread::sleep(Duration::from_millis(300));
}

// Color helper (works in most terminals)
#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn colorize(text: &str, color: Color) -> String {
    let code = match color {
        Color::Red => "31",
        Color::Green => "32",
        Color::Yellow => "33",
        Color::Blue => "34",
        Color::Magenta => "35",
        Color::Cyan => "36",
        Color::White => "37",
    };
    format!("\x1b[{}m{}\x1b[0m", code, text)
}

// Optional: Progress bar for "loading" effect
pub fn show_loading() {
    print!("  Loading");
    io::stdout().flush().unwrap();
    
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(300));
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!(" {}", colorize("Done!", Color::Green));
    thread::sleep(Duration::from_millis(200));
}
