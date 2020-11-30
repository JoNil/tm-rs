use crate::api::{Api, ApiWithCtx};
use std::ffi::c_void;
use tm_sys::ffi::{
    tm_graph_interpreter_api, tm_graph_interpreter_o, tm_graph_interpreter_wire_content_t,
    TM_GRAPH_INTERPRETER_API_NAME,
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

    fn new(api: *mut c_void) -> Self {
        Self { api: api as _ }
    }
}

#[derive(Copy, Clone)]
pub struct GraphInterpreterApiInstance {
    api: *mut tm_graph_interpreter_api,
    ctx: *mut tm_graph_interpreter_o,
}

impl ApiWithCtx for GraphInterpreterApi {
    type Ctx = tm_graph_interpreter_o;
    type ApiInstance = GraphInterpreterApiInstance;

    #[inline]
    fn wrap(self, ctx: *mut Self::Ctx) -> Self::ApiInstance {
        GraphInterpreterApiInstance { api: self.api, ctx }
    }
}

impl GraphInterpreterApiInstance {
    pub fn read_variable(&mut self, variable: u64) -> tm_graph_interpreter_wire_content_t {
        unsafe { (*self.api).read_variable.unwrap()(self.ctx, variable) }
    }
}
