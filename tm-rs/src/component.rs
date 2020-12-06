use std::marker::PhantomData;

use tm_sys::ffi::{
    tm_engine_update_array_t, tm_engine_update_set_t, tm_entity_context_o, tm_the_truth_o,
};

use crate::{entity::EntityApiInstanceMut, hash};

pub trait Component {
    const NAME: &'static [u8];
    type CType: Copy;
}

pub trait DerivedComponent: Component {
    const CREATE_TYPES: Option<unsafe extern "C" fn(*mut tm_the_truth_o)>;
    const CREATE_COMPONENT: unsafe extern "C" fn(*mut tm_entity_context_o);
}

pub trait Accessor {
    const WRITE: bool;
    type C: Component;
    type RefT;

    #[allow(clippy::missing_safety_doc)]
    unsafe fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT;
}

pub struct Read<'a, C: Component + 'a> {
    _r: PhantomData<&'a C>,
}

impl<'a, C: Component> Accessor for Read<'a, C> {
    const WRITE: bool = false;
    type C = C;
    type RefT = &'a C::CType;

    #[inline]
    unsafe fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT {
        ptr.as_ref().unwrap()
    }
}

pub struct Write<'a, C: Component + 'a> {
    _w: PhantomData<&'a mut C>,
}

impl<'a, C: Component> Accessor for Write<'a, C> {
    const WRITE: bool = true;
    type C = C;
    type RefT = &'a mut C::CType;

    #[inline]
    unsafe fn ref_from_ptr(ptr: *mut <Self::C as Component>::CType) -> Self::RefT {
        ptr.as_mut().unwrap()
    }
}

pub struct ComponentsIterator<'a, C> {
    arrays: &'a [tm_engine_update_array_t],
    arrays_index: usize,
    components_index: usize,
    phantom_data: PhantomData<C>,
}

impl<'a, C> ComponentsIterator<'a, C> {
    #[inline]
    pub fn new(update_set: &'a mut tm_engine_update_set_t) -> Self {
        Self {
            arrays: unsafe {
                (*update_set)
                    .arrays
                    .as_mut_slice((*update_set).num_arrays as usize)
            },
            arrays_index: 0,
            components_index: 0,
            phantom_data: PhantomData,
        }
    }
}

pub trait ComponentTuple {
    fn get_components(entity_api: &mut EntityApiInstanceMut) -> [u32; 16];
    fn get_writes() -> [bool; 16];
    fn get_count() -> u32;
}

macro_rules! replace_expr {
    ($_i:ident, $sub:expr) => {
        $sub
    };
}

macro_rules! replace_lit {
    ($_t:literal, $sub:expr) => {
        $sub
    };
}

macro_rules! count_idents {
    ($($tts:ident),*) => {
        0u32 $(+ replace_expr!($tts, 1u32))*
    };
}

macro_rules! impl_component_tuple {
    ($($t:ident),* $(,)* $($none:literal),*) => {

        paste::paste! {
            impl<'a, $($t),*> Iterator for ComponentsIterator<'a, ($($t),*,)>
            where
                $($t: Accessor),*
            {
                type Item = ($($t::RefT),*,);

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

                    #[allow(unused_assignments)]
                    unsafe {

                        let mut component_count = 0;

                        $(
                            let [<$t:lower>] = (array.components[component_count] as *mut <$t::C as Component>::CType)
                                .add(self.components_index);

                            component_count += 1;
                        )*

                        self.components_index += 1;

                        Some(($($t::ref_from_ptr([<$t:lower>])),*,))
                    }
                }
            }
        }

        impl<$($t),*> ComponentTuple for ($($t),*,)
        where
            $($t: Accessor),*
        {
            #[inline]
            fn get_components(entity_api: &mut $crate::entity::EntityApiInstanceMut) -> [u32; 16] {
                [
                    $(entity_api.lookup_component(hash($t::C::NAME))),*,
                    $(replace_lit!($none, 0)),*
                ]
            }

            #[inline]
            fn get_writes() -> [bool; 16] {
                [
                    $($t::WRITE),*,
                    $(replace_lit!($none, false)),*
                ]
            }

            #[inline]
            fn get_count() -> u32 {
                count_idents!($($t),*)
            }
        }
    };
}

impl_component_tuple!(A, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, 0, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, 0, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, 0, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, 0, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, 0, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, 0, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, 0, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, 0, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, 0);
impl_component_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
