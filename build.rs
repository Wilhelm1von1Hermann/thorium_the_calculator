// build control system❤️

use std::{
    env,
    fs,
    io::Write,
};
use toml_edit::*;

fn main() {
    println!("cargo:rerun-if-changed=Cargo.toml");

    let mut toml_str = fs::read_to_string("Cargo.toml")
        .expect("Cannot read Cargo.toml");
    let mut doc = toml_str.parse::<Document>()
        .expect("Invalid TOML");

    let mut num = doc["package"]["metadata"]["build"]["number"]
        .as_integer()
        .expect("build.number missing");
    num += 1;
    doc["package"]["metadata"]["build"]["number"] = value(num);

    fs::write("Cargo.toml", doc.to_string())
        .expect("cargo.toml update fail");

    println!("cargo:rustc-env=BUILD_NUMBER={}", num);
}
