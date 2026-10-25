fn main() {
    println!("cargo::rerun-if-changed=src/native.m");
    cc::Build::new().file("src/native.m").compile("native");
}