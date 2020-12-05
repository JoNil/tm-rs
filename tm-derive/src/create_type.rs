use crate::Property;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn expand_fn<'a>(
    struct_ident: &Ident,
    snake_case_name: &str,
    create_type_ident: &Ident,
    properties: &[Property<'a>],
) -> TokenStream {
    if properties.is_empty() {
        quote! {}
    } else {
        let mut properties_array_content = quote! {};

        for (i, property) in properties.iter().enumerate() {
            let property_name = property.ident.to_string();
            let type_ident = property.ttt.get_enum_ident(property.ident.span());

            properties_array_content.extend(quote! {
                ::tm_rs::ffi::tm_the_truth_property_definition_t {
                    name: ::std::concat!(#property_name, "\0").as_bytes() as *const _ as *const _,
                    type_: ::tm_rs::ffi::#type_ident as u32,
                    editor: 0u32,
                    __bindgen_anon_1: ::tm_rs::ffi::tm_the_truth_property_definition_t__bindgen_ty_1 {
                        enum_editor: ::tm_rs::ffi::tm_the_truth_editor_enum_t {
                            count: 0u32, 
                            _padding_363: [0u8 as ::std::os::raw::c_char; 4usize],
                            names: ::std::ptr::null(),
                            tooltips: ::std::ptr::null(),
                        }
                    },
                    type_hash: 0u64,
                    buffer_extension: ::std::ptr::null(),
                    buffer_extension_f: None,
                    tooltip: ::std::ptr::null(),
                    not_serialized: false,
                    _padding_454: [0u8 as ::std::os::raw::c_char; 7usize],
                    ui_name: ::std::ptr::null(),
                },
            });
        }

        let properties_count = properties.len();

        quote! {
            unsafe extern "C" fn #create_type_ident(
                tt: *mut ::tm_rs::ffi::tm_the_truth_o,
            ) {
                let mut the_truth_api = ::tm_rs::api::with_ctx_mut::<::tm_rs::the_truth::TheTruthApi>(tt);

                let properties: [::tm_rs::ffi::tm_the_truth_property_definition_t; #properties_count] = [
                    #properties_array_content
                ];

                let component_type = the_truth_api
                    .create_object_type(::std::concat!(#snake_case_name, "\0").as_bytes(), &properties);

                /*const tm_tt_id_t default_object = tm_the_truth_api->quick_create_object(
                    tt, TM_TT_NO_UNDO_SCOPE, TM_TT_TYPE_HASH__CAVE_COMPONENT, TM_TT_PROP__CAVE_COMPONENT__FREQUENCY, 1.0f, TM_TT_PROP__CAVE_COMPONENT__AMPLITUDE, 1.0f, -1);
                tm_the_truth_api->set_default_object(tt, cave_component_type, default_object);

                tm_the_truth_api->set_aspect(tt, cave_component_type, TM_CI_EDITOR_UI, editor_aspect);*/
            }
        }
    }
}
