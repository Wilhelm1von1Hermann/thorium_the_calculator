mod math;
mod misc;
mod interface;

mod trashbin;

fn main() {
    misc::screenclear();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        if args[1] == String::from("-legacy") {
            println!("ENTERING LEGACY MODE: not supported");
            misc::screenhold();
            trashbin::calculator_legacy();
        }
    }

    let debug_value = misc::DebugValue::new(None);

    debug_value.print_debugval();

    misc::screenclear();
    interface::calculator();
}
