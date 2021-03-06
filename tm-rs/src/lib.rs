pub mod api;

pub mod component;
mod hash;
pub mod registry;

pub use hash::hash;
pub use tm_sys::ffi;

pub use tm_sys::ffi::tm_vec2_t as Vec2;
pub use tm_sys::ffi::tm_vec3_t as Vec3;
pub use tm_sys::ffi::tm_vec4_t as Vec4;

pub use log;

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

            api::register::<$crate::api::log::LogApi>($reg);
            api::register::<$crate::api::entity::EntityApi>($reg);
            api::register::<$crate::api::the_truth::TheTruthApi>($reg);

            $crate::log::set_logger(&$crate::api::log::LOGGER).ok();
            $crate::log::set_max_level($crate::log::LevelFilter::Trace);

            $body
        }
    };
}
