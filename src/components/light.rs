use crate::component::Component;
use tm_sys::ffi::{tm_light_component_t, TM_TT_TYPE__LIGHT_COMPONENT};

pub struct LightComponent;

impl Component for LightComponent {
    const NAME: &'static [u8] = TM_TT_TYPE__LIGHT_COMPONENT;
    type CType = tm_light_component_t;
}
