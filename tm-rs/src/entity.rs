use crate::{
    api::{self, Api, ApiWithCtx, ApiWithCtxMut},
    component::{ComponentTuple, ComponentsIterator},
};
use std::{
    ffi::{c_void, CString},
};
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

impl Api for EntityApi {
    type CType = *mut tm_entity_api;
    const NAME: &'static [u8] = TM_ENTITY_API_NAME;

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self {
            api: api as Self::CType,
        }
    }
}

#[derive(Copy, Clone)]
pub struct EntityApiInstance {
    pub api: *mut tm_entity_api,
    pub ctx: *const tm_entity_context_o,
}

#[derive(Copy, Clone)]
pub struct EntityApiInstanceMut {
    pub api: *mut tm_entity_api,
    pub ctx: *mut tm_entity_context_o,
}

impl ApiWithCtx for EntityApi {
    type Ctx = tm_entity_context_o;
    type ApiInstance = EntityApiInstance;

    #[inline]
    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
        EntityApiInstance { api: self.api, ctx }
    }
}

impl ApiWithCtxMut for EntityApi {
    type ApiInstanceMut = EntityApiInstanceMut;

    #[inline]
    fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut {
        EntityApiInstanceMut { api: self.api, ctx }
    }
}

struct EngineCallbackData {
    ctx: *mut tm_entity_context_o,
    update: Box<dyn Fn(*mut tm_entity_context_o, &mut tm_engine_update_set_t)>,
}

unsafe extern "C" fn engine_update(inst: *mut tm_engine_o, data: *mut tm_engine_update_set_t) {
    assert!(!inst.is_null());
    assert!(!data.is_null());

    let callback_data = (inst as *const EngineCallbackData).as_ref().unwrap();

    (callback_data.update)(callback_data.ctx, data.as_mut().unwrap());
}

impl EntityApiInstanceMut {
    #[inline]
    pub fn lookup_component(&mut self, name_hash: u64) -> u32 {
        unsafe { (*self.api).lookup_component.unwrap()(self.ctx, name_hash) }
    }

    #[inline]
    pub fn register_engine<C>(
        &mut self,
        name: &'static str,
        update: impl Fn(&mut EntityApiInstanceMut, ComponentsIterator<C>) + Send + Sync + 'static,
    ) where
        C: ComponentTuple,
    {
        let name = CString::new(name).unwrap();

        // This is leaked
        let inst = Box::into_raw(Box::new(EngineCallbackData {
            ctx: self.ctx,
            update: Box::new(move |ctx, update_set| {
                let mut entity_api = api::with_ctx_mut::<EntityApi>(ctx);
                let components = ComponentsIterator::<C>::new(update_set);
                update(&mut entity_api, components)
            }),
        })) as *mut tm_engine_o;

        let engine = tm_engine_i {
            name: name.as_ptr(),
            disabled: false,
            _padding_215: [0; 3],
            num_components: C::get_count(),
            components: C::get_components(self),
            excludes: [false; 16],
            writes: C::get_writes(),
            inst,
            update: Some(engine_update),
            filter: None,
        };

        unsafe {
            (*self.api).register_engine.unwrap()(
                self.ctx,
                &engine as *const ::tm_sys::ffi::tm_engine_i,
            );
        }
    }

    #[inline]
    pub fn register_component(&mut self, component: &tm_component_i) -> u32 {
        unsafe {
            (*self.api).register_component.unwrap()(
                self.ctx,
                component as *const ::tm_sys::ffi::tm_component_i,
            )
        }
    }
}

#[inline]
pub fn mask_has_component(mask: &tm_component_mask_t, i: u32) -> bool {
    (mask.bits[(i / 64) as usize] & (1 << (i % 64))) > 0
}
