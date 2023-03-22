use clap::Parser;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

extern "C" {
    fn parse_Exp(input: *const c_char, location: *const c_char) -> *const c_char;
}

fn parse_exp(source_file: &Path) -> String {
    let path_str = source_file.to_str().expect("Invalid input path");

    let c_path_str = CString::new(path_str).expect("Couldn't create C string");
    let null: *const c_char = std::ptr::null();

    let cstr = unsafe {
        let returned_ptr = parse_Exp(c_path_str.as_ptr(), null);
        CStr::from_ptr(returned_ptr)
    };

    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{}", parse_exp(&args.input_file));
}
