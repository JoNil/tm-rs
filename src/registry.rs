use super::ffi;
use std::ffi::{c_void, CString};

pub trait RegistryApi {
    fn set(self, name: &[u8], api: *mut c_void, bytes: u32);
    fn remove(self, api: *mut c_void);
    fn get(self, name: &[u8]) -> *mut c_void;
    fn get_optional(self, name: &[u8]) -> *mut c_void;
    fn add_implementation(self, name: &[u8], implementation: *mut c_void);
    fn remove_implementation(self, name: &[u8], implementation: *mut c_void);
    fn implementations(self, name: &[u8], count: &mut u32) -> *mut *mut c_void;
    fn add_listener(self, listener: *const ffi::tm_api_registry_listener_i);
    fn static_variable(self, id: u64, size: u32, file: &[u8], line: u32) -> *mut c_void;
    fn log_missing_apis(self);
}

impl RegistryApi for *mut ffi::tm_api_registry_api {
    fn set(self, name: &[u8], api: *mut c_void, bytes: u32) {
        assert!(!self.is_null());
        unsafe { (*self).set.unwrap()(name.as_ptr() as _, api, bytes) }
    }
    fn remove(self, api: *mut c_void) {
        assert!(!self.is_null());
        unsafe { (*self).remove.unwrap()(api) }
    }
    fn get(self, name: &[u8]) -> *mut c_void {
        assert!(!self.is_null());
        unsafe { (*self).get.unwrap()(name.as_ptr() as _) }
    }
    fn get_optional(self, name: &[u8]) -> *mut c_void {
        assert!(!self.is_null());
        unsafe { (*self).get_optional.unwrap()(name.as_ptr() as _) }
    }
    fn add_implementation(self, name: &[u8], implementation: *mut c_void) {
        assert!(!self.is_null());
        unsafe { (*self).add_implementation.unwrap()(name.as_ptr() as _, implementation) }
    }
    fn remove_implementation(self, name: &[u8], implementation: *mut c_void) {
        assert!(!self.is_null());
        unsafe { (*self).remove_implementation.unwrap()(name.as_ptr() as _, implementation) }
    }
    fn implementations(self, name: &[u8], count: &mut u32) -> *mut *mut c_void {
        assert!(!self.is_null());
        unsafe { (*self).implementations.unwrap()(name.as_ptr() as _, count as _) }
    }
    fn add_listener(self, listener: *const ffi::tm_api_registry_listener_i) {
        assert!(!self.is_null());
        unsafe { (*self).add_listener.unwrap()(listener) }
    }
    fn static_variable(self, id: u64, size: u32, file: &[u8], line: u32) -> *mut c_void {
        assert!(!self.is_null());
        unsafe { (*self).static_variable.unwrap()(id, size, file.as_ptr() as _, line) }
    }
    fn log_missing_apis(self) {
        assert!(!self.is_null());
        unsafe { (*self).log_missing_apis.unwrap()() }
    }
}
