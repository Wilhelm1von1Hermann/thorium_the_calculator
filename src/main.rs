use std::io::Write;

// 0% of the modern code still uses lowercase
// todo: make the code use uppercase rust-standard constants
use crate::misc::STYLE_BOLD as style_bold;
use crate::misc::STYLE_RESET as style_reset;

mod math;
mod misc;
mod interface;

// builds count
include!(concat!(env!("OUT_DIR"), "/build_num.rs"));

fn main() {
    misc::screenclear();

    print!("Welcome to the Thorium build {BUILD_NUMBER}!\n");

    std::io::stdout().flush().unwrap();

    // debug value operations

    let mut debug_value: Option<usize>;

    // FOR DEBUG VALUE: use 'debug_value = *whatever*'

    debug_value = None;

    if debug_value != None {
        let debug_value_unwrapped = debug_value.unwrap();
        println!("Today's debug value: {debug_value_unwrapped}");
    } else {
        println!("Today's debug value: None");
    }

    misc::silenthold();

    misc::screenclear();
    interface::calculator();
}
