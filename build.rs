use std::{env, fs};
use std::path::PathBuf;
use std::process::Command;


fn main() {
  println!("cargo:rerun-if-changed=build.rs");

  Command::new("make")
    .arg("-C")
    .arg("../berry")
    .arg("clean")
    .arg("prebuild")
    .output()
    .expect("failed to execute make -C prebuild");

  let paths = fs::read_dir("../berry/src/").unwrap();
  let paths = paths.map(|d| d.unwrap().path()).filter(|p| p.extension().and_then(|e| Some(e == "c")).unwrap_or(false));

  cc::Build::new()
    .files(paths)
    .file("../berry/default/be_modtab.c")
    .flag("-DBE_EMBED")
    .flag("-std=c99")
    .flag("-O2")
    .include("../berry/default/")
    .include("../berry/src/")
    .compile("berry");

  let bindings = bindgen::Builder::default()
    .header("../berry/src/berry.h")
    .header("../berry/src/be_sys.h")
    .clang_arg("-I../berry/default/")
    .use_core()
    .ctypes_prefix("cty")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("berry.rs"))
    .expect("Couldn't write bindings");
}
