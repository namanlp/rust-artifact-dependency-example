fn main() {
    println!("cargo:rustc-link-lib=static=hello");
    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());

    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
