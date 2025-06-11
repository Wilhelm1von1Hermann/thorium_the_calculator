// rewrite of the calculator() from main.rs
// shall use geneva rust

use crate::misc::*;
use crate::math::*;

use std::io::{stdout, stdin, Write};

// builds count
include!(concat!(env!("OUT_DIR"), "/build_num.rs"));

pub fn calculator() {
    // does not include the screenclear(), you have to do it yourself
    // should also include better error handling
    // and maybe some serious logic changes

    // TODO: modular system for these commands

    let mut failed_count = 0;
    const HELP_STRING: &str = "Current list: + for plus, - for minus, * for multiple, / for divide, more for more options, exit to EXIT";
    const MORE_OPTIONS_STRING: &str = "More options: \
        sq for squaring, cop for custom operation, c2f for celsius to fahrenreit \
        \nvec for vectors, sqrt for square root, t0-t9 for rust by practice things \
        \ncirc for circle operations, exp for exponent, sin for sine calculation \
        \nrend for rendering mode, rand for random number, time for timer
    ";

    loop {

        print!("$ > ");
        stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(input) => (),
            Err(err) => {
                println!("Couldn't read input: {err}");
            }
        }

        // example: ["math", "add", "5", "12"]
        let args: Vec<&str> = input.trim().split_whitespace().collect();

        if args.is_empty() {
            continue;
        }

        match args[0].to_lowercase().as_str() {
            // math commands

            "+" => {

                if args.len() != 3 {
                    println!("Usage: + a b");
                    continue;
                }

                let num1: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing first number: {err}");
                        continue;
                    }
                };

                let num2: f32 = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing second number: {err}");
                        continue;
                    }
                };

                let result = mathematical_operation(num1, num2, "plus".to_string());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "-" => {
                if args.len() != 3 {
                    println!("Usage: - a b");
                    continue;
                }

                let num1: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing first number: {err}");
                        continue;
                    }
                };

                let num2: f32 = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing second number: {err}");
                        continue;
                    }
                };

                let result = mathematical_operation(num1, num2, "minus".to_string());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "*" => {
                if args.len() != 3 {
                    println!("Usage: * a b");
                    continue;
                }

                let num1: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing first number: {err}");
                        continue;
                    }
                };

                let num2: f32 = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing second number: {err}");
                        continue;
                    }
                };

                let result = mathematical_operation(num1, num2, "multiply".to_string());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "/" => {
                if args.len() != 3 {
                    println!("Usage: / a b");
                    continue;
                };

                let num1: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing first number: {err}");
                        continue;
                    }
                };

                let num2: f32 = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing second number: {err}");
                        continue;
                    }
                };

                if num2 == 0.0 {
                    println!("Error: Couldn't divide by zero");
                    continue;
                }

                let result = mathematical_operation(num1, num2, "divide".to_string());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "sq" => {
                if args.len() != 2 {
                    println!("Usage: sq n");
                    continue;
                };

                let num: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                if num > 65535. || num <= 1. {
                    println!("Number range: 1 < n < 65535");
                    continue;
                };

                let result = number_square(num as i32);

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "c2f" => {
                if args.len() != 2 {
                    println!("Usage: c2f n");
                    continue;
                };

                let num: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                if num < -273.15 {
                    println!("Number is below absolute zero in Celsius.");
                    continue;
                };

                let result: f32 = celsius_to_fahrenreit(num);

                println!("Result: {STYLE_BOLD}{result}°F{STYLE_RESET}");
                continue;
            }
            "sqrt" => {
                // hopefully will work with negative numbers
                if args.len() != 2 {
                    println!("Usage: sqrt n");
                    continue;
                };

                let num: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                let result: f32 = sqrt_pow(num);

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "r2a" => {
                // radius to area

                if args.len() != 2 {
                    println!("Usage: r2a n");
                    continue;
                };

                let num: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                let result: f32 = circle_calculation(num, CircleCalcType::radius_to_area());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "a2r" => {
                // area to radius

                if args.len() != 2 {
                    println!("Usage: a2r n");
                    continue;
                };

                let num: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                let result: f32 = circle_calculation(num, CircleCalcType::area_to_radius());

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }
            "exp" => {
                if args.len() != 3 {
                    println!("Usage: exp *number* *power*");
                    continue;
                };

                let number: f32 = match args[1].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing number: {err}");
                        continue;
                    }
                };

                let power_n: u32 = match args[2].trim().parse() {
                    Ok(num) => num,
                    Err(err) => {
                        println!("Error parsing power: {err}");
                        continue;
                    }
                };

                // power_n + 1 is necessary balancing strategy
                let result: f32 = power(number, power_n + 1);

                println!("Result: {STYLE_BOLD}{result}{STYLE_RESET}");
                continue;
            }

            // misc commands

            "clear" => {
                screenclear();
                continue;
            }
            "help" => {
                println!("{HELP_STRING}");
                stdout().flush().unwrap();
                continue;
            }
            "build" => {
                println!("Thorium's build is build {BUILD_NUMBER}");
                continue;
            }
            // 1 and 0 are remains of the past
            "1" | "more" => {
                println!("{MORE_OPTIONS_STRING}");
                stdout().flush().unwrap();
                continue;
            }
            "0" | "exit" => {

                screenclear();
                exit(ExitType::normal());
            }

            // obsolete commands

            "cr" | "circ" => {
                println!("Obsolete. Use 'r2a' for radius to area or 'a2r' for area to radius");
                continue;
            }
            "pow" => {
                println!("Obsolete. Use 'exp' for the same results.");
                continue;
            }

            // wildcard
            _ => {
                failed_count += 1;
                if failed_count == 5 {
                    println!("Can you stop?");
                    continue;
                }
                println!("Command not found.");
                continue;
            }
        }
    }

}
