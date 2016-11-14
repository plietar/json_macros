#![cfg_attr(not(feature = "with-syntex"), feature(quote, plugin_registrar, rustc_private))]

#[cfg(not(feature = "with-syntex"))] extern crate rustc;
#[cfg(not(feature = "with-syntex"))] extern crate rustc_plugin;
#[cfg(not(feature = "with-syntex"))] extern crate syntax;
#[cfg(not(feature = "with-syntex"))] use rustc_plugin::Registry;

#[cfg(feature = "with-syntex")] extern crate quasi;
#[cfg(feature = "with-syntex")] extern crate syntex;
#[cfg(feature = "with-syntex")] extern crate syntex_syntax as syntax;
#[cfg(feature = "with-syntex")] use syntex::Registry;

#[cfg(feature="with-rustc-serialize")]
extern crate rustc_serialize;
#[cfg(feature="with-serde")]
extern crate serde_json;

mod plugin {
    #[cfg(feature = "with-syntex")]
    include!(concat!(env!("OUT_DIR"), "/plugin.rs"));

    #[cfg(not(feature = "with-syntex"))]
    include!("plugin.in.rs");
}

#[cfg(not(feature = "with-syntex"))]
#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", plugin::expand);
}

#[cfg(feature = "with-syntex")]
pub fn expand<S, D>(src: S, dst: D) -> Result<(), syntex::Error>
    where S: AsRef<std::path::Path>,
          D: AsRef<std::path::Path>,
{
    let mut registry = Registry::new();
    registry.add_macro("json", plugin::expand);
    registry.expand("", src.as_ref(), dst.as_ref())
}
