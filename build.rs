#[cfg(feature = "with-syntex")]
pub fn main() {
    extern crate quasi_codegen;

    use std::env;
    use std::path::Path;

    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("plugin.rs");
    quasi_codegen::expand("src/plugin.in.rs", dst).unwrap();
}

#[cfg(not(feature = "with-syntex"))]
pub fn main() {}
