use std::time::{SystemTime, UNIX_EPOCH};

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

pub fn sqrt(n: f32) -> f32 {
    //! replaced by sqrt_pow()
    // probably high amounts of unaccuracy
    if n <= 1.0 {
        return n;
    }
    let mut low = 0.0;
    let mut high = n;
    let mut result = 0.0;

    while low <= high {
        let mid = low + (high - low) / 2.0;
        let square = mid * mid;
        if square == n {
            return mid;
        } else if square < n {
            result = mid;
            low = mid + 1.0;
        } else {
            high = mid - 1.0;
        }
    }
    result
}

pub fn sqrt_pow(num: f32) -> f32 {
    // better representation of the square root
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
    if power_n < 0 {
        println!("floats don't support values less than zero.");
        return 1.0;
    } else if power_n == 0 {
        return 1.0;
    } else if power_n == 1 {
        return input;
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
        multiple if multiple == "multiple".to_string() => {
            let result: f32 = num1 * num2;
            return result;
        }
        divide if divide == "divide".to_string() => {
            let result: f32 = num1 / num2;
            return result;
        }
        _ => {
            println!("mathematical_operation() ERROR: Operator not found");
            return 1.0;
        }
    }
}

pub enum CircleCalcType {
    AreaToRadius,
    RadiusToArea
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

pub fn render_function(operationtype: RenderType) -> Vec<char> {
    let con_height = 30;
    let con_width = 120;
    let mut con_vec: Vec<char> = Vec::new();
    let con_sign: char = '#';

    match operationtype {
        RenderType::FULL => { // covers entire screen
            for sign in 0..con_width {
                con_vec.push(con_sign);
                for sign in 0..con_height {
                    con_vec.push(con_sign);
                }
            }
            return con_vec;
        }
        RenderType::CUBE => { // cube rendering using matrics
            // todo
            println!("ERROR: Hasn't been implemented yet.");
            return vec!['E', 'r', 'r'];
        }
        RenderType::TRIANGLE => { // simple triangle rendering
            println!("ERROR: Use render_triangle() instead");
            return vec!['E', 'r', 'r'];
        }
    }
}

pub fn render_triangle() -> Vec<String> {
    let con_sign: char = '#';
    let mut con_vec: Vec<String> = Vec::new();
    let num = 29; // n; length of the triangle, should be equal to the terminal's one - 1
    let mut iter = 1; // i

    while iter <= num {
        let mut iter2 = 1; // j
        while iter2 <= iter {
            con_vec.push(con_sign.to_string());
            iter2 += 1;
        }
        con_vec.push("\n".to_string());
        iter += 1;
    }
    return con_vec;
}

pub fn random_number() -> u32 {
    // totally cryptosafe numbers
    let rand = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    return rand as u32;
}

pub enum RenderType {
    FULL,
    CUBE,
    TRIANGLE
}
