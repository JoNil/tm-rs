use std::fmt::{self, Debug, Formatter};

use tm_sys::ffi::{
    tm_the_truth_api, tm_the_truth_o, tm_the_truth_object_o, tm_the_truth_property_definition_t,
    tm_tt_id_t, TM_THE_TRUTH_API_NAME,
};

#[derive(Copy, Clone)]
pub struct TheTruthId(pub(crate) tm_tt_id_t);

impl Debug for TheTruthId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("id", unsafe { &self.0.__bindgen_anon_1.u64_ })
            .finish()
    }
}

impl_api_with_ctx!(
    TheTruthApi,
    tm_the_truth_api,
    tm_the_truth_o,
    TM_THE_TRUTH_API_NAME,
);

impl TheTruthApiInstance {
    /// Get a read pointer for reading properties from the object.
    #[inline]
    pub fn read(&self, id: TheTruthId) -> *const tm_the_truth_object_o {
        unsafe { (*self.api).read.unwrap()(self.ctx, id.0) }
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
    /// Creates a new object type with the specified `name` and the specified set of properties.
    ///
    /// If a type with `name` already exists, that type is returned. Different types with the same
    /// name is not supported.
    ///
    /// `create_object_type()` is not thread-safe and can only be called during the serial "start-up"
    /// phase of using The Truth.
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
