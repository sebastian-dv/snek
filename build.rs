fn main() {
    println!("cargo:rustc-link-search=src/runtime");
    println!("cargo:rustc-link-lib=our_code");
}
