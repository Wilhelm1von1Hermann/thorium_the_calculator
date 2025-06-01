use std::io::stdin;
use std::io::Write;

pub enum ExitType {
    Error,
    Normal
}

pub fn screenhold() {
    // holds current text with text
    println!("Press Enter to continue...");
    stdin()
        .read_line(&mut String::new())
        .expect("Invalid input.");
}

pub fn silenthold() {
    // holds current text silently
    stdin()
        .read_line(&mut String::new())
        .expect("Invalid input");
}

pub fn screenclear() {
    // clears out the text
    print!("{esc}c", esc = 27 as char);
}

pub fn thorium_panic(message: String) {
    print!("\n:(\nERROR: {message}");
    std::io::stdout().flush().expect("error error, you're cooked");
    silenthold();
    exit(ExitType::Error);
}

pub fn exit(exittype: ExitType) {
    match exittype {
        ExitType::Error => {
            std::process::exit(1);
        }
        ExitType::Normal => {
            std::process::exit(0);
        }
    }
}

// constants for text formatting
pub const style_bold: &str = "\x1B[1m";
pub const style_reset: &str = "\x1B[0m";
