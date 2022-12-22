extern crate cbindgen;

use cbindgen::Config;
use std::env;

// fn main_alt() {
//     // let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let crate_dir = "./";
//     use cbindgen::Language::C;

//     cbindgen::Builder::new()
//       .with_crate(crate_dir)
//       .with_language(C)
//       .generate()
//       .expect("Unable to generate bindings")
//       .write_to_file("built/midi2idim.h");

//     // cbindgen::generate(crate_dir);
// }

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // let build_ffi = env::var("CARGO_GENERATE_FFI").unwrap();

    // if build_ffi == "True" {
    //     let output_file = target_dir().join("smartscreen.h").display().to_string();

    let config = Config::from_file("cbindgen.toml").unwrap();

    cbindgen::generate_with_config(&crate_dir, config)
        .unwrap()
        // .write_to_file(&output_file);
        .write_to_file("built/midi2idim.h");
    // }
}

// Find the location of the `target/` directory. Note that this may be
// overridden by `cmake`, so we also need to check the `CARGO_TARGET_DIR`
// variable.
// fn target_dir() -> PathBuf {
//     if let Ok(target) = env::var("CARGO_TARGET_DIR") {
//         PathBuf::from(target)
//     } else {
//         PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("target")
//     }
// }
