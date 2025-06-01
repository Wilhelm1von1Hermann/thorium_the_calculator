use std::io::stdin;
use std::io::{Read, Write};
use std::time::Instant;

use crate::misc::{
    style_bold, style_reset, ExitType
};

mod math;
mod misc;

fn main() {
    misc::screenclear();

    print!("Welcome to the Thorium!");
    std::io::stdout().flush().unwrap();
    print!("\nToday's debug value: None");
    std::io::stdout().flush().unwrap();
    misc::silenthold();
    calculator();
}

fn calculator() {
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
            println!("\n{style_bold}{num1} + {num2} = {result}{style_reset}\n");

            misc::screenhold();
            calculator();
        }
        "-" => {
            println!("Minus selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "minus".to_string());
            println!("\n {style_bold} {num1} - {num2} = {result} {style_reset} \n");

            misc::screenhold();
            calculator();
        }
        "*" => {
            println!("Multiplication selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "multiple".to_string());
            println!("\n {style_bold} {num1} * {num2} = {result} {style_reset} \n");

            misc::screenhold();
            calculator();
        }
        "/" => {
            println!("Division selected. Enter first number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1 = num1.trim().parse::<f32>().expect("Invalid input");
            println!("Enter second number:");
            stdin().read_line(&mut num2).expect("Invalid input");
            let num2 = num2.trim().parse::<f32>().expect("Invalid input");

            let result: f32 = math::mathematical_operation(num1, num2, "divide".to_string());
            println!("\n {style_bold} {num1} / {num2} = {result} {style_reset} \n");

            misc::screenhold();
            calculator();
        }
        "sq" | "SQ" => {
            println!("Number squaring selected. Enter your number:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1: i32 = num1.trim().parse().expect("Invalid input");

            let perf_now = Instant::now();
            let result = math::number_square(num1);
            let perf_elapsed = perf_now.elapsed();

            println!("\n {style_bold} {num1} squared is {result} {style_reset} \n");
            println!("Calculation time: {perf_elapsed:.2?}");

            misc::screenhold();
            calculator();
        }
        "sqrt" | "SQRT" => {
            println!("Square root selected. This operator only supports integers as an input.");

            stdin().read_line(&mut num1).expect("Float?");
            let num1: u64 = num1.trim().parse().expect("Entered string");

            let perf_now = Instant::now();
            let result = math::sqrt_pow(num1 as f32);
            let perf_elapsed = perf_now.elapsed();

            println!("\n {style_bold} Square root of {num1} is {result} {style_reset}");
            println!("Calculation time: {style_bold}{perf_elapsed:.2?}{style_reset}");

            misc::screenhold();
            calculator();
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

            println!("{style_bold} {input}°C is {result}°F {style_reset}");

            misc::screenhold();
            calculator();
        }
        "t0" | "T0" => {
            // text editor project
            use std::fs;

            let working_dir = "C:/Users/ma200/Desktop/test.txt"; // set desktop as the working dir
            let mut input = String::new();
            let mut file =

            misc::screenclear();

            println!("t0 is currently reserved for text editor project.");
            println!("Enter string to place in the file 'text.txt' at the desktop:");
            stdin().read_line(&mut input).expect("big error");

            fs::write("{working_input}", input).expect("big error with the file");
            println!("Writing complete");

            misc::screenhold();
            misc::screenclear();
            calculator();
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

                    let result = math::circle_calculation(input, math::CircleCalcType::RadiusToArea);
                    println!("An area of an circle with the radius of {input} units is {result} units squared");

                    misc::screenhold();
                    misc::screenclear();
                    calculator();
                }
                "2" => {
                    // Area to radius
                    println!("Area to radius selected. Enter area:");
                    let mut input = String::new();

                    stdin().read_line(&mut input).expect("Input error");
                    let input: f32 = input.trim().parse().expect("Parse error");

                    let result = math::circle_calculation(input, math::CircleCalcType::RadiusToArea);
                    println!("An radius of an circle with the area of {input} units squared is {result} units");

                    misc::screenhold();
                    misc::screenclear();
                    calculator();
                }
                _ => {
                    println!("Couldn't find that operation.");
                    misc::silenthold();
                    misc::screenclear();
                    calculator();
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
            println!("{style_bold}{num1} to the power of {num2} is {result} {style_reset}");

            misc::screenhold();
            calculator();
        }
        "sin" | "SIN" => {
            misc::screenclear();

            println!("Sine calculation selected. Enter your value in degrees:");
            stdin().read_line(&mut num1).expect("Invalid input");
            let num1: f32 = num1.trim().parse().expect("error parsing");
            let result = math::sin(num1);

            println!("{style_bold}Sin of {num1} is {result} {style_reset}");

            misc::screenhold();
            calculator();
        }
        "sind" | "SIND" => {
            // draw a sine to the console

            misc::screenclear();
            println!("Unfortunately, this operation has not been implemented yet.");
            misc::screenhold();
            calculator();
        }
        "rend" | "REND" => {
            // used for rendering different stuff
            let mut input = String::new();
            let console_height = 30;
            let console_width = 120;
            let console_sign: char = '#';

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
                    calculator();
                    1
                }
            };

            match input {
                1 => {
                    let perf_now = Instant::now();
                    let console_vec = math::render_function(math::RenderType::FULL);

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

                    calculator();
                }
                2 => {
                    println!("Hasn't been implemented yet.");
                    misc::screenhold();
                    calculator();
                }
                3 => {
                    // render simple triangle
                    misc::screenclear();

                    let con_vec = math::render_triangle();

                    for sign in con_vec {
                        print!("{sign}");
                        std::io::stdout().flush().expect("flush fail");
                    }

                    misc::silenthold();
                    calculator();
                }
                _ => {
                    println!("Option not found.");
                    misc::screenhold();
                    calculator();
                }
            }
        }
        "rand" | "RAND" => {
            misc::screenclear();

            let random_number = math::random_number();
            println!("RAND number is {style_bold}{random_number}{style_reset}");

            misc::screenhold();
            calculator();
        }
        "1" => {
            misc::screenclear();
            print!(
                "
                More options: sq for squaring, c for custom operation, f for celsius to fahrenreit
                v for vectors, sqrt for square root, t0-t9 for learning rust things
                cr for circle operations, pow for exponent, sin for sine calculation
                sind to draw a sine to the console, rend for rendering mode
                rand for a random number
                "
            );
            std::io::stdout().flush().expect("flush fail");
            misc::screenhold();
            calculator();
        }
        "0" | "stop" | "exit" => {
            misc::silenthold();
            misc::exit(ExitType::Normal);
        }
        _ => {
            println!("Unknown operation");
            misc::screenhold();
            calculator();
        }
    }
}
