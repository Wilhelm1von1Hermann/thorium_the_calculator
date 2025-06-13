use std::io::{stdin, stdout, Write};

// constants for text formatting
pub const STYLE_BOLD: &str = "\x1B[1m";
pub const STYLE_RESET: &str = "\x1B[0m";

enum ExitTypeEnum {
    Error,
    Normal
}

pub struct ExitType {
    r#type: ExitTypeEnum,
    exit_code: u32
}

impl ExitType {
    pub fn error(exit_code: u32) -> Self {
        let result = ExitType {
            r#type: ExitTypeEnum::Error,
            exit_code
        };
        return result;
    }

    pub fn normal() -> Self {
        let result = ExitType {
            r#type: ExitTypeEnum::Normal,
            exit_code: 0
        };
        return result;
    }
}

pub enum DebugValue {
    Yes(i32),
    No
}

impl DebugValue {
    pub fn new(num: Option<i32>) -> Self {
        if num != None {
            let enumeration = DebugValue::Yes(num.unwrap());
            return enumeration;
        } else {
            return DebugValue::No;
        }
    }

    pub fn print_debugval(&self) {
        match self {
            DebugValue::Yes(num) => {
                println!("Today's debug value: {num}");
                silenthold();
            }
            DebugValue::No => ()
        }
    }
}

// example usage: printf("Hello, " + "World!");
pub fn printf(string: &str) {
    let mut lock = stdout().lock();
    lock.write_all(string.as_bytes()).unwrap();
    lock.write(b"\n").unwrap();
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
    match std::io::stdout().flush() {
        Ok(()) => (),
        Err(_) => {
            println!("ERROR: Failed to flush");
            ()
        }
    }
    silenthold();
    exit(ExitType::error(1));
}

pub fn exit(exittype: ExitType) {
    match exittype.r#type { // r#type means 'type' as identifier, not keyword
        ExitTypeEnum::Error => {
            std::process::exit(exittype.exit_code as i32);
        }
        ExitTypeEnum::Normal => {
            std::process::exit(0);
        }
    }
}

pub fn print_result(result: &str) {
    println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
}
