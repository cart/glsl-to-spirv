use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let mut bin_dir = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    bin_dir.push("build");

    if target.contains("x86_64-pc-windows-msvc") {
        bin_dir.push("windows-x64");
    } else if target.contains("i686-pc-windows-msvc") {
        bin_dir.push("windows-i686");
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

    // Link order matters, make sure dependents are linked before their dependencies.
    println!("cargo:rustc-link-search={}", bin_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=glslang.glsltospirv");
    println!("cargo:rustc-link-lib=HLSL.glsltospirv");
    println!("cargo:rustc-link-lib=OGLCompiler.glsltospirv");
    println!("cargo:rustc-link-lib=OSDependent.glsltospirv");
    println!("cargo:rustc-link-lib=SPIRV.glsltospirv");
    println!("cargo:rustc-link-lib=SPVRemapper.glsltospirv");
    if target.contains("x86_64-unknown-linux-gnu") || target.contains("x86_64-apple-darwin") {
        println!("cargo:rustc-link-lib=SPIRV-Tools-opt.glsltospirv");
        println!("cargo:rustc-link-lib=SPIRV-Tools.glsltospirv");
    }
    if target.contains("android") {
        println!("cargo:rustc-link-lib=c++_shared");
    }
}
