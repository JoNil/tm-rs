use crate::{
    api::{Api, ApiWithCtx, ApiWithCtxMut},
    hash,
};
use std::{convert::TryInto, ffi::c_void, slice};
use tm_sys::ffi::{
    tm_graph_interpreter_api, tm_graph_interpreter_o, TM_GRAPH_INTERPRETER_API_NAME,
};

#[derive(Copy, Clone)]
pub struct GraphInterpreterApi {
    api: *mut tm_graph_interpreter_api,
}

unsafe impl Send for GraphInterpreterApi {}
unsafe impl Sync for GraphInterpreterApi {}

impl Api for GraphInterpreterApi {
    type CType = *mut tm_graph_interpreter_api;
    const NAME: &'static [u8] = TM_GRAPH_INTERPRETER_API_NAME;

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self {
            api: api as Self::CType,
        }
    }
}

#[derive(Copy, Clone)]
pub struct GraphInterpreterApiInstance {
    pub api: *mut tm_graph_interpreter_api,
    pub ctx: *const tm_graph_interpreter_o,
}

#[derive(Copy, Clone)]
pub struct GraphInterpreterApiInstanceMut {
    pub api: *mut tm_graph_interpreter_api,
    pub ctx: *mut tm_graph_interpreter_o,
}

impl ApiWithCtx for GraphInterpreterApi {
    type Ctx = tm_graph_interpreter_o;
    type ApiInstance = GraphInterpreterApiInstance;

    #[inline]
    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
        GraphInterpreterApiInstance { api: self.api, ctx }
    }
}

impl ApiWithCtxMut for GraphInterpreterApi {
    type ApiInstanceMut = GraphInterpreterApiInstanceMut;

    #[inline]
    fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut {
        GraphInterpreterApiInstanceMut { api: self.api, ctx }
    }
}

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
