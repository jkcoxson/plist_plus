// jkcoxson

extern crate bindgen;

use std::{env, fs::canonicalize, path::PathBuf};

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    ////////////////////////////
    //   BINDGEN GENERATION   //
    ////////////////////////////

    if cfg!(feature = "pls-generate") {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Include in clang build
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

    // Check if folder ./override exists
    let override_path = PathBuf::from("./override").join(env::var("TARGET").unwrap());
    if override_path.exists() {
        println!(
            "cargo:rustc-link-search={}",
            canonicalize(&override_path).unwrap().display()
        );
    }

    // Set literally every imaginable path as a search path gosh darn it
    println!("cargo:rustc-link-search=/usr/local/lib");
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-search=/opt/homebrew/lib");
    println!("cargo:rustc-link-search=/usr/local/opt/libimobiledevice/lib");
    println!("cargo:rustc-link-search=/usr/local/opt/libplist/lib");

    let location_determinator;
    if cfg!(feature = "static") {
        location_determinator = "static";
    } else if cfg!(feature = "dynamic") {
        location_determinator = "dylib";
    } else {
        location_determinator = "dylib";
    }

    println!("cargo:rustc-link-lib={}=plist-2.0", location_determinator);
}
