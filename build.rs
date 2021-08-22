fn main() {
    //get profile
    let profile = std::env::var("PROFILE").unwrap();

    // set cxx build
    let mut build = cxx_build::bridge("src/lib.rs");

    //set debug or release preprocessor
    if profile == "debug" {
        build.define("DEBUG", "DEBUG");
    } else {
        build.define("NDEBUG", "NDEBUG");
    }

    build
        .file("src/cxx/cr.cpp")
        .flag_if_supported("-std=c++17")
        .compile("cr-rs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/cxx/cr/cr.h");
    println!("cargo:rerun-if-changed=src/cxx/cr.hpp");
    println!("cargo:rerun-if-changed=src/cxx/cr.cpp");
}
