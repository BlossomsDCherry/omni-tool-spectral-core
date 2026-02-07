fn main() {
    println!("cargo:rerun-if-changed=../d16_kernel_recovered/src/d16_kernel.S");
    cc::Build::new()
        .file("../d16_kernel_recovered/src/d16_kernel.S")
        .compile("d16_kernel");
}
