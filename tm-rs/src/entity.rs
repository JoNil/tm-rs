use crate::{
    api::{self, Api, ApiWithCtx, ApiWithCtxMut},
    component::{ComponentTuple, ComponentsIterator},
    ComponentMask,
};
use std::{
    ffi::{c_void, CString},
    slice,
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
    #[allow(clippy::type_complexity)]
    filter: Option<Box<dyn Fn(&[u32], &ComponentMask) -> bool>>,
}

unsafe extern "C" fn engine_update(inst: *mut tm_engine_o, data: *mut tm_engine_update_set_t) {
    assert!(!inst.is_null());
    assert!(!data.is_null());

    let callback_data = (inst as *const EngineCallbackData).as_ref().unwrap();

    (callback_data.update)(callback_data.ctx, data.as_mut().unwrap());
}

unsafe extern "C" fn engine_filter(
    inst: *mut tm_engine_o,
    components: *const u32,
    num_components: u32,
    mask: *const tm_component_mask_t,
) -> bool {
    assert!(!inst.is_null());
    assert!(!components.is_null());
    assert!(!mask.is_null());

    let callback_data = (inst as *const EngineCallbackData).as_ref().unwrap();
    let components = slice::from_raw_parts(components, num_components as usize);
    let mask = mask.as_ref().unwrap();

    if let Some(filter) = &callback_data.filter {
        filter(components, mask)
    } else {
        false
    }
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
        filter: Option<fn(&[u32], &ComponentMask) -> bool>,
    ) where
        C: ComponentTuple,
    {
        let name = CString::new(name).unwrap();

        let has_filter_fn = filter.is_some();

        // This is leaked
        let inst = Box::into_raw(Box::new(EngineCallbackData {
            ctx: self.ctx,
            update: Box::new(move |ctx, update_set| {
                let mut entity_api = api::with_ctx_mut::<EntityApi>(ctx);
                let components = ComponentsIterator::<C>::new(update_set);
                update(&mut entity_api, components)
            }),
            filter: if let Some(filter) = filter {
                Some(Box::new(filter))
            } else {
                None
            },
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
            filter: if has_filter_fn {
                Some(engine_filter)
            } else {
                None
            },
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
