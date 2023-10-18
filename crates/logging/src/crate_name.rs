/// Returns the name of the caller's crate
#[macro_export]
macro_rules! crate_name {
    () => {{
        // using module_path!() instead of env!("CARGO_PKG_NAME") to use the correct name within examples
        // must be used within a macro, otherwise module_path!() will return this crate's module and not the caller's module
        let module_name = module_path!();
        module_name
            .find(":")
            .map(|index| &module_name[..index])
            .unwrap_or(module_name)
    }};
}
