mod plugin;

#[cfg(not(feature = "with-syntex"))]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("json", plugin::expand);
}

#[cfg(feature = "with-syntex")]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.add_macro("json", plugin::expand);
}
