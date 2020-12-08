use std::fmt::{self, Debug, Formatter};

use tm_sys::ffi::{
    tm_the_truth_api, tm_the_truth_o, tm_the_truth_object_o, tm_the_truth_property_definition_t,
    tm_tt_id_t, TM_THE_TRUTH_API_NAME,
};

#[derive(Copy, Clone)]
pub struct TheTruthId(pub(crate) tm_tt_id_t);

impl TheTruthId {
    pub fn wrap(input: tm_tt_id_t) -> TheTruthId {
        TheTruthId(input)
    }
}

impl Debug for TheTruthId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("TheTruthId")
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

pub struct TheTruthObject<'a> {
    the_truth: &'a TheTruthApiInstance,
    object: *const tm_the_truth_object_o,
}

impl TheTruthApiInstance {
    /// Get a read pointer for reading properties from the object.
    #[inline]
    pub fn read(&self, id: TheTruthId) -> Option<TheTruthObject> {
        let object = unsafe { (*self.api).read.unwrap()(self.ctx, id.0) };

        if object.is_null() {
            None
        } else {
            Some(TheTruthObject {
                the_truth: self,
                object,
            })
        }
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

impl<'a> TheTruthObject<'a> {
    #[inline]
    pub fn get_f32(&self, property: u32) -> f32 {
        unsafe {
            (*self.the_truth.api).get_float.unwrap()(self.the_truth.ctx, self.object, property)
        }
    }

    #[inline]
    pub fn get_f64(&self, property: u32) -> f64 {
        unsafe {
            (*self.the_truth.api).get_double.unwrap()(self.the_truth.ctx, self.object, property)
        }
    }

    #[inline]
    pub fn get_u32(&self, property: u32) -> u32 {
        unsafe {
            (*self.the_truth.api).get_uint32_t.unwrap()(self.the_truth.ctx, self.object, property)
        }
    }

    #[inline]
    pub fn get_u64(&self, property: u32) -> u64 {
        unsafe {
            (*self.the_truth.api).get_uint64_t.unwrap()(self.the_truth.ctx, self.object, property)
        }
    }

    #[inline]
    pub fn get_bool(&self, property: u32) -> bool {
        unsafe {
            (*self.the_truth.api).get_bool.unwrap()(self.the_truth.ctx, self.object, property)
        }
    }
}
