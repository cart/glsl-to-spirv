use std::{
    env::var,
    fs::{remove_file, File},
    io::copy,
    path::PathBuf,
};

const RELEASE_URL: &str = "https://github.com/creator-rs/glsl-to-spirv/releases/download/0.2.0";

fn main() {
    let target = var("TARGET").unwrap();
    let mut bin_dir = PathBuf::from(&var("CARGO_MANIFEST_DIR").unwrap());
    bin_dir.push("build");

    let platform = if target.contains("x86_64-pc-windows-msvc") {
        "windows"
    } else if target.contains("x86_64-unknown-linux-gnu") {
        "linux"
    } else if target.contains("x86_64-apple-darwin") {
        "osx"
    } else if target.contains("android") {
        if target.contains("aarch64") {
            "android-arm64-v8a"
        } else if target.contains("armv7") {
            "android-armeabi-v7a"
        } else {
            panic!("Missing Android target support {}", target);
        }
    } else {
        panic!("Missing target support {}", target);
    };
    download_and_extract_lib(platform, bin_dir.to_str().unwrap());
    bin_dir.push(platform);

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

fn download_and_extract_lib(platform: &str, path: &str) {
    let mut resp = reqwest::blocking::get(&format!("{}/{}.zip", RELEASE_URL, platform))
        .expect("request failed");
    let zip_file = format!("{}.zip", path);
    let mut out = File::create(&zip_file).expect("failed to create file");
    copy(&mut resp, &mut out).expect("failed to copy content");
    // Upacks zip file to folder
    unzpack::Unzpack::extract(zip_file.clone(), path.to_owned()).unwrap();
    // Deletes zip file
    remove_file(zip_file).expect("failed to delete zip file");
}
