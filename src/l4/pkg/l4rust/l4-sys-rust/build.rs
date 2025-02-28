extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut bindings = bindgen::Builder::default()
        .header("bindgen.h")
        .use_core()
        .derive_default(true)
        .rustified_enum(".*")
        .blacklist_type("l4_addr_t")
        .ctypes_prefix("::libc");
    if let Ok(include_dirs) = ::std::env::var("L4_INCLUDE_DIRS") {
        println!("hi: {}", include_dirs);
        for item in include_dirs.split(" ") {
            bindings = bindings.clang_arg(item);
        }
    }
    let bindings = bindings.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

