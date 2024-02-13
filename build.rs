fn main() {
    println!("cargo:rustc-link-search=native=./cubiomes");
    println!("cargo:rustc-link-lib=static=cubiomes");
}
