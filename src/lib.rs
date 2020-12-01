#![allow(clippy::missing_safety_doc)]

pub mod api;
pub mod component;
pub mod components;
pub mod entity;
pub mod graph_interpreter;
mod hash;
pub mod log;
pub mod registry;

pub use hash::hash;
pub use tm_sys::ffi;

#[macro_export]
macro_rules! tm_plugin {
    (|$reg:ident: &mut RegistryApi| $body:block) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn tm_load_plugin(reg: *mut tm_api_registry_api, load: bool) {
            let $reg = &mut $crate::registry::RegistryApi::new(reg, load);
            $body
        }
    };
}
