use crate::api::{the_truth::TheTruthId, ApiWithCtxMut};
use tm_sys::ffi::{tm_application_api, tm_application_o, TM_APPLICATION_API_NAME};

impl_api_with_ctx!(
    ApplicationApi,
    tm_application_api,
    tm_application_o,
    TM_APPLICATION_API_NAME,
);

impl ApplicationApi {
    #[inline]
    pub fn application(self) -> ApplicationApiInstanceMut {
        self.wrap_mut(unsafe { (*self.api).application.unwrap()() })
    }
}

impl ApplicationApiInstance {
    #[inline]
    pub fn asset_root(&self) -> TheTruthId {
        unsafe { (*self.api).asset_root.unwrap()(self.ctx) }
    }
}
