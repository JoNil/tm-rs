use inflector::Inflector;
use proc_macro2::{Delimiter, Span, TokenStream, TokenTree};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident, Type};

mod create_type;
mod load_asset;

#[derive(Copy, Clone)]
enum TheTruthType {
    Float,
    Double,
    U32,
    U64,
    Bool,
}

impl TheTruthType {
    fn new(ty: &Type) -> Self {
        match ty {
            Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    match ident.to_string().as_str() {
                        "f32" => TheTruthType::Float,
                        "f64" => TheTruthType::Double,
                        "u32" => TheTruthType::U32,
                        "u64" => TheTruthType::U64,
                        "bool" => TheTruthType::Bool,
                        _ => panic!("Unsupported property type"),
                    }
                } else {
                    panic!("Unsupported property type");
                }
            }
            _ => {
                panic!("Unsupported property type");
            }
        }
    }

    fn get_enum_ident(self, span: Span) -> Ident {
        match self {
            TheTruthType::Float => Ident::new(
                "tm_the_truth_property_type_TM_THE_TRUTH_PROPERTY_TYPE_FLOAT",
                span,
            ),
            TheTruthType::Double => Ident::new(
                "tm_the_truth_property_type_TM_THE_TRUTH_PROPERTY_TYPE_DOUBLE",
                span,
            ),
            TheTruthType::U32 => Ident::new(
                "tm_the_truth_property_type_TM_THE_TRUTH_PROPERTY_TYPE_UINT32_T",
                span,
            ),
            TheTruthType::U64 => Ident::new(
                "tm_the_truth_property_type_TM_THE_TRUTH_PROPERTY_TYPE_UINT64_T",
                span,
            ),
            TheTruthType::Bool => Ident::new(
                "tm_the_truth_property_type_TM_THE_TRUTH_PROPERTY_TYPE_BOOL",
                span,
            ),
        }
    }

    fn get_tt_getter_ident(self, span: Span) -> Ident {
        match self {
            TheTruthType::Float => Ident::new("get_f32", span),
            TheTruthType::Double => Ident::new("get_f64", span),
            TheTruthType::U32 => Ident::new("get_u32", span),
            TheTruthType::U64 => Ident::new("get_u64", span),
            TheTruthType::Bool => Ident::new("get_bool", span),
        }
    }

    fn get_tt_variadic_convertor(self) -> TokenStream {
        match self {
            TheTruthType::Float => quote! { as ::std::os::raw::c_double },
            TheTruthType::Double => quote! { as ::std::os::raw::c_double },
            TheTruthType::U32 => quote! { as u32 },
            TheTruthType::U64 => quote! { as u64 },
            TheTruthType::Bool => quote! { as bool },
        }
    }
}

fn get_default_value(attribute: &Attribute) -> TokenStream {
    if let Some(tree) = attribute.tokens.clone().into_iter().next() {
        if let TokenTree::Group(group) = tree {
            if group.delimiter() != Delimiter::Parenthesis {
                return quote! { ::std::default::Default::default() };
            }

            let tokens = group.stream().into_iter().collect::<Vec<_>>();

            if tokens.len() != 3 {
                return quote! { ::std::default::Default::default() };
            }

            if let (TokenTree::Ident(ident), TokenTree::Punct(punct), TokenTree::Literal(lit)) =
                (&tokens[0], &tokens[1], &tokens[2])
            {
                if ident == "default" && punct.as_char() == '=' {
                    return TokenTree::from(lit.clone()).into();
                } else {
                    panic!("Only (default = ..) supported")
                }
            }
        }
    }

    quote! { ::std::default::Default::default() }
}

struct Property<'a> {
    ident: &'a Ident,
    ttt: TheTruthType,
    default_value: TokenStream,
}

#[proc_macro_derive(Component, attributes(property))]
pub fn derive_component(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
                            ttt: TheTruthType::new(&field.ty),
                            default_value: get_default_value(&attribute),
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

    let struct_ident = Ident::new(&name, input.ident.span());
    let internal_mod_ident = Ident::new(
        &format!("__{}_internal", snake_case_name),
        input.ident.span(),
    );
    let create_ident = Ident::new(&format!("{}_create", snake_case_name), input.ident.span());
    let create_type_ident = Ident::new(
        &format!("{}_create_types", snake_case_name),
        input.ident.span(),
    );
    let load_asset_ident = Ident::new(
        &format!("{}_load_asset", snake_case_name),
        input.ident.span(),
    );

    let create_type_fn = create_type::expand_fn(&snake_case_name, &create_type_ident, &properties);

    let load_asset_fn = load_asset::expand_fn(&struct_ident, &load_asset_ident, &properties);
    let load_asset_option = load_asset::expand_option(&load_asset_ident, &properties);

    let expanded = quote! {

        impl ::tm_rs::component::Component for #struct_ident {
            const NAME: &'static [u8] = ::std::concat!(#snake_case_name, "\0").as_bytes();
            type CType = #struct_ident;
        }

       mod #internal_mod_ident {

            #create_type_fn

            #load_asset_fn

            unsafe extern "C" fn #create_ident(
                ctx: *mut ::tm_rs::ffi::tm_entity_context_o
            ) {
                let mut entity_api = ::tm_rs::api::with_ctx_mut::<::tm_rs::entity::EntityApi>(ctx);

                let component = ::tm_rs::ffi::tm_component_i {
                    name: ::std::concat!(#snake_case_name, "\0").as_bytes().as_ptr() as *const ::std::os::raw::c_char,
                    bytes: ::std::mem::size_of::<super::#struct_ident>() as u32,
                    _padding_103: [0u8 as ::std::os::raw::c_char; 4usize],
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

    proc_macro::TokenStream::from(expanded)
}
