use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro]
pub fn hello_world(item: TokenStream) -> TokenStream {
    let item_as_stream = proc_macro2::TokenStream::from(item.clone());
    let input = syn::parse::<DeriveInput>(item).unwrap();
    let name = &input.ident;

    let expanded = quote::quote! {
        #item_as_stream

        impl #name {
            pub fn hello_world(&self) {
                println!("Hello, world!");
            }
        }
    };

    expanded.into()
}
