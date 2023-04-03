fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=../include/simple.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-link-lib=simple");
    println!("cargo:rustc-link-search=../target/debug/");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../include/simple.h")
        // Generate bindings for the given file only.
        .allowlist_file("../include/simple.h")
        // Don't prepend the enum name to the variants.
        .prepend_enum_name(false)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
