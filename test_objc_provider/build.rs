fn main() {
    println!("cargo::rerun-if-changed=objc_src/test_object.h");
    println!("cargo::rerun-if-changed=objc_src/test_object.m");
    cc::Build::new()
        .file("objc_src/test_object.m")
        .flag("-fmodules")
        .compile("objc_lib");
}
