use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new().file("mathshim.c").compile("mathshim");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("couldn't write bindings");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=mathshim.c");
}