pub mod api;
pub mod component;
pub mod components;
pub mod entity;
pub mod graph_interpreter;
mod hash;
pub mod log;
pub mod registry;
pub mod the_truth;
pub mod the_truth_assets;

pub use hash::hash;
pub use tm_sys::ffi;

pub use tm_sys::ffi::tm_vec2_t as Vec2;
pub use tm_sys::ffi::tm_vec3_t as Vec3;
pub use tm_sys::ffi::tm_vec4_t as Vec4;

#[doc(hidden)]
pub use paste;

#[macro_export]
macro_rules! tm_plugin {
    (|$reg:ident: &mut RegistryApi| $body:block) => {
        #[no_mangle]
        #[allow(clippy::missing_safety_doc)]
        pub unsafe extern "C" fn tm_load_plugin(
            reg: *mut $crate::ffi::tm_api_registry_api,
            load: bool,
        ) {
            let $reg = &mut $crate::registry::RegistryApi::new(reg, load);

            api::register::<EntityApi>($reg);
            api::register::<TheTruthApi>($reg);

            $body
        }
    };
}
