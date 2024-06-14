fn main() {
    println!("cargo:rustc-link-arg=-Tcrates/app/src/linker.ld");
    println!("cargo:rustc-env=RUSTFLAGS=-Cforce-frame-pointers=yes");
}