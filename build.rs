fn main() {
    let mut build = cc::Build::new();

    //set debug or release preprocessor
    let profile = std::env::var("PROFILE").unwrap();
    if profile == "debug" {
        build.define("CR_DEBUG", "CR_DEBUG");
    }

    build
        .cpp(true)
        .file("src/cr.cpp")
        .flag_if_supported("-std=c++17")
        .compile("libcr-rs.a");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cr.h");
    println!("cargo:rerun-if-changed=src/cr.cpp");
    println!("cargo:rerun-if-changed=src/cr/cr.h");
}
