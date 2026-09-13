use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident, Type, Visibility, parse::Parse,
    punctuated::Punctuated, token::Colon,
};

#[proc_macro_attribute]
pub fn public3(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = syn::parse::<DeriveInput>(item).unwrap();
    let name = &item.ident;
    let fields = match item {
        DeriveInput {
            data:
                Data::Struct(DataStruct {
                    fields: Fields::Named(FieldsNamed { named, .. }),
                    ..
                }),
            ..
        } => named,
        _ => unimplemented!(),
    };

    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    quote! {
        struct #name {
            #(#builder_fields)*
        }
    }
    .into()
}

struct StructField {
    name: Ident,
    ty: Type,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ty = &self.ty;

        tokens.extend(quote! { pub #name: #ty, })
    }
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let _vis = input.parse::<Visibility>();
        let name = input.parse::<Ident>().unwrap();
        let _colon = input.parse::<Colon>().unwrap();
        let ty = input.parse::<Type>().unwrap();
        let _list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(StructField { name, ty })
    }
}
