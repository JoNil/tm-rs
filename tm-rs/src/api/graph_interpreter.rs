use crate::{
    api::{Api, ApiWithCtx, ApiWithCtxMut},
    hash, impl_api_with_ctx,
};
use std::{convert::TryInto, ffi::c_void, slice};
use tm_sys::ffi::{
    tm_graph_interpreter_api, tm_graph_interpreter_o, TM_GRAPH_INTERPRETER_API_NAME,
};

impl_api_with_ctx!(
    GraphInterpreterApi,
    tm_graph_interpreter_api,
    tm_graph_interpreter_o,
    TM_GRAPH_INTERPRETER_API_NAME,
);

impl GraphInterpreterApiInstanceMut {
    #[inline]
    pub fn read_variable_f32(&mut self, variable: &str) -> Option<f32> {
        let variable = hash(variable.as_bytes());
        let var = unsafe { (*self.api).read_variable.unwrap()(self.ctx, variable) };

        if !var.data.is_null() {
            let data = unsafe { slice::from_raw_parts(var.data as *mut u8, var.size as usize) };
            Some(f32::from_ne_bytes(data.try_into().unwrap()))
        } else {
            None
        }
    }
}
