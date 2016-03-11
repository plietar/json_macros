#[cfg(feature = "with-syntex")]
pub fn main() {
    extern crate syntex;
    extern crate quasi_codegen;
    use std::env;
    use std::path::Path;

    let mut registry = syntex::Registry::new();
    let src = Path::new("src/plugin.in.rs");
    let dst = Path::new(&env::var("OUT_DIR").unwrap()).join("plugin.rs");

    quasi_codegen::register(&mut registry);
    registry.expand("json_macros", &src, &dst).unwrap();
}

#[cfg(not(feature = "with-syntex"))]
pub fn main() {}
