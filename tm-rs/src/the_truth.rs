use crate::api::{Api, ApiWithCtx};
use std::ffi::c_void;
use tm_sys::ffi::{
    tm_the_truth_api, tm_the_truth_o, tm_the_truth_object_o, tm_tt_id_t, TM_THE_TRUTH_API_NAME,
};

#[derive(Copy, Clone)]
pub struct TheTruthApi {
    api: *mut tm_the_truth_api,
}

unsafe impl Send for TheTruthApi {}
unsafe impl Sync for TheTruthApi {}

impl Api for TheTruthApi {
    type CType = *mut tm_the_truth_api;
    const NAME: &'static [u8] = TM_THE_TRUTH_API_NAME;

    fn new(api: *mut c_void) -> Self {
        Self { api: api as _ }
    }
}

#[derive(Copy, Clone)]
pub struct TheTruthApiInstance {
    api: *mut tm_the_truth_api,
    ctx: *const tm_the_truth_o,
}

impl ApiWithCtx for TheTruthApi {
    type Ctx = tm_the_truth_o;
    type ApiInstance = TheTruthApiInstance;

    #[inline]
    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
        TheTruthApiInstance { api: self.api, ctx }
    }
}

impl TheTruthApiInstance {
    pub fn read(&self, id: tm_tt_id_t) -> *const tm_the_truth_object_o {
        unsafe { (*self.api).read.unwrap()(self.ctx, id) }
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_f32(&self, tto: *const tm_the_truth_object_o, property: u32) -> f32 {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_float.unwrap()(self.ctx, tto, property) }
    }
}
