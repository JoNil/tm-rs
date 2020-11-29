use std::marker::PhantomData;

use tm_sys::ffi::{
    tm_engine_update_array_t, tm_graph_component_t, tm_light_component_t,
    TM_TT_TYPE__GRAPH_COMPONENT, TM_TT_TYPE__LIGHT_COMPONENT,
};

pub trait Component {
    const NAME: &'static [u8];
    type CType;
}

pub struct LightComponent;

impl Component for LightComponent {
    const NAME: &'static [u8] = TM_TT_TYPE__LIGHT_COMPONENT;
    type CType = tm_light_component_t;
}

pub struct GraphComponent;

impl Component for GraphComponent {
    const NAME: &'static [u8] = TM_TT_TYPE__GRAPH_COMPONENT;
    type CType = tm_graph_component_t;
}

pub trait Accessor {
    type RefT;
    type T;

    fn ref_from_ptr(ptr: *mut Self::T) -> Self::RefT;
}

pub struct Read<'a, C: Component + 'a> {
    _r: PhantomData<&'a C>,
}

impl<'a, C: Component> Accessor for Read<'a, C> {
    type RefT = &'a C::CType;
    type T = C::CType;

    fn ref_from_ptr(ptr: *mut Self::T) -> Self::RefT {
        unsafe { ptr.as_ref().unwrap() }
    }
}

pub struct Write<'a, C: Component + 'a> {
    _w: PhantomData<&'a mut C>,
}

impl<'a, C: Component> Accessor for Write<'a, C> {
    type RefT = &'a mut C::CType;
    type T = C::CType;

    fn ref_from_ptr(ptr: *mut Self::T) -> Self::RefT {
        unsafe { ptr.as_mut().unwrap() }
    }
}

pub struct Components<'a, C> {
    pub arrays: &'a [tm_engine_update_array_t],
    pub arrays_index: usize,
    pub components_index: usize,
    pub phantom_data: PhantomData<C>,
}

impl<'a, A, B> Iterator for Components<'a, (A, B)>
where
    A: Accessor,
    B: Accessor,
{
    type Item = (A::RefT, B::RefT);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.arrays_index >= self.arrays.len() {
            return None;
        }

        let mut array = &self.arrays[self.arrays_index];

        if self.components_index >= array.n as usize {
            self.arrays_index += 1;
            self.components_index = 0;

            if self.arrays_index >= self.arrays.len() {
                return None;
            }

            array = &self.arrays[self.arrays_index];
        }

        unsafe {
            let a = (array.components[0] as *mut A::T).add(self.components_index);
            let b = (array.components[1] as *mut B::T).add(self.components_index);

            self.components_index += 1;

            Some((A::ref_from_ptr(a), B::ref_from_ptr(b)))
        }
    }
}
