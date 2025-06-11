// this will automatically count compiling attempts

use std::fs;
use std::path::Path;

fn main () {

    let build_file = Path::new("build_num.txt");

    let mut build_number: u32 = match fs::read_to_string(build_file)
        .unwrap_or("0".to_string())
        .trim()
        .parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Couldn't find the build_num.txt: {err}");
                0
            }
    };

    build_number += 1;

    match fs::write(build_file, build_number.to_string()) {
        Ok(_) => (),
        Err(err) => {
            println!("Couldn't write to the build_num.txt: {err}");
        }
    }

    // no idea what is going on
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("build_num.rs");

    match fs::write(
        &dest_path,
        format!("pub const BUILD_NUMBER: u32 = {};", build_number)
    ) {
        Ok(_) => (),
        Err(err) => {
            println!("Couldn't write to the output file build_num.rs: {err}");
        }
    }
}
