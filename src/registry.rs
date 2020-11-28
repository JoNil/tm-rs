use super::ffi;
use std::ffi::c_void;

pub trait RegistryApi {
    unsafe fn set(self, name: &[u8], api: *mut c_void, bytes: u32);
    unsafe fn remove(self, api: *mut c_void);
    unsafe fn get(self, name: &[u8]) -> *mut c_void;
    unsafe fn get_optional(self, name: &[u8]) -> *mut c_void;
    unsafe fn add_implementation(self, name: &[u8], implementation: *mut c_void);
    unsafe fn remove_implementation(self, name: &[u8], implementation: *mut c_void);
    unsafe fn implementations(self, name: &[u8], count: &mut u32) -> *mut *mut c_void;
    unsafe fn add_listener(self, listener: *const ffi::tm_api_registry_listener_i);
    unsafe fn static_variable(self, id: u64, size: u32, file: &[u8], line: u32) -> *mut c_void;
    unsafe fn log_missing_apis(self);
    unsafe fn add_or_remove_implementation(self, load: bool, name: &[u8], ptr: *mut c_void);
}

impl RegistryApi for *mut ffi::tm_api_registry_api {
    #[inline]
    unsafe fn set(self, name: &[u8], api: *mut c_void, bytes: u32) {
        assert!(!self.is_null());
        (*self).set.unwrap()(name.as_ptr() as _, api, bytes)
    }
    #[inline]
    unsafe fn remove(self, api: *mut c_void) {
        assert!(!self.is_null());
        (*self).remove.unwrap()(api)
    }
    #[inline]
    unsafe fn get(self, name: &[u8]) -> *mut c_void {
        assert!(!self.is_null());
        (*self).get.unwrap()(name.as_ptr() as _)
    }
    #[inline]
    unsafe fn get_optional(self, name: &[u8]) -> *mut c_void {
        assert!(!self.is_null());
        (*self).get_optional.unwrap()(name.as_ptr() as _)
    }
    #[inline]
    unsafe fn add_implementation(self, name: &[u8], implementation: *mut c_void) {
        assert!(!self.is_null());
        (*self).add_implementation.unwrap()(name.as_ptr() as _, implementation)
    }
    #[inline]
    unsafe fn remove_implementation(self, name: &[u8], implementation: *mut c_void) {
        assert!(!self.is_null());
        (*self).remove_implementation.unwrap()(name.as_ptr() as _, implementation)
    }
    #[inline]
    unsafe fn implementations(self, name: &[u8], count: &mut u32) -> *mut *mut c_void {
        assert!(!self.is_null());
        (*self).implementations.unwrap()(name.as_ptr() as _, count as _)
    }
    #[inline]
    unsafe fn add_listener(self, listener: *const ffi::tm_api_registry_listener_i) {
        assert!(!self.is_null());
        (*self).add_listener.unwrap()(listener)
    }
    #[inline]
    unsafe fn static_variable(self, id: u64, size: u32, file: &[u8], line: u32) -> *mut c_void {
        assert!(!self.is_null());
        (*self).static_variable.unwrap()(id, size, file.as_ptr() as _, line)
    }
    #[inline]
    unsafe fn log_missing_apis(self) {
        assert!(!self.is_null());
        (*self).log_missing_apis.unwrap()()
    }

    #[inline]
    unsafe fn add_or_remove_implementation(self, load: bool, name: &[u8], ptr: *mut c_void) {
        if load {
            self.add_implementation(name, ptr);
        } else {
            self.remove_implementation(name, ptr);
        }
    }
}
