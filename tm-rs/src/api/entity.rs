use crate::{
    api::{
        self,
        the_truth::{TheTruthApi, TheTruthApiInstanceMut, TheTruthId},
        the_truth_assets::{TheTruthAssetsApi, TheTruthAssetsApiInstanceMut},
    },
    component::{ComponentTuple, ComponentsIterator},
};
use std::ffi::CString;
use tm_sys::ffi::{
    tm_component_i, tm_component_mask_t, tm_engine_i, tm_engine_o, tm_engine_update_set_t,
    tm_entity_api, tm_entity_context_o, TM_ENTITY_API_NAME,
};

pub use crate::ffi::tm_entity_t as Entity;

impl_api_with_ctx!(
    EntityApi,
    tm_entity_api,
    tm_entity_context_o,
    TM_ENTITY_API_NAME,
);

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
    /// Registers a component with the context.
    #[inline]
    pub fn register_component(&mut self, component: &tm_component_i) -> u32 {
        unsafe {
            (*self.api).register_component.unwrap()(
                self.ctx,
                component as *const ::tm_sys::ffi::tm_component_i,
            )
        }
    }

    /// Registers an engine with the context.
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

    /// Returns The Truth object that the context was created with.
    #[inline]
    pub fn the_truth(&mut self) -> TheTruthApiInstanceMut {
        api::with_ctx_mut::<TheTruthApi>(unsafe { (*self.api).the_truth.unwrap()(self.ctx) })
    }

    /// Returns The Truth object that the context was created with.
    #[inline]
    pub fn the_truth_assets(&mut self) -> TheTruthAssetsApiInstanceMut {
        api::with_ctx_mut::<TheTruthAssetsApi>(unsafe { (*self.api).the_truth.unwrap()(self.ctx) })
    }

    /// Creates an entity based on an entity asset in The Truth. Components and children of the asset will
    /// be automatically created.
    #[inline]
    pub fn create_entity_from_asset(&mut self, asset: TheTruthId) -> Entity {
        unsafe { (*self.api).create_entity_from_asset.unwrap()(self.ctx, asset) }
    }

    /// Looks up a component by name and returns its index. Returns 0 if the component doesn't exist.
    #[inline]
    pub fn lookup_component(&mut self, name_hash: u64) -> u32 {
        unsafe { (*self.api).lookup_component.unwrap()(self.ctx, name_hash) }
    }

    /// Returns the parent of the entity when spawned as a child entity.
    #[inline]
    pub fn parent(&mut self, entity: Entity) -> Entity {
        unsafe { (*self.api).parent.unwrap()(self.ctx, entity) }
    }

    /// Returns the asset used to create the specified entity.
    #[inline]
    pub fn asset(&mut self, entity: Entity) -> TheTruthId {
        unsafe { (*self.api).asset.unwrap()(self.ctx, entity) }
    }
}

#[inline]
pub fn mask_has_component(mask: &tm_component_mask_t, i: u32) -> bool {
    (mask.bits[(i / 64) as usize] & (1 << (i % 64))) > 0
}
