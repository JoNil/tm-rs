use crate::Property;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn expand_fn<'a>(load_asset_ident: &Ident, properties: &Vec<Property<'a>>) -> TokenStream {
    if properties.is_empty() {
        quote! {}
    } else {
        quote! {
                unsafe extern "C" fn #load_asset_ident(
                    man: *mut ::tm_rs::ffi::tm_component_manager_o,
                    e: ::tm_rs::ffi::tm_entity_t,
                    c_vp: *mut std::ffi::c_void,
                    tt: *const ::tm_rs::ffi::tm_the_truth_o,
                    asset: ::tm_rs::ffi::tm_tt_id_t) -> bool {

                    /*struct tm_cave_component_t *c = c_vp;
                    const tm_the_truth_object_o *asset_r = tm_tt_read(tt, asset);
                    c->y0 = 0;
                    c->frequency = tm_the_truth_api->get_float(tt, asset_r, TM_TT_PROP__CAVE_COMPONENT__FREQUENCY);
                    c->amplitude = tm_the_truth_api->get_float(tt, asset_r, TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE);*/
                    true
                }
        }
    }
}

pub(crate) fn expand_option<'a>(load_asset_ident: &Ident, properties: &Vec<Property<'a>>) -> TokenStream {
    if properties.is_empty() {
        quote! { None }
    } else {
        quote! { Some(#load_asset_ident) }
    }
}
