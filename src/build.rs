extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .include("vendor/src")
        .file("vendor/src/charset.c")
        .file("vendor/src/filename.c")
        .file("vendor/src/migemo.c")
        .file("vendor/src/mnode.c")
        .file("vendor/src/romaji.c")
        .file("vendor/src/rxgen.c")
        .file("vendor/src/wordbuf.c")
        .file("vendor/src/wordlist.c")
        .compile("migemo");

    let bindings = bindgen::Builder::default()
        .header("vendor/src/migemo.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
