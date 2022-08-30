use std::env;
use std::path::{PathBuf, Path};

#[cfg(target_arch = "x86_64")]
fn configure_builder(builder: &mut cc::Build) -> &mut cc::Build {
    builder
        .define("_ENABLE_AVX2", "ON")
        .flag("-mavx2")
        .flag("-mfma")
}

#[cfg(target_arch = "aarch64")]
fn configure_builder(builder: &mut cc::Build) -> &mut cc::Build {
    builder
        .define("_ENABLE_NEON", "ON")
        .build()
}

fn main() {
    println!("cargo:rerun-if-changed=wrapper.hpp");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&crate_dir);
    let libfacedetection_dir = dir.join("libfacedetection/src");

    let mut builder = cc::Build::new();
    builder
        .cpp(true)
        .include(dir.join("libfacedetection/src"))
        .include(&dir)
        .file(libfacedetection_dir.join("facedetectcnn-data.cpp"))
        .file(libfacedetection_dir.join("facedetectcnn-model.cpp"))
        .file(libfacedetection_dir.join("facedetectcnn.cpp"));
    let builder = configure_builder(&mut builder);
    builder.compile("facedetection");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.hpp")
        // give path to headers
        .clang_arg(format!(
            "-I{}",
            libfacedetection_dir.display(),
        ))
        .clang_arg(format!(
            "-I{}",
            dir.display(),
        ))
        // only export one function
        .allowlist_function("facedetect_cnn")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
