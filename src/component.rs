use std::{marker::PhantomData, mem::size_of};

use tm_sys::ffi::{
    tm_engine_update_array_t, tm_graph_component_t, tm_light_component_t,
    TM_TT_TYPE__GRAPH_COMPONENT, TM_TT_TYPE__LIGHT_COMPONENT,
};

use crate::{entity::EntityApiInstance, hash};

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
    const WRITE: bool;
    type C: Component;
    type RefT;

    fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT;
}

pub struct Read<'a, C: Component + 'a> {
    _r: PhantomData<&'a C>,
}

impl<'a, C: Component> Accessor for Read<'a, C> {
    const WRITE: bool = false;
    type C = C;
    type RefT = &'a C::CType;

    fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT {
        unsafe { ptr.as_ref().unwrap() }
    }
}

pub struct Write<'a, C: Component + 'a> {
    _w: PhantomData<&'a mut C>,
}

impl<'a, C: Component> Accessor for Write<'a, C> {
    const WRITE: bool = true;
    type C = C;
    type RefT = &'a mut C::CType;

    fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT {
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
            let a =
                (array.components[0] as *mut <A::C as Component>::CType).add(self.components_index);
            let b =
                (array.components[1] as *mut <B::C as Component>::CType).add(self.components_index);

            self.components_index += 1;

            Some((A::ref_from_ptr(a), B::ref_from_ptr(b)))
        }
    }
}

pub trait ComponentTuple {
    fn get_struct_sizes() -> [usize; 16];
    fn get_components(entity_api: &EntityApiInstance) -> [u32; 16];
    fn get_writes() -> [bool; 16];
    fn get_count() -> u32;
}

impl<A, B> ComponentTuple for (A, B)
where
    A: Accessor,
    B: Accessor,
{
    #[inline]
    fn get_struct_sizes() -> [usize; 16] {
        [
            size_of::<<A::C as Component>::CType>(),
            size_of::<<B::C as Component>::CType>(),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }

    #[inline]
    fn get_components(entity_api: &EntityApiInstance) -> [u32; 16] {
        [
            entity_api.lookup_component(hash(A::C::NAME)),
            entity_api.lookup_component(hash(B::C::NAME)),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    }

    #[inline]
    fn get_writes() -> [bool; 16] {
        [
            A::WRITE,
            B::WRITE,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
        ]
    }

    #[inline]
    fn get_count() -> u32 {
        2
    }
}
