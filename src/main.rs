mod math;
mod misc;
mod interface;

fn main() {
    misc::screenclear();

    let mut debug_value: misc::DebugValue = misc::DebugValue::new(None);

    debug_value.print_debug();

    misc::screenclear();
    interface::calculator();
}
