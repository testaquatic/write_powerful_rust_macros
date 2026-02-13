use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, Ident, parse::Parse,
    parse_macro_input,
};

struct StructField {
    name: Ident,
    ty: Ident,
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;
        quote! {pub #n: #t,}.to_tokens(tokens);
    }
}

impl Parse for StructField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first = input.cursor().ident().unwrap();
        let res = if first.0.to_string().contains("pub") {
            let second = first.1.ident().unwrap();
            let third = second.1.punct().unwrap().1.ident().unwrap();
            Ok(StructField {
                name: second.0,
                ty: third.0,
            })
        } else {
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

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", &ast);
    let name = ast.ident;
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only work for structs with named fields"),
    };
    let builder_fields = fields
        .iter()
        .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());

    let public_version = quote! {
      pub struct #name {
        #(#builder_fields)*
      }
    };

    public_version.into()
}
