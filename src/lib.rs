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

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(not(feature = "with-syntex"))]
include!("lib.in.rs");
