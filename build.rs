extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let lib_name = "libplist-2.0";
    println!("cargo:return-if-changed=wrapper.h");
    let output = Command::new("pkg-config")
        .arg("--libs-only-L")
        .arg("--static")
        .arg(lib_name)
        .output()
        .expect("pkg-config command failed")
        .stdout;

    let libs = String::from_utf8_lossy(&output)
        .split(" ")
        .collect::<Vec<&str>>()
        .join(" ");
    println!("cargo:rustc-flags={}", libs);
    println!("cargo:rustc-link-lib=static={}", "plist-2.0");

    let cargs = Command::new("pkg-config")
        .arg("--cflags")
        .arg(lib_name)
        .output()
        .expect("pkg-config command failed")
        .stdout;

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(String::from_utf8_lossy(&cargs).trim_end().split(" "))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .size_t_is_usize(true)
        .layout_tests(false)
        .generate()
        .expect("Unsable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}