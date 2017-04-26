extern crate bindgen;

fn main() {
    bindgen::Builder::default()
        .header("test.h")
        .generate().unwrap()
        .write_to_file("src/out.rs").unwrap();
}