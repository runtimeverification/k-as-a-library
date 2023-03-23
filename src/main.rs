use clap::Parser;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

fn get_owned_string(ptr: *const c_char) -> String {
    let cstr = unsafe { CStr::from_ptr(ptr) };
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

#[repr(C)]
pub struct KorePatternHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct KoreSortHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct BlockHandle {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

pub struct KorePattern {
    _handle: *mut KorePatternHandle,
}

pub struct KoreSort {
    _handle: *mut KoreSortHandle,
}

pub struct Block {
    _handle: *mut BlockHandle,
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
    fn new(handle: *mut BlockHandle) -> Self {
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

extern "C" {
    fn parse_Exp(input: *const c_char, location: *const c_char) -> *const c_char;

    fn kore_composite_sort_new(sort: *const c_char) -> *mut KoreSortHandle;

    fn kore_pattern_parse(kore: *const c_char) -> *mut KorePatternHandle;

    fn kore_pattern_dump(pat: *const KorePatternHandle) -> *const c_char;

    fn kore_pattern_make_interpreter_input(
        pat: *const KorePatternHandle,
        sort: *const KoreSortHandle,
    ) -> *mut KorePatternHandle;

    fn kore_pattern_construct(pat: *const KorePatternHandle) -> *mut BlockHandle;

    fn take_steps(depth: i64, term: *mut BlockHandle) -> *mut BlockHandle;

    fn kore_block_dump(term: *const BlockHandle) -> *const c_char;
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
