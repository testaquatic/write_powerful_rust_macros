use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, parse_macro_input};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    // 이름이 있는 필드를 가진 구조체의 필드를 가져온다.
    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("Only works for structs with named fields."),
    };

    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        quote! {pub #name: #ty}
    });

    let public_version = quote! {
        pub struct #name {
            #(#builder_fields,)*
        }
    };

    public_version.into()
}
