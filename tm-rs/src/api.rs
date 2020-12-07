use crate::registry::RegistryApi;
use anymap::{any::Any, Map};
use lazy_static::lazy_static;
use std::{ffi::c_void, sync::RwLock};

pub mod entity;
pub mod graph_interpreter;
pub mod log;
pub mod the_truth;
pub mod the_truth_assets;

pub trait Api: Copy + Clone + Send + Sync + 'static {
    type CType;
    const NAME: &'static [u8];

    fn new(api: *mut c_void) -> Self;
}

pub trait ApiWithCtx: Api {
    type Ctx;
    type ApiInstance;

    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance;
}

pub trait ApiWithCtxMut: ApiWithCtx {
    type ApiInstanceMut;

    fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut;
}

lazy_static! {
    static ref REGISTERED_APIS: RwLock<Map<dyn Any + Send + Sync>> = RwLock::new(Map::new());
}

#[inline]
#[allow(clippy::missing_safety_doc)]
pub unsafe fn register<A: Api>(reg: &mut RegistryApi) {
    REGISTERED_APIS
        .write()
        .unwrap()
        .insert(A::new(reg.get(A::NAME)));
}

#[inline]
pub fn get<A: Api>() -> A {
    *REGISTERED_APIS.read().unwrap().get::<A>().unwrap()
}

#[inline]
pub fn with_ctx<A: ApiWithCtx>(ctx: *const A::Ctx) -> A::ApiInstance {
    assert!(!ctx.is_null());
    REGISTERED_APIS
        .read()
        .unwrap()
        .get::<A>()
        .unwrap()
        .wrap(ctx)
}

#[inline]
pub fn with_ctx_mut<A: ApiWithCtxMut>(ctx: *mut A::Ctx) -> A::ApiInstanceMut {
    assert!(!ctx.is_null());
    REGISTERED_APIS
        .read()
        .unwrap()
        .get::<A>()
        .unwrap()
        .wrap_mut(ctx)
}

#[macro_export]
macro_rules! impl_api {
    ($struct_name:ident, $api_type:ident, $api_name:ident $(,)*) => {
        paste::paste! {

            #[derive(Copy, Clone)]
            pub struct $struct_name {
                api: *mut $api_type,
            }

            unsafe impl Send for $struct_name {}
            unsafe impl Sync for $struct_name {}

            impl Api for $struct_name {
                type CType = *mut $api_type;
                const NAME: &'static [u8] = $api_name;

                #[inline]
                fn new(api: *mut c_void) -> Self {
                    Self {
                        api: api as Self::CType,
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_api_with_ctx {
    ($struct_name:ident, $api_type:ident, $ctx_type:ident, $api_name:ident $(,)*) => (
        paste::paste! {

            #[derive(Copy, Clone)]
            pub struct $struct_name {
                api: *mut $api_type,
            }

            unsafe impl Send for $struct_name {}
            unsafe impl Sync for $struct_name {}

            impl Api for $struct_name {
                type CType = *mut $api_type;
                const NAME: &'static [u8] = $api_name;

                #[inline]
                fn new(api: *mut c_void) -> Self {
                    Self {
                        api: api as Self::CType,
                    }
                }
            }

            pub struct [<$struct_name Instance>] {
                pub api: *mut $api_type,
                pub ctx: *const $ctx_type,
            }

            pub struct [<$struct_name InstanceMut>] {
                pub api: *mut $api_type,
                pub ctx: *mut $ctx_type,
            }

            impl ::std::ops::Deref for [<$struct_name InstanceMut>] {
                type Target = [<$struct_name Instance>];

                #[allow(clippy::transmute_ptr_to_ptr)]
                fn deref(&self) -> &Self::Target {
                    unsafe { ::std::mem::transmute::<&[<$struct_name InstanceMut>], &[<$struct_name Instance>]>(self) }
                }
            }

            impl ApiWithCtx for $struct_name {
                type Ctx = $ctx_type;
                type ApiInstance = [<$struct_name Instance>];

                #[inline]
                fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
                    [<$struct_name Instance>] { api: self.api, ctx }
                }
            }

            impl ApiWithCtxMut for $struct_name {
                type ApiInstanceMut = [<$struct_name InstanceMut>];

                #[inline]
                fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut {
                    [<$struct_name InstanceMut>] { api: self.api, ctx }
                }
            }
        }
    );
}
