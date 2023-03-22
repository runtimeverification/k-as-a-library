fn main() {
    println!("cargo:rustc-link-search=native=.build/lib");
    println!("cargo:rustc-link-lib=dylib=arithmetic");
    println!("cargo:rustc-link-lib=dylib=parser_Exp_ARITHMETIC-SYNTAX");
}
