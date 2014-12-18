fn main() {
    // May want to use pkg-config after submitting that upstream or something.
    println!("cargo:rustc-flags=-L /usr/local/lib -l kcapi");
}
