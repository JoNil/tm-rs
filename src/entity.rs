use crate::api::{Api, ApiWithCtx};
use std::ffi::c_void;
use tm_sys::ffi::{tm_entity_api, tm_entity_context_o, TM_ENTITY_API_NAME};

#[derive(Copy, Clone)]
pub struct EntityApi {
    api: *mut tm_entity_api,
}

unsafe impl Send for EntityApi {}
unsafe impl Sync for EntityApi {}

#[derive(Copy, Clone)]
pub struct EntityApiInstance {
    api: *mut tm_entity_api,
    ctx: *mut tm_entity_context_o,
}

impl Api for EntityApi {
    type CType = *mut tm_entity_api;
    const NAME: &'static [u8] = TM_ENTITY_API_NAME;

    fn new(api: *mut c_void) -> Self {
        Self { api: api as _ }
    }
}

impl ApiWithCtx for EntityApi {
    type Ctx = *mut tm_entity_context_o;
    type ApiInstance = EntityApiInstance;

    fn wrap(self, ctx: Self::Ctx) -> Self::ApiInstance {
        EntityApiInstance { api: self.api, ctx }
    }
}

impl EntityApiInstance {
    pub fn lookup_component(&mut self, name_hash: u64) -> u32 {
        unsafe { (*self.api).lookup_component.unwrap()(self.ctx, name_hash) }
    }
}
