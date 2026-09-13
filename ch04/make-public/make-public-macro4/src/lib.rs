use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{DataStruct, DeriveInput, FieldsNamed, Ident, parse::Parse};

#[proc_macro_attribute]
pub fn public4(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let derive_input = syn::parse::<DeriveInput>(item).unwrap();
    let name = derive_input.ident;

    let fields = match derive_input {
        DeriveInput {
            data:
                syn::Data::Struct(DataStruct {
                    fields: syn::Fields::Named(FieldsNamed { named, .. }),
                    ..
                }),
            ..
        } => named,
        _ => unimplemented!(),
    };

    let struct_fields = fields
        .iter()
        .map(|field| syn::parse2::<StructField>(field.into_token_stream()).unwrap());

    let expanded = quote! {
        struct #name {
            #(#struct_fields)*
        }
    };

    TokenStream::from(expanded)
}

struct StructField {
    name: Ident,
    ty: Ident,
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first = input.cursor().ident().unwrap();
        let res = if first.0.to_string().contains("pub") {
            // pub(first.0) field(second.0) :(punct) type(third.0)
            let second = first.1.ident().unwrap();
            let third = second.1.punct().unwrap().1.ident().unwrap();
            Ok(StructField {
                name: second.0,
                ty: third.0,
            })
        } else {
            // field(first.0) :(punct) type(second.0)
            let second = first.1.punct().unwrap().1.ident().unwrap();
            Ok(StructField {
                name: first.0,
                ty: second.0,
            })
        };

        let _ = input.parse::<proc_macro2::TokenStream>();

        res
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ty = &self.ty;

        let expanded = quote! {
            pub #name: #ty,
        };
        tokens.extend(expanded);
    }
}
