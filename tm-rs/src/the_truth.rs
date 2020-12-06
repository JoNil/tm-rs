use crate::api::{Api, ApiWithCtx, ApiWithCtxMut};
use std::ffi::c_void;
use tm_sys::ffi::{
    tm_the_truth_api, tm_the_truth_o, tm_the_truth_object_o, tm_the_truth_property_definition_t,
    TM_THE_TRUTH_API_NAME,
};

pub use super::ffi::tm_tt_id_t as TheTruthId;

#[derive(Copy, Clone)]
pub struct TheTruthApi {
    api: *mut tm_the_truth_api,
}

unsafe impl Send for TheTruthApi {}
unsafe impl Sync for TheTruthApi {}

impl Api for TheTruthApi {
    type CType = *mut tm_the_truth_api;
    const NAME: &'static [u8] = TM_THE_TRUTH_API_NAME;

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self {
            api: api as Self::CType,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TheTruthApiInstance {
    pub api: *mut tm_the_truth_api,
    pub ctx: *const tm_the_truth_o,
}

#[derive(Copy, Clone)]
pub struct TheTruthApiInstanceMut {
    pub api: *mut tm_the_truth_api,
    pub ctx: *mut tm_the_truth_o,
}

impl ApiWithCtx for TheTruthApi {
    type Ctx = tm_the_truth_o;
    type ApiInstance = TheTruthApiInstance;

    #[inline]
    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
        TheTruthApiInstance { api: self.api, ctx }
    }
}

impl ApiWithCtxMut for TheTruthApi {
    type ApiInstanceMut = TheTruthApiInstanceMut;

    #[inline]
    fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut {
        TheTruthApiInstanceMut { api: self.api, ctx }
    }
}

impl TheTruthApiInstance {
    pub fn read(&self, id: TheTruthId) -> *const tm_the_truth_object_o {
        unsafe { (*self.api).read.unwrap()(self.ctx, id) }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_f32(&self, tto: *const tm_the_truth_object_o, property: u32) -> f32 {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_float.unwrap()(self.ctx, tto, property) }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_f64(&self, tto: *const tm_the_truth_object_o, property: u32) -> f64 {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_double.unwrap()(self.ctx, tto, property) }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_u32(&self, tto: *const tm_the_truth_object_o, property: u32) -> u32 {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_uint32_t.unwrap()(self.ctx, tto, property) }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_u64(&self, tto: *const tm_the_truth_object_o, property: u32) -> u64 {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_uint64_t.unwrap()(self.ctx, tto, property) }
    }

    #[inline]
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn get_bool(&self, tto: *const tm_the_truth_object_o, property: u32) -> bool {
        assert!(!tto.is_null());
        unsafe { (*self.api).get_bool.unwrap()(self.ctx, tto, property) }
    }
}

impl TheTruthApiInstanceMut {
    #[inline]
    pub fn create_object_type(
        &mut self,
        name: &[u8],
        properties: &[tm_the_truth_property_definition_t],
    ) -> u64 {
        unsafe {
            (*self.api).create_object_type.unwrap()(
                self.ctx,
                name.as_ptr() as *const ::std::os::raw::c_char,
                properties.as_ptr(),
                properties.len() as u32,
            )
        }
    }
}
