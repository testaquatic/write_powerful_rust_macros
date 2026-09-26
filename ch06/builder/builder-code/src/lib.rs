mod fields;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataStruct, DeriveInput, Fields, FieldsNamed};

use crate::fields::{
    builder_field_definition, builder_init_values, builder_methods, original_struct_setters,
};

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast = syn::parse2::<DeriveInput>(item).unwrap();
    let name = ast.ident;
    let builder = format_ident!("{}Builder", name);

    let fields = match ast.data {
        syn::Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only implemented for structs"),
    };

    let builder_fields = builder_field_definition(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields);

    quote! {
       struct #builder {
        #(#builder_fields,)*
       }

       impl #builder {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder{
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::create_builder;

    #[test]
    fn builder_struct_name_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoField{}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains("StructWithNoFieldBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoField {}
        };
        let expected = quote! {
            struct StructWithNoFieldBuilder{}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains(&expected.to_string()));
    }

    // #[test]
    // fn assert_with_parsing() {
    //     let input = quote! {
    //         struct StructWithNoField{}
    //     };

    //     let actual = create_builder(input);

    //     let derived = syn::parse2::<DeriveInput>(actual).unwrap();
    //     let name = derived.ident;
    //     assert_eq!(name.to_string(), "StructWithNoFieldBuilder");
    // }
}
