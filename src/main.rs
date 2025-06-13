mod math;
mod misc;
mod interface;

fn main() {
    misc::screenclear();

    let debug_value = misc::DebugValue::new(None);

    debug_value.print_debug();

    misc::screenclear();
    interface::calculator();
}
