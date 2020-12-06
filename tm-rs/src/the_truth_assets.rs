use crate::{
    api::{Api, ApiWithCtx, ApiWithCtxMut},
    the_truth::TheTruthId,
};
use std::ffi::{c_void, CString};
use tm_sys::ffi::{tm_the_truth_assets_api, tm_the_truth_o, TM_THE_TRUTH_ASSETS_API_NAME};

#[derive(Copy, Clone)]
pub struct TheTruthAssetsApi {
    api: *mut tm_the_truth_assets_api,
}

unsafe impl Send for TheTruthAssetsApi {}
unsafe impl Sync for TheTruthAssetsApi {}

impl Api for TheTruthAssetsApi {
    type CType = *mut tm_the_truth_assets_api;
    const NAME: &'static [u8] = TM_THE_TRUTH_ASSETS_API_NAME;

    #[inline]
    fn new(api: *mut c_void) -> Self {
        Self {
            api: api as Self::CType,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TheTruthAssetsApiInstance {
    pub api: *mut tm_the_truth_assets_api,
    pub ctx: *const tm_the_truth_o,
}

#[derive(Copy, Clone)]
pub struct TheTruthAssetsApiInstanceMut {
    pub api: *mut tm_the_truth_assets_api,
    pub ctx: *mut tm_the_truth_o,
}

impl ApiWithCtx for TheTruthAssetsApi {
    type Ctx = tm_the_truth_o;
    type ApiInstance = TheTruthAssetsApiInstance;

    #[inline]
    fn wrap(self, ctx: *const Self::Ctx) -> Self::ApiInstance {
        TheTruthAssetsApiInstance { api: self.api, ctx }
    }
}

impl ApiWithCtxMut for TheTruthAssetsApi {
    type ApiInstanceMut = TheTruthAssetsApiInstanceMut;

    #[inline]
    fn wrap_mut(self, ctx: *mut Self::Ctx) -> Self::ApiInstanceMut {
        TheTruthAssetsApiInstanceMut { api: self.api, ctx }
    }
}

impl TheTruthAssetsApiInstance {
    #[inline]
    pub fn asset_from_path(&self, root: TheTruthId, path: &str) -> TheTruthId {
        let c_path = CString::new(path).unwrap();
        unsafe { (*self.api).asset_from_path.unwrap()(self.ctx, root, c_path.as_ptr()) }
    }
}

impl TheTruthAssetsApiInstanceMut {
    #[inline]
    pub fn asset_from_path(&self, root: TheTruthId, path: &str) -> TheTruthId {
        let c_path = CString::new(path).unwrap();
        unsafe { (*self.api).asset_from_path.unwrap()(self.ctx, root, c_path.as_ptr()) }
    }
}
