use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Serialize)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => panic!("Tuple structs not supported"),
            Fields::Unit => panic!("Unit structs not supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_serializations = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            self.#field_name.serialize(serializer)?;
        }
    });

    let expanded = quote! {
        impl common_serde::Serialize for #name {
            fn serialize<S: common_serde::Serializer + ?Sized>(
                &self,
                serializer: &mut S
            ) -> Result<(), S::Error> {
                #(#field_serializations)*
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

//=========================================================================================================

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_) => panic!("Tuple structs not supported"),
            Fields::Unit => panic!("Unit structs not supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let field_deserializations = fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! {
            #field_name: common_serde::Deserialize::deserialize(deserializer)?,
        }
    });

    let expanded = quote! {
        impl<'a> common_serde::Deserialize<'a> for #name {
            fn deserialize<D: common_serde::Deserializer<'a>>(
                deserializer: &mut D
            ) -> Result<Self, D::Error> {
                Ok(Self {
                    #(#field_deserializations)*
                })
            }
        }
    };

    TokenStream::from(expanded)
}
