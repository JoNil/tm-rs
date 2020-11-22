use crate::api::Api;
use std::ffi::{c_void, CString};
use tm_sys::ffi::{tm_log_type_TM_LOG_TYPE_INFO, tm_logger_api, TM_LOGGER_API_NAME};

#[derive(Copy, Clone)]
pub struct LogApi {
    api: *mut tm_logger_api,
}

unsafe impl Send for LogApi {}
unsafe impl Sync for LogApi {}

impl Api for LogApi {
    type CType = *mut tm_logger_api;
    type RsType = LogApi;
    const NAME: &'static [u8] = TM_LOGGER_API_NAME;

    fn new(api: *mut tm_logger_api) -> LogApi {
        LogApi { api }
    }

    fn from_void(api: *mut c_void) -> Self::CType {
        api as _
    }
}

impl LogApi {
    pub fn print_info(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_INFO, message.as_ptr() as _) };
    }

    pub fn print_debug(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_INFO, message.as_ptr() as _) };
    }

    pub fn print_error(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe { (*self.api).print.unwrap()(tm_log_type_TM_LOG_TYPE_INFO, message.as_ptr() as _) };
    }
}
