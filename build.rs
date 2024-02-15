fn main() {
    println!("cargo:rustc-link-search=native=./cubiomes");
    println!("cargo:rustc-link-lib=static=cubiomes");

    std::env::set_current_dir("./cubiomes").expect("failed to change directory");
    std::process::Command::new("make")
        .output()
        .expect("failed to run `make`");
}
