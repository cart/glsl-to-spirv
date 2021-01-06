use std::path::Path;

fn main() {
    let target: &str = &std::env::var("TARGET").unwrap();
    let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let bin_dir = match target {
        "x86_64-pc-windows-msvc" => cargo_dir.join("build").join("windows"),
        "x86_64-unknown-linux-gnu" => cargo_dir.join("build").join("linux"),
        "x86_64-apple-darwin" => cargo_dir.join("build").join("osx"),
        "aarch64-linux-android" => cargo_dir.join("build").join("android-arm64-v8a"),
        "armv7-linux-androideabi" => cargo_dir.join("build").join("android-armeabi-v7a"),
        "i686-pc-windows-msvc" => build::build_libraries(&target),
        _ => panic!("Unsupported target {}", target),
    };

    // Link order matters, make sure dependants are linked before their dependencies.
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

#[cfg(feature = "build-from-source")]
mod build {
    extern crate cmake;

    use std::path::Path;
    use std::path::PathBuf;

    /// Build target libraries if required,
    /// and returns the location of library files
    pub fn build_libraries(_target: &str) -> PathBuf {
        // Prepare directories
        let cargo_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let source_dir = cargo_dir.join("glslang");

        let out_dir_env = std::env::var("OUT_DIR").unwrap().clone();
        let out_dir = Path::new(&out_dir_env);
        let install_dir = out_dir.join("install");
        let library_dir = install_dir.join("lib");

        // Re-use libraries if they exist
        if let Ok(mut entry) = library_dir.read_dir() {
            if entry.next().is_some() {
                // a file exists in the path
                return library_dir;
            }
        }

        // Check glslang folder is initialized
        let cmakelists = source_dir.join("CMakeLists.txt");
        if !cmakelists.exists() {
            panic!("Please make sure the glslang submodule is initialized");
        }

        // Set up "install" subdirectory
        match std::fs::create_dir_all(&install_dir) {
            Ok(_) => {}
            Err(err) => panic!("Unable to create directory: {:?}", err),
        }

        // Common configuration
        cmake::Config::new(&source_dir)
            .define("CMAKE_INSTALL_PREFIX", &install_dir)
            .define("ENABLE_GLSLANG_BINARIES", "OFF")
            .profile("Release")
            .build_target("install")
            .build();

        // Add vendor suffix to all library names
        for path in library_dir
            .read_dir()
            .expect("Unable to locate compiled glslang libraries")
        {
            let filename = path.unwrap().path();
            let metadata = std::fs::metadata(&filename).unwrap();
            if metadata.is_file() {
                let extension = filename.extension().unwrap().to_str().unwrap();
                let new_extension = format!("glsltospirv.{}", extension);
                let new_name = filename.with_extension(new_extension);
                std::fs::copy(&filename, &new_name)
                    .expect("Failed to rename a glslang library for linking");
            }
        }

        return library_dir;
    }
}

#[cfg(not(feature = "build-from-source"))]
mod build {
    use std::path::PathBuf;

    /// Build target libraries if required,
    /// and returns the location of library files
    pub fn build_libraries(target: &str) -> PathBuf {
        panic!("Platform {} must build glslang from source.", &target);
    }
}
