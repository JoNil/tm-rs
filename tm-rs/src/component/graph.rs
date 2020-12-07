use crate::component::Component;
use tm_sys::ffi::{tm_graph_component_t, TM_TT_TYPE__GRAPH_COMPONENT};

pub struct GraphComponent;

impl Component for GraphComponent {
    const NAME: &'static [u8] = TM_TT_TYPE__GRAPH_COMPONENT;
    type CType = tm_graph_component_t;
}
