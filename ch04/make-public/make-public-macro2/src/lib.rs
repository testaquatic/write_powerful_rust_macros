use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Data, DataStruct, DeriveInput, Field, Ident, Type, parse_macro_input};

struct StructField {
    name: Ident,
    ty: Type,
}

impl StructField {
    fn new(field: &Field) -> Self {
        Self {
            name: field.ident.as_ref().unwrap().clone(),
            ty: field.ty.clone(),
        }
    }
}

impl ToTokens for StructField {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let n = &self.name;
        let t = &self.ty;

        let expanded = quote! {
            pub #n: #t,
        };

        expanded.to_tokens(tokens);
    }
}

#[proc_macro_attribute]
pub fn public2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    eprintln!("{:#?}", &item);
    let DeriveInput {
        ident,
        data: Data::Struct(DataStruct { fields, .. }),
        ..
    } = item
    else {
        unimplemented!("Only struct is supported");
    };

    let builder_fields = fields.iter().map(StructField::new);

    let expanded = quote! {
        struct #ident {
            #(#builder_fields)*
        }
    };

    expanded.into()
}
