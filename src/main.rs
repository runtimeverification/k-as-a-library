#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use clap::Parser;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn get_owned_string(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

pub struct KorePattern {
    _handle: *mut kore_pattern,
}

pub struct KoreSort {
    _handle: *mut kore_sort,
}

pub struct Block {
    _handle: *mut block,
}

impl KorePattern {
    fn load_pretty(source_file: &Path) -> Self {
        let path_str = source_file.to_str().expect("Invalid input path");

        let c_path_str = CString::new(path_str).expect("Couldn't create C string");
        let null: *const c_char = std::ptr::null();

        let handle = unsafe {
            let kore = parse_Exp(c_path_str.as_ptr(), null);
            kore_pattern_parse(kore)
        };

        Self { _handle: handle }
    }

    fn construct(&self) -> Block {
        let handle = unsafe { kore_pattern_construct(self._handle) };
        Block::new(handle)
    }

    fn make_interpreter_input(&self, sort: &KoreSort) -> Self {
        let new_handle = unsafe { kore_pattern_make_interpreter_input(self._handle, sort._handle) };
        Self {
            _handle: new_handle,
        }
    }

    fn dump(&self) {
        let str = unsafe { get_owned_string(kore_pattern_dump(self._handle)) };
        println!("{}", str);
    }
}

impl KoreSort {
    fn new(sort: &str) -> Self {
        let c_str = CString::new(sort).expect("Couldn't create C string");
        let handle = unsafe { kore_composite_sort_new(c_str.as_ptr()) };
        Self { _handle: handle }
    }
}

impl Block {
    fn new(handle: *mut block) -> Self {
        Self { _handle: handle }
    }

    fn run(&mut self) {
        self.take_steps(-1);
    }

    fn step(&mut self) {
        self.take_steps(1);
    }

    fn take_steps(&mut self, depth: i64) {
        let new_handle = unsafe { take_steps(depth, self._handle) };
        self._handle = new_handle;
    }

    fn dump(&self) {
        let str = unsafe { get_owned_string(kore_block_dump(self._handle)) };
        println!("{}", str);
    }
}

#[derive(Parser)]
struct Cli {
    input_file: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let sort_exp = KoreSort::new("SortExp");
    let pat = KorePattern::load_pretty(&args.input_file).make_interpreter_input(&sort_exp);
    let mut block = pat.construct();
    block.run();
    block.dump();
}
