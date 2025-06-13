use crate::misc;

//
// PRIVATE FUNCTIONS
//

fn arctan(num: f32) -> f32 {
    let n0 = num;
    let n1 = (power(num, 3)) / 3.; // x^3/3
    let n2 = (power(num, 5)) / 5.; // x^5/5
    let n3 = (power(num, 7)) / 7.; // x^7/7
    let n4 = (power(num, 9)) / 9.; // x^9/9

    let result = n0 - n1 + n2 - n3 + n4;
    return result;
}

fn approximate_pi() -> f32 {
    // Pi/4 = 4 arctan (1/5) - arctan (1/239)
    let pi_divided_by_4 = (4.0 * arctan(1.0 / 5.0)) - arctan(1.0 / 239.0); // arctan (1/5) - arctan (1/239)

    return pi_divided_by_4 * 4.0;
}

//
// PUBLIC FUNCTIONS
//

pub fn sqrt_pow(num: f32) -> f32 {
    // better representation of the square root
    // but why not the function from std?
    // 08/06/25: Now the only method for square roots
    let mut iter = 0;
    let mut root: f32 = num / 3.0;
    if num <= 0.0 {
        return 0.0;
    }
    while iter < 2048 {
        // oh yeah, 2048 iterations for the square root
        root = (root + num / root) / 2.0;
        iter += 1;
    }
    return root;
}

pub fn sin(x: f32) -> f32 {
    // the input is in degrees
    let pi = approximate_pi();
    let x = x / 180.0 * pi; // converting degrees to radians

    // calculating sin(x) in 3 iterations

    let x0: f32 = x;
    let x1: f32 = (power(x, 3)) / 6.0; // the factorial of 3
    let x2: f32 = (power(x, 5)) / 120.0; // the factorial of 5
    let x3: f32 = (power(x, 7)) / 5040.0; // the factorial of 7
    let result = x0 - x1 + x2 - x3;
    return result;
}

pub fn power(input: f32, power_n: u32) -> f32 {
    // let mut input = input;
    if power_n == 0 {
        println!("floats don't support values less than zero.");
        return 1.0;
    } else if power_n == 1 {
        return 1.0;
    } else {
        return input * power(input, power_n - 1);
    }
}

pub fn mathematical_operation(num1: f32, num2: f32, operation_type: String) -> f32 {
    if num1 <= 0.0 || num2 <= 0.0 {
        println!("mathematical_operation() is useless with zeroes.");
        return 1.0;
    }

    match operation_type {
        plus if plus == "plus".to_string() => {
            let result: f32 = num1 + num2;
            return result;
        }
        minus if minus == "minus".to_string() => {
            let result: f32 = num1 - num2;
            return result;
        }
        multiple if multiple == "multiply".to_string() => {
            let result: f32 = num1 * num2;
            return result;
        }
        divide if divide == "divide".to_string() => {
            let result: f32 = num1 / num2;
            return result;
        }
        _ => {
            println!("mathematical_operation() ERROR: Operator '{operation_type}' not found");
            return 1.0;
        }
    }
}

pub enum CircleCalcType {
    AreaToRadius,
    RadiusToArea
}

impl CircleCalcType {
    pub fn area_to_radius() -> Self {
        let result = CircleCalcType::AreaToRadius;
        return result;
    }

    pub fn radius_to_area() -> Self {
        let result = CircleCalcType::RadiusToArea;
        return result;
    }
}

pub fn circle_calculation(input: f32, operation_type: CircleCalcType) -> f32 {
    if input <= 0.0 {
        println!("circle_calculation() is useless with zeroes.");
        return 1.0;
    }

    let pi = approximate_pi();
    match operation_type {
        CircleCalcType::RadiusToArea => {
            // radius to area
            let num1: f32 = power(input, 2) * pi;
            return num1;
        }
        CircleCalcType::AreaToRadius => {
            // area to radius
            let num1: f32 = sqrt_pow(input) / pi;
            return num1;
        }
    }
}

pub fn number_square(number: i32) -> i32 {
    if number <= 0 {
        misc::thorium_panic("number_square() is useless with zeroes".to_string());
        return 1;
    }

    if number > 65535 {
        println!("ERROR: An calculation of that kind will result in overflow.");
        1
    } else {
        number * number
    }
}

pub fn full_render() -> Vec<char> {
    let con_height = 30;
    let con_width = 120;
    let mut con_vec: Vec<char> = Vec::new();
    let con_sign: char = '#';

    for _sign in 0..con_width {
        con_vec.push(con_sign);
        for _sign in 0..con_height {
            con_vec.push(con_sign);
        }
    }
    return con_vec;
}

pub fn triangle_render() -> Vec<char> {
    let con_sign: char = '#';
    let mut con_vec: Vec<char> = Vec::new();
    let num = 29; // n; length of the triangle, should be equal to the terminal's one - 1
    let mut iter = 1; // i

    while iter <= num {
        let mut iter2 = 1; // j
        while iter2 <= iter {
            con_vec.push(con_sign);
            iter2 += 1;
        }
        con_vec.push('\n');
        iter += 1;
    }
    return con_vec;
}

pub fn sine_wave_render() -> Vec<char> {
    // some copypasta from some program in C

    let mut con_vec: Vec<char> = Vec::new();

    let screen_width = 80;
    let amplitude = 15.0;
    let frequency = 1.0;

    for x in 0..screen_width {
        let angle = x as f32 / screen_width as f32 * 2.0 * approximate_pi() * frequency;
        let sine_value = (amplitude * sin(angle)) as i32;

        let character_index = sine_value + amplitude as i32;

        let character = if character_index <= 0 {
            ' ' // low value
        } else if character_index >= amplitude as i32 {
            '+' // high value
        } else {
            '.'
        };

        con_vec.push(character);
    }
    con_vec.push('\n');

    return con_vec;
}

pub fn celsius_to_fahrenreit(num: f32) -> f32 {
    let result: f32 = num * 1.8 + 32.0;
    return result;
}

pub fn random_number() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    // totally cryptosafe numbers

    // 13/06/25: rewrited to utilize both poor man's rgen and pointers

    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos();

    let pointer = seed as *const u32;
    let result = pointer as usize;

    // usually returns numbers like 820 or 487
    return result % 4096;
}
