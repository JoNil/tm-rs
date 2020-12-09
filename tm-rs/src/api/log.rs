use crate::api::{self, Api};
use log::{Level, Log, Metadata, Record};
use std::ffi::CString;
use tm_sys::ffi::{
    tm_log_type_TM_LOG_TYPE_DEBUG, tm_log_type_TM_LOG_TYPE_ERROR, tm_log_type_TM_LOG_TYPE_INFO,
    tm_logger_api, TM_LOGGER_API_NAME,
};

impl_api!(LogApi, tm_logger_api, TM_LOGGER_API_NAME);

impl LogApi {
    #[inline]
    pub fn info(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe {
            (*self.api).print.unwrap()(
                tm_log_type_TM_LOG_TYPE_INFO,
                message.as_ptr() as *const ::std::os::raw::c_char,
            )
        };
    }

    #[inline]
    pub fn debug(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe {
            (*self.api).print.unwrap()(
                tm_log_type_TM_LOG_TYPE_DEBUG,
                message.as_ptr() as *const ::std::os::raw::c_char,
            )
        };
    }

    #[inline]
    pub fn error(&self, message: &str) {
        let message = CString::new(message).unwrap();
        unsafe {
            (*self.api).print.unwrap()(
                tm_log_type_TM_LOG_TYPE_ERROR,
                message.as_ptr() as *const ::std::os::raw::c_char,
            )
        };
    }
}

pub struct TmLogger;

impl Log for TmLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        let api = api::get::<LogApi>();
        let string = format!("{}", record.args());

        match record.level() {
            Level::Error => {
                api.error(&string);
            }
            Level::Warn => {
                api.error(&string);
            }
            Level::Info => {
                api.info(&string);
            }
            Level::Debug | Level::Trace => {
                api.debug(&string);
            }
        }
    }

    fn flush(&self) {}
}

pub static LOGGER: TmLogger = TmLogger;
