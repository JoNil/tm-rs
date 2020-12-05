use crate::api::Api;
use std::ffi::{c_void, CString};
use tm_sys::ffi::{
    tm_log_type_TM_LOG_TYPE_DEBUG, tm_log_type_TM_LOG_TYPE_ERROR, tm_log_type_TM_LOG_TYPE_INFO,
    tm_logger_api, TM_LOGGER_API_NAME,
};

#[derive(Copy, Clone)]
pub struct LogApi {
    api: *mut tm_logger_api,
}

unsafe impl Send for LogApi {}
unsafe impl Sync for LogApi {}

impl Api for LogApi {
    type CType = *mut tm_logger_api;
    const NAME: &'static [u8] = TM_LOGGER_API_NAME;

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self { api: api as _ }
    }
}

impl LogApi {
    #[inline]
    pub fn info(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_INFO, message.as_ptr() as _) };
    }

    #[inline]
    pub fn debug(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_DEBUG, message.as_ptr() as _) };
    }

    #[inline]
    pub fn error(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_ERROR, message.as_ptr() as _) };
    }
}
