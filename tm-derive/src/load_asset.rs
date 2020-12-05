use crate::Property;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn expand_fn<'a>(
    struct_ident: &Ident,
    load_asset_ident: &Ident,
    properties: &[Property<'a>],
) -> TokenStream {
    if properties.is_empty() {
        quote! {}
    } else {
        let mut get_properties = quote! {};

        for (i, property) in properties.iter().enumerate() {
            let property_ident = property.ident;
            let getter_ident = property.ttt.get_tt_getter_ident(property.ident.span());
            let i = i as u32;

            get_properties.extend(quote! {
                c.#property_ident = the_truth_api.#getter_ident(asset_r, #i);
            });
        }

        quote! {
            unsafe extern "C" fn #load_asset_ident(
                _man: *mut ::tm_rs::ffi::tm_component_manager_o,
                _e: ::tm_rs::ffi::tm_entity_t,
                c_vp: *mut std::ffi::c_void,
                tt: *const ::tm_rs::ffi::tm_the_truth_o,
                asset: ::tm_rs::ffi::tm_tt_id_t,
            ) -> bool {

                let the_truth_api = ::tm_rs::api::with_ctx::<::tm_rs::the_truth::TheTruthApi>(tt);

                let c = c_vp as *mut super::#struct_ident;
                let c = c.as_mut().unwrap();

                let asset_r = the_truth_api.read(asset);

                if asset_r.is_null() {
                    return false;
                }

                *c = Default::default();
                #get_properties

                true
            }
        }
    }
}

pub(crate) fn expand_option<'a>(
    load_asset_ident: &Ident,
    properties: &[Property<'a>],
) -> TokenStream {
    if properties.is_empty() {
        quote! { None }
    } else {
        quote! { Some(#load_asset_ident) }
    }
}
