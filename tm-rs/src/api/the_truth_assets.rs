use crate::api::the_truth::TheTruthId;
use std::ffi::CString;
use tm_sys::ffi::{tm_the_truth_assets_api, tm_the_truth_o, TM_THE_TRUTH_ASSETS_API_NAME};

impl_api_with_ctx!(
    TheTruthAssetsApi,
    tm_the_truth_assets_api,
    tm_the_truth_o,
    TM_THE_TRUTH_ASSETS_API_NAME,
);

impl TheTruthAssetsApiInstance {
    /// Given a path, returns the asset at that path. If there is no asset at the path, returns zero.
    /// If there are multiple assets with the same path, an arbitrary one will be returned.
    ///
    /// If the path includes an extension, an asset will only be returned if its type matches that
    /// extension. If the path doesn't include an extension, any asset type matching the path
    /// matches.

    #[inline]
    pub fn asset_from_path(&self, root: TheTruthId, path: &str) -> TheTruthId {
        let c_path = CString::new(path).unwrap();
        unsafe { (*self.api).asset_from_path.unwrap()(self.ctx, root, c_path.as_ptr()) }
    }
}
