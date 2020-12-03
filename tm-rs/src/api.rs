use crate::registry::RegistryApi;
use anymap::{any::Any, Map};
use lazy_static::lazy_static;
use std::{ffi::c_void, sync::RwLock};

pub trait Api: Copy + Clone + Send + Sync + 'static {
    type CType;
    const NAME: &'static [u8];

    fn new(api: *mut c_void) -> Self;
}

pub trait ApiWithCtx: Api {
    type Ctx;
    type ApiInstance: Copy + Clone;

    fn wrap(self, ctx: *mut Self::Ctx) -> Self::ApiInstance;
}

lazy_static! {
    static ref REGISTERED_APIS: RwLock<Map<dyn Any + Send + Sync>> = RwLock::new(Map::new());
}

#[allow(clippy::missing_safety_doc)]
pub unsafe fn register<A: Api>(reg: &mut RegistryApi) {
    REGISTERED_APIS
        .write()
        .unwrap()
        .insert(A::new(reg.get(A::NAME)));
}

pub fn get<A: Api>() -> A {
    *REGISTERED_APIS.read().unwrap().get::<A>().unwrap()
}

pub fn with_ctx<A: ApiWithCtx>(ctx: *mut A::Ctx) -> A::ApiInstance {
    assert!(!ctx.is_null());
    REGISTERED_APIS
        .read()
        .unwrap()
        .get::<A>()
        .unwrap()
        .wrap(ctx)
}
