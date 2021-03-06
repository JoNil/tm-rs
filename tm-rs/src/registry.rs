use crate::component::DerivedComponent;
use crate::ffi;
use std::{ffi::c_void, os::raw::c_char};
use tm_sys::ffi::{
    TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME, TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
};

pub struct RegistryApi {
    reg: *mut ffi::tm_api_registry_api,
    load: bool,
}

impl RegistryApi {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn new(reg: *mut ffi::tm_api_registry_api, load: bool) -> Self {
        assert!(!reg.is_null());
        Self { reg, load }
    }

    #[inline]
    pub fn get(&mut self, name: &[u8]) -> *mut c_void {
        unsafe { (*self.reg).get.unwrap()(name.as_ptr() as *const c_char) }
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn add_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        assert!(!implementation.is_null());
        (*self.reg).add_implementation.unwrap()(name.as_ptr() as *const c_char, implementation)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn remove_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        assert!(!implementation.is_null());
        (*self.reg).remove_implementation.unwrap()(name.as_ptr() as *const c_char, implementation)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn add_or_remove_implementation(&mut self, name: &[u8], ptr: *mut c_void) {
        assert!(!ptr.is_null());

        if self.load {
            self.add_implementation(name, ptr);
        } else {
            self.remove_implementation(name, ptr);
        }
    }

    pub fn add_or_remove_component<C: DerivedComponent>(&mut self) {
        unsafe {
            self.add_or_remove_implementation(
                TM_THE_TRUTH_CREATE_TYPES_INTERFACE_NAME,
                C::CREATE_TYPES as *mut c_void,
            );
            self.add_or_remove_implementation(
                TM_ENTITY_CREATE_COMPONENT_INTERFACE_NAME,
                C::CREATE_COMPONENT as *mut c_void,
            );
        }
    }
}

#[macro_export]
macro_rules! add_or_remove_entity_simulation {
    ($reg:expr, $name:ident) => {
        $crate::paste::paste! {
            unsafe extern "C" fn [<$name _extern>](ctx: *mut $crate::ffi::tm_entity_context_o) {
                assert!(!ctx.is_null());

                let mut entity_api = $crate::api::with_ctx_mut::<$crate::api::entity::EntityApi>(ctx);

                $name(&mut entity_api);
            }

            $reg.add_or_remove_implementation(
                $crate::ffi::TM_ENTITY_SIMULATION_INTERFACE_NAME,
                [<$name _extern>] as *mut ::std::ffi::c_void,
            );
        }
    };
}
