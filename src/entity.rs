use crate::api::{Api, ApiWithCtx};
use std::ffi::{c_void, CString};
use tm_sys::ffi::{
    tm_component_i, tm_component_mask_t, tm_engine_i, tm_engine_o, tm_engine_update_set_t,
    tm_entity_api, tm_entity_context_o, TM_ENTITY_API_NAME,
};

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

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self { api: api as _ }
    }
}

impl ApiWithCtx for EntityApi {
    type Ctx = *mut tm_entity_context_o;
    type ApiInstance = EntityApiInstance;

    #[inline]
    fn wrap(self, ctx: Self::Ctx) -> Self::ApiInstance {
        EntityApiInstance { api: self.api, ctx }
    }
}

pub struct Engine<'a, C, E, W>
where
    C: IntoIterator<Item = &'a u32>,
    E: IntoIterator<Item = &'a bool>,
    W: IntoIterator<Item = &'a bool>,
{
    pub name: &'static str,
    pub disabled: bool,
    pub num_components: u32,
    pub components: C,
    pub excludes: E,
    pub writes: W,
    pub update: unsafe extern "C" fn(inst: *mut tm_engine_o, data: *mut tm_engine_update_set_t),
    pub filter: unsafe extern "C" fn(
        inst: *mut tm_engine_o,
        components: *const u32,
        num_components: u32,
        mask: *const tm_component_mask_t,
    ) -> bool,
}

impl EntityApiInstance {
    #[inline]
    pub fn lookup_component(&mut self, name_hash: u64) -> u32 {
        unsafe { (*self.api).lookup_component.unwrap()(self.ctx, name_hash) }
    }

    #[inline]
    pub fn register_engine<'a, C, E, W>(&mut self, engine: Engine<'a, C, E, W>)
    where
        C: IntoIterator<Item = &'a u32>,
        E: IntoIterator<Item = &'a bool>,
        W: IntoIterator<Item = &'a bool>,
    {
        let name = CString::new(engine.name).unwrap();

        let mut components = [0; 16];
        for (component, out) in engine.components.into_iter().zip(components.iter_mut()) {
            *out = *component;
        }

        let mut excludes = [false; 16];
        for (exclude, out) in engine.excludes.into_iter().zip(excludes.iter_mut()) {
            *out = *exclude;
        }

        let mut writes = [false; 16];
        for (write, out) in engine.writes.into_iter().zip(writes.iter_mut()) {
            *out = *write;
        }

        let engine = tm_engine_i {
            name: name.as_ptr(),
            disabled: engine.disabled,
            _padding_215: [0; 3],
            num_components: engine.num_components,
            components,
            excludes,
            writes,
            inst: self.ctx as *mut tm_engine_o,
            update: Some(engine.update),
            filter: Some(engine.filter),
        };

        unsafe {
            (*self.api).register_engine.unwrap()(self.ctx, &engine as *const _);
        }
    }

    #[inline]
    pub fn register_component(&mut self, component: &tm_component_i) -> u32 {
        unsafe { (*self.api).register_component.unwrap()(self.ctx, component as *const _) }
    }
}

#[inline]
pub unsafe fn mask_has_component(mask: *const tm_component_mask_t, i: u32) -> bool {
    ((*mask).bits[(i / 64) as usize] & (1 << (i % 64))) > 0
}
