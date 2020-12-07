use crate::Property;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub(crate) fn expand_fn<'a>(
    snake_case_name: &str,
    create_type_ident: &Ident,
    properties: &[Property<'a>],
) -> TokenStream {
    let mut properties_array_content = quote! {};
    let mut properties_default_values = quote! {};

    for (i, property) in properties.iter().enumerate() {
        let property_name = property.ident.to_string();
        let type_ident = property.ttt.get_enum_ident(property.ident.span());

        properties_array_content.extend(quote! {
            ::tm_rs::ffi::tm_the_truth_property_definition_t {
                name: ::std::concat!(#property_name, "\0").as_bytes().as_ptr() as *const ::std::os::raw::c_char,
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

        let i = i as u32;
        let default_value = &property.default_value;
        let convertor = property.ttt.get_tt_variadic_convertor();

        properties_default_values.extend(quote! {
            #i, #default_value #convertor,
        });
    }
    
    let properties_count = properties.len();

    quote! {

        struct EditorUiWrapper {
            inner: ::tm_rs::ffi::tm_ci_editor_ui_i,
        }

        unsafe impl Send for EditorUiWrapper {}
        unsafe impl Sync for EditorUiWrapper {}

        unsafe extern "C" fn #create_type_ident(
            tt: *mut ::tm_rs::ffi::tm_the_truth_o,
        ) {
            let name = ::std::concat!(#snake_case_name, "\0").as_bytes();
            let hash = ::tm_rs::hash(name);

            let mut the_truth_api = ::tm_rs::api::with_ctx_mut::<::tm_rs::api::the_truth::TheTruthApi>(tt);

            let properties: [::tm_rs::ffi::tm_the_truth_property_definition_t; #properties_count] = [
                #properties_array_content
            ];

            let component_type = the_truth_api
                .create_object_type(name, &properties);

            unsafe {
                let default_object = (*the_truth_api.get_api_ptr()).quick_create_object.unwrap()(
                    the_truth_api.get_ctx_ptr(),
                    ::tm_rs::ffi::tm_tt_undo_scope_t { 
                        u64_: 0u64,
                    },
                    hash,
                    #properties_default_values
                    -1,
                );

                (*the_truth_api.get_api_ptr()).set_default_object.unwrap()(
                    the_truth_api.get_ctx_ptr(),
                    component_type,
                    default_object);

                static mut EDITOR_ASPECT: EditorUiWrapper = EditorUiWrapper {
                    inner: ::tm_rs::ffi::tm_ci_editor_ui_i {
                        disabled: None,
                        category: None,
                        icon_interface: None,
                        gizmo_priority: 0.0f32,
                        _padding_90: [0u8 as::std::os::raw::c_char; 4usize],
                        gizmo_get_transform: None,
                        gizmo_set_transform: None,
                        gizmo_duplicate: None,
                        override_properties: None,
                        viewport_interact: None,
                        create: None,
                        toolbar: None,
                    },
                };
                (*the_truth_api.get_api_ptr()).set_aspect.unwrap()(
                    the_truth_api.get_ctx_ptr(),
                    component_type,
                    ::tm_rs::hash(b"tm_ci_editor_ui_i\0"),
                    &mut EDITOR_ASPECT.inner as *mut ::tm_rs::ffi::tm_ci_editor_ui_i
                        as *mut ::std::ffi::c_void);
            }
        }
    }
}
