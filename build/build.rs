extern crate cmake;

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build/glslangValidator.exe");

    let target = env::var("TARGET").unwrap();
    let out_file = Path::new(&env::var("OUT_DIR").unwrap()).join("glslang_validator");

    let path = if target.contains("windows") {
        Path::new("build/glslangValidator.exe").to_owned()
    } else if target.contains("linux") {
        Path::new("build/glslangValidatorLinux").to_owned()
    } else if target.contains("apple-darwin") {
        Path::new("build/glslangValidatorOsx").to_owned()
    } else {
        // Try to initialize submodules. Don't care if it fails, since this code also runs for
        // the crates.io package.
        let _ = Command::new("git")
            .arg("submodule")
            .arg("update")
            .arg("--init")
            .status();
        cmake::build("glslang");
        Path::new(&env::var("OUT_DIR").unwrap())
            .join("bin")
            .join("glslangValidator")
    };

    fs::copy(&path, &out_file).expect("failed to copy executable");
}
