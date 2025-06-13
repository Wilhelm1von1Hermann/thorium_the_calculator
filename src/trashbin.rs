// for storing all of the obsolete, discountinued and unnecessary functions

// original calculator_legacy() from the main.rs
// to be used in the legacy mode

use std::io::{Write, stdin};
use std::time::Instant;
use crate::misc;
use crate::misc::*;
use crate::math;

pub fn calculator_legacy() {
    misc::screenclear();

    print!(
        "Current list: + for plus, - for minus, * for multiple, / for divide, 1 for more options, 0 to EXIT\n"
    );
    print!("Select operation > ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    stdin().read_line(&mut input).expect("Invalid input");
    let input = input.trim();

    match input {
        "+" => {
            println!("Plus selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse().expect("Invalid input");

            let result = math::mathematical_operation(num1, num2, "plus".to_string());
            println!("\n{STYLE_BOLD}{num1} + {num2} = {result}{STYLE_RESET}\n");

            misc::screenhold();
            calculator_legacy();
        }
        "-" => {
            println!("Minus selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "minus".to_string());
            println!("\n {STYLE_BOLD} {num1} - {num2} = {result} {STYLE_RESET} \n");

            misc::screenhold();
            calculator_legacy();
        }
        "*" => {
            println!("Multiplication selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "multiple".to_string());
            println!("\n {STYLE_BOLD} {num1} * {num2} = {result} {STYLE_RESET} \n");

            misc::screenhold();
            calculator_legacy();
        }
        "/" => {
            println!("Division selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "divide".to_string());
            println!("\n {STYLE_BOLD} {num1} / {num2} = {result} {STYLE_RESET} \n");

            misc::screenhold();
            calculator_legacy();
        }
        "sq" | "SQ" => {
            println!("Number squaring selected. Enter your number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1: i32 = num1.trim().parse().expect("Invalid input");

            let perf_now = Instant::now();
            let result = math::number_square(num1);
            let perf_elapsed = perf_now.elapsed();

            println!("\n {STYLE_BOLD} {num1} squared is {result} {STYLE_RESET} \n");
            println!("Calculation time: {perf_elapsed:.2?}");

            misc::screenhold();
            calculator_legacy();
        }
        "sqrt" | "SQRT" => {
            println!("Square root selected. This operator only supports integers as an input.");

            stdin().read_line(&mut num1).expect("Float?");
            let num1: u64 = num1.trim().parse().expect("Entered string");

            let perf_now = Instant::now();
            let result = math::sqrt_pow(num1 as f32);
            let perf_elapsed = perf_now.elapsed();

            println!("\n {STYLE_BOLD} Square root of {num1} is {result} {STYLE_RESET}");
            println!("Calculation time: {STYLE_BOLD}{perf_elapsed:.2?}{STYLE_RESET}");

            misc::screenhold();
            calculator_legacy();
        }
        "c" | "C" => {
            println!("Custom operation selected. ");
            println!("Enter operation length: ");
            let mut operation_length = String::new();
            stdin()
                .read_line(&mut operation_length)
                .expect("invalid input");

            let mut operators: Vec<String> = Vec::new();
            println!("Enter all operators: ");
            loop {
                let mut input = String::new();
                println!("Do you want to continue? [Y/N]");
                stdin().read_line(&mut input).expect("ERROR: Invalid input");
                let input = input.trim().to_string();

                if input == "Y" {
                    let mut temp_operator: String = String::new();
                    stdin()
                        .read_line(&mut temp_operator)
                        .expect("ERROR: Invalid input");
                    let temp_operator = temp_operator.trim().to_string();
                    operators.push(temp_operator);
                } else {
                    // TODO
                }
            }
        }
        "f" | "F" => {
            // let ratio: f32 = { 5.0 / 9.0 };
            let mut input = String::new();
            println!("Enter temperature in Celsius: ");
            stdin().read_line(&mut input).expect("Invalid input");
            let input: f32 = input.trim().parse().expect("Invalid input");

            let result: f32 = { input * 1.8 + 32.0 };

            println!("{STYLE_BOLD} {input}°C is {result}°F {STYLE_RESET}");

            misc::screenhold();
            calculator_legacy();
        }
        "cr" | "CR" => {
            // circle operations
            misc::screenclear();
            println!("Circle operations selected. 1 for radius to area, 2 for area to radius.");
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Invalid input");
            let input = input.trim();

            match input {
                "1" => {
                    // Radius to area
                    println!("Radius to area selected. Enter radius:");
                    let mut input = String::new();

                    stdin().read_line(&mut input).expect("Input error");
                    let input: f32 = input.trim().parse().expect("Parse error");

                    let result = math::circle_calculation(input, math::CircleCalcType::radius_to_area());
                    println!("An area of an circle with the radius of {input} units is {result} units squared");

                    misc::screenhold();
                    misc::screenclear();
                    calculator_legacy();
                }
                "2" => {
                    // Area to radius
                    println!("Area to radius selected. Enter area:");
                    let mut input = String::new();

                    stdin().read_line(&mut input).expect("Input error");
                    let input: f32 = input.trim().parse().expect("Parse error");

                    let result = math::circle_calculation(input, math::CircleCalcType::area_to_radius());
                    println!("An radius of an circle with the area of {input} units squared is {result} units");

                    misc::screenhold();
                    misc::screenclear();
                    calculator_legacy();
                }
                _ => {
                    println!("Couldn't find that operation.");
                    misc::silenthold();
                    misc::screenclear();
                    calculator_legacy();
                }
            }
        }
        "pow" | "POW" => {
            misc::screenclear();
            println!("Exponent operator selected. Enter your value:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1: f32 = num1.trim().parse().expect("error parsing"); // number
            println!("Enter your power:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2: u32 = num2.trim().parse().expect("error parsing"); // power
            let result = math::power(num1, num2);
            println!("{STYLE_BOLD}{num1} to the power of {num2} is {result} {STYLE_RESET}");

            misc::screenhold();
            calculator_legacy();
        }
        "sin" | "SIN" => {
            misc::screenclear();

            println!("Sine calculation selected. Enter your value in degrees:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1: f32 = num1.trim().parse().expect("error parsing");
            let result = math::sin(num1);

            println!("{STYLE_BOLD}Sin of {num1} is {result} {STYLE_RESET}");

            misc::screenhold();
            calculator_legacy();
        }
        "sind" | "SIND" => {
            misc::screenclear();
            println!("Sine rendering selected.");
            misc::screenhold();

            let con_vec = math::sine_wave_render();

            for sign in con_vec {
                print!("{}", sign);
                std::io::stdout().flush().unwrap();
            }

            misc::silenthold();
            calculator_legacy();
        }
        "rend" | "REND" => {
            // used for rendering different stuff
            let mut input = String::new();

            println!(
                "
                \nRender mode selected. Select one of the options:
                \n1 - FULL, 2 - CUBE, 3 - TRIANGLE
                "
            );
            stdin().read_line(&mut input).expect("Invalid input");
            let input: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Error parsing that input.");
                    misc::screenhold();
                    calculator_legacy();
                    1
                }
            };

            match input {
                1 => {
                    let perf_now = Instant::now();
                    let console_vec = math::full_render();

                    misc::screenclear();
                    // rendering begins

                    for chr in &console_vec {
                        print!("{chr}");
                        std::io::stdout().flush().expect("flush fail");
                    }
                    let perf_elapsed = perf_now.elapsed();

                    misc::silenthold();
                    misc::screenclear();

                    println!("Calculation time: {perf_elapsed:.2?}");
                    println!("Total vector length: {}", console_vec.len());
                    misc::screenhold();
                    misc::screenclear();

                    calculator_legacy();
                }
                2 => {
                    println!("Hasn't been implemented yet.");
                    misc::screenhold();
                    calculator_legacy();
                }
                3 => {
                    // render simple triangle
                    misc::screenclear();

                    let con_vec = math::triangle_render();

                    for sign in con_vec {
                        print!("{sign}");
                        std::io::stdout().flush().expect("flush fail");
                    }

                    misc::silenthold();
                    calculator_legacy();
                }
                _ => {
                    println!("Option not found.");
                    misc::screenhold();
                    calculator_legacy();
                }
            }
        }
        "rand" | "RAND" => {
            println!("Sorry! Currently under maintenance!");
            misc::screenhold();
            calculator_legacy();
        }
        "time" | "TIME" => {
            misc::screenclear();
            println!("Timer selected. It will run at 60 FPS");
            misc::screenhold();

            let fps = 24.0; // high fps can cause Some(problem)
            let delta_time: f32 = 1000.0 / fps;

            let mut millisecs_count = 0.0;

            loop {
                misc::screenclear();
                millisecs_count += delta_time;
                println!("Timer: {millisecs_count} ms");
                std::thread::sleep(std::time::Duration::from_millis(delta_time as u64));
            }
        }
        "1" => {
            misc::screenclear();
            print!(
"More options: sq for squaring, c for custom operation, f for celsius to fahrenreit
v for vectors, sqrt for square root, t0-t9 for learning rust things
cr for circle operations, pow for exponent, sin for sine calculation
sind to draw a sine to the console, rend for rendering mode
rand for a random number, time for timer \n"
            );
            std::io::stdout().flush().expect("flush fail");
            misc::screenhold();
            calculator_legacy();
        }
        "clear" => {
            misc::screenclear();
            calculator_legacy();
        }
        "0" | "stop" | "exit" => {
            misc::silenthold();
            misc::exit(ExitType::normal());
        }
        _ => {
            println!("Unknown operation");
            misc::screenhold();
            calculator_legacy();
        }
    }
}
