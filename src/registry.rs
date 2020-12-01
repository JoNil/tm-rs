use super::ffi;
use std::ffi::c_void;

pub struct RegistryApi {
    reg: *mut ffi::tm_api_registry_api,
    load: bool,
}

impl RegistryApi {
    pub unsafe fn new(reg: *mut ffi::tm_api_registry_api, load: bool) -> Self {
        assert!(!reg.is_null());
        Self { reg, load }
    }

    #[inline]
    pub fn get(&mut self, name: &[u8]) -> *mut c_void {
        unsafe { (*self.reg).get.unwrap()(name.as_ptr() as _) }
    }

    #[inline]
    pub fn add_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        unsafe { (*self.reg).add_implementation.unwrap()(name.as_ptr() as _, implementation) }
    }

    #[inline]
    pub fn remove_implementation(&mut self, name: &[u8], implementation: *mut c_void) {
        unsafe { (*self.reg).remove_implementation.unwrap()(name.as_ptr() as _, implementation) }
    }

    #[inline]
    pub fn add_or_remove_implementation(&mut self, name: &[u8], ptr: *mut c_void) {
        if self.load {
            self.add_implementation(name, ptr);
        } else {
            self.remove_implementation(name, ptr);
        }
    }
}
