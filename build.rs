fn main() {
    println!("cargo:rustc-link-arg-bin=risc-v-rust-bare-metal=-Tlink_script.ld");
 }