use anymap::{any::Any, Map};
use lazy_static::lazy_static;
use std::{ffi::c_void, sync::RwLock};
use tm_sys::ffi::tm_api_registry_api;

use crate::registry::RegistryApi;

pub trait Api {
    type CType;
    type RsType: Copy + Clone + Send + Sync + 'static;
    const NAME: &'static [u8];

    fn new(api: Self::CType) -> Self::RsType;
    fn from_void(api: *mut c_void) -> Self::CType;
}

lazy_static! {
    static ref REGISTERED_APIS: RwLock<Map<dyn Any + Send + Sync>> = RwLock::new(Map::new());
}

pub fn register<A: Api>(reg: *mut tm_api_registry_api) {
    REGISTERED_APIS
        .write()
        .unwrap()
        .insert(A::new(A::from_void(reg.get(A::NAME))));
}

pub fn get<A: Api>() -> A::RsType {
    *REGISTERED_APIS.read().unwrap().get::<A::RsType>().unwrap()
}
