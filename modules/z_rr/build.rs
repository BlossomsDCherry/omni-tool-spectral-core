fn main() {
    println!("cargo:rerun-if-changed=../d16_kernel_recovered/src/d16_kernel_portable.c");
    cc::Build::new()
        .file("../d16_kernel_recovered/src/d16_kernel_portable.c")
        .compile("d16_kernel");

    // Explicitly link the library we just built
    println!("cargo:rustc-link-lib=static=d16_kernel");

    // Ensure the linker finds it (cc-rs puts it in OUT_DIR)
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
}
