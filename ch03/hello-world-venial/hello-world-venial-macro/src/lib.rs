use proc_macro::TokenStream;
use quote::quote;
use venial::{Enum, Item, Struct, parse_item};

#[proc_macro_derive(Hello)]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let declaration = parse_item(input.into()).unwrap();

    let name = match declaration {
        Item::Struct(Struct { name, .. }) => name,
        Item::Enum(Enum { name, .. }) => name,
        _ => panic!("only implementd for struct and enum"),
    };

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("hello world");
            }
        }
    };

    add_hello_world.into()
}
