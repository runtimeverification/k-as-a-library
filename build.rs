extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-search=native=.build/lib");
    println!("cargo:rustc-link-lib=dylib=arithmetic");
    println!("cargo:rustc-link-lib=dylib=parser_Exp_ARITHMETIC-SYNTAX");

    println!("cargo:rerun-if-changed=wrapper.h");

    let llvm_kompile_out = Command::new("llvm-kompile")
        .arg("--include-dir")
        .output()
        .expect("Failed to run llvm-kompile");

    let include_path = String::from_utf8_lossy(&llvm_kompile_out.stdout);

    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", include_path.trim()))
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
