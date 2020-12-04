use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, Ident};

#[derive(Debug)]
struct Property<'a> {
    ident: &'a Ident,
    field: &'a Field,
    attribute: &'a Attribute,
}

#[proc_macro_derive(Component, attributes(property))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut properties = Vec::new();

    if let Data::Struct(data) = &input.data {
        if let Fields::Named(fields) = &data.fields {
            for field in &fields.named {
                println!("{:?}", field.attrs);

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

    println!("{:#?}", properties);

    //panic!("Hej");

    let expanded = quote! {
        fn hej() {
            println!("Hej");
        }
    };

    TokenStream::from(expanded)
}
