use super::ffi;
use std::ffi::c_void;

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
        unsafe { (*self.reg).get.unwrap()(name.as_ptr() as _) }
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn add_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        assert!(!implementation.is_null());
        (*self.reg).add_implementation.unwrap()(name.as_ptr() as _, implementation)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn remove_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        assert!(!implementation.is_null());
        (*self.reg).remove_implementation.unwrap()(name.as_ptr() as _, implementation)
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
}

#[macro_export]
macro_rules! add_or_remove_entity_simulation {
    ($reg:expr, fn $name:ident($entity_api:ident: &mut EntityApiInstance) $body:block) => {
        unsafe extern "C" fn $name(ctx: *mut $crate::ffi::tm_entity_context_o) {
            assert!(!ctx.is_null());

            let mut entity_api = $crate::api::with_ctx::<$crate::entity::EntityApi>(ctx);

            (|$entity_api: &mut $crate::entity::EntityApiInstance| $body)(&mut entity_api);
        }

        $reg.add_or_remove_implementation(
            $crate::ffi::TM_ENTITY_SIMULATION_INTERFACE_NAME,
            $name as _,
        );
    };
}
