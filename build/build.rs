use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut bin_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    bin_dir.push("build");

    if target.contains("x86_64-pc-windows-msvc") {
        bin_dir.push("windows");
    } else if target.contains("x86_64-unknown-linux-gnu") {
        bin_dir.push("linux");
    } else if target.contains("x86_64-apple-darwin") {
        bin_dir.push("osx");
    } else if target.contains("android") {
        if target.contains("aarch64") {
            bin_dir.push("android-arm64-v8a");
        } else if target.contains("armv7") {
            bin_dir.push("android-armeabi-v7a");
        } else {
            panic!("Missing Android target support {}", target);
        }
    } else {
        panic!("Missing target support {}", target);
    };

    println!("cargo:rustc-link-search={}", bin_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=glslang");
    println!("cargo:rustc-link-lib=HLSL");
    println!("cargo:rustc-link-lib=OGLCompiler");
    println!("cargo:rustc-link-lib=OSDependent");
    println!("cargo:rustc-link-lib=SPIRV");
    println!("cargo:rustc-link-lib=SPVRemapper");
    if target.contains("x86_64-unknown-linux-gnu") || target.contains("x86_64-apple-darwin") {
    	println!("cargo:rustc-link-lib=SPIRV-Tools-opt");
    	println!("cargo:rustc-link-lib=SPIRV-Tools");
    }
}
