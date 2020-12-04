use inflector::Inflector;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Ident};

mod load_asset;

#[derive(Debug)]
struct Property<'a> {
    ident: &'a Ident,
    field: &'a Field,
    attribute: &'a Attribute,
}

#[proc_macro_derive(Component, attributes(property))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident.to_string();
    let snake_case_name = name.to_snake_case();

    let mut properties = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                if let Some(attribute) = field.attrs.iter().find(|a| {
                    if let Some(ident) = a.path.get_ident() {
                        if ident == "property" {
                            return true;
                        }
                    }
                    false
                }) {
                    if let Some(ident) = &field.ident {
                        properties.push(Property {
                            ident,
                            field,
                            attribute,
                        });
                    }
                }
            }
        } else {
            panic!("Only supports Structs with named fields");
        }
    } else {
        panic!("Only supports Structs");
    }

    //println!("{:#?}", properties);

    //panic!("Hej");

    let struct_ident = Ident::new(&name, input.ident.span());
    let internal_mod_ident = Ident::new(
        &format!("__{}_internal", snake_case_name),
        input.ident.span(),
    );
    let create_ident = Ident::new(&format!("{}_create", snake_case_name), input.ident.span());
    let load_asset_ident = Ident::new(
        &format!("{}_load_asset", snake_case_name),
        input.ident.span(),
    );

    let load_asset_fn = load_asset::expand_fn(&load_asset_ident, &properties);
    let load_asset_option = load_asset::expand_option(&load_asset_ident, &properties);

    let expanded = quote! {

        impl ::tm_rs::component::Component for #struct_ident {
            const NAME: &'static [u8] = ::std::concat!(#snake_case_name, "\0").as_bytes();
            type CType = #struct_ident;
        }

       mod #internal_mod_ident {

            #load_asset_fn

            unsafe extern "C" fn #create_ident(
                ctx: *mut ::tm_rs::ffi::tm_entity_context_o
            ) {
                let mut entity_api = ::tm_rs::api::with_ctx::<::tm_rs::entity::EntityApi>(ctx);

                let component = ::tm_rs::ffi::tm_component_i {
                    name: ::std::concat!(#snake_case_name, "\0").as_bytes() as *const _ as *const _,
                    bytes: ::std::mem::size_of::<super::#struct_ident>() as u32,
                    _padding_103: [0 as ::std::os::raw::c_char; 4],
                    default_data: ::std::ptr::null(),
                    manager: ::std::ptr::null_mut(),
                    components_created: None,
                    load_asset: #load_asset_option,
                    asset_loaded: None,
                    asset_loaded_sort_order: 0.0f64,
                    asset_reloaded: None,
                    add: None,
                    remove: None,
                    destroy: None,
                    debug_draw: None,
                    debug_draw_settings: ::tm_rs::ffi::tm_tt_id_t {
                        __bindgen_anon_1: ::tm_rs::ffi::tm_tt_id_t__bindgen_ty_1 {
                            u64_: 0u64,
                        },
                    },
                };

                entity_api.register_component(&component);
            }

            fn assert_send<T: Send>() {}
            fn assert_sync<T: Sync>() {}
            fn assert_copy<T: Copy>() {}
            fn assert_default<T: Default>() {}

            fn assertions() {
                assert_send::<super::#struct_ident>();
                assert_sync::<super::#struct_ident>();
                assert_copy::<super::#struct_ident>();
                assert_default::<super::#struct_ident>();
            }
        }
    };

    //println!("{}", expanded.to_string());

    TokenStream::from(expanded)
}
