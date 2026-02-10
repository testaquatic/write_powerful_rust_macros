use proc_macro::TokenStream;
use quote::quote;
use venial::{Enum, Item, Struct, parse_item};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let declaration = parse_item(item.into()).unwrap();

    let name = match declaration {
        Item::Struct(Struct { name, .. }) => name,
        Item::Enum(Enum { name, .. }) => name,
        _ => panic!("only implement for struct and enum"),
    };

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello World!");
            }
            }

    };

    add_hello_world.into()
}
