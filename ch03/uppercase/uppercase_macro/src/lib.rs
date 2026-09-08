use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(UpperCaseName)]
pub fn upppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let uppercase_name = name.to_string().to_uppercase();

    let add_uppercase = quote! {
        impl #name {
            fn upppercase(&self) {
                println!("{}", #uppercase_name);
            }

            fn testing_tesing() {
                println!("One two three");
            }

            fn hello(&self) {
                println!("{} and {}", stringify!(#name), #uppercase_name);
            }
        }
    };

    add_uppercase.into()
}
