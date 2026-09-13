use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{Data, DataEnum, DataStruct, Fields, FieldsNamed, FieldsUnnamed, Ident, Type};

#[proc_macro_attribute]
pub fn make_public_macro_field_unnamed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let attrs = input.attrs;
    let name = input.ident;
    match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) => {
            let fields = named
                .into_iter()
                .map(|f| (f.ident.unwrap(), f.ty))
                .collect();

            let fields_data = FieldsData::NamedFields {
                name,
                fields,
                pub_option: Some(()),
            };

            quote! {
                #(#attrs)*
                struct
                #fields_data
            }
            .into()
        }
        Data::Struct(DataStruct {
            fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }),
            ..
        }) => {
            let fields = unnamed.into_iter().map(|f| f.ty).collect();
            let fields_data = FieldsData::UnNamedFields {
                name,
                fields,
                pub_option: Some(()),
            };

            quote! {
                #(#attrs)*
                struct #fields_data;
            }
            .into()
        }
        Data::Struct(DataStruct {
            fields: Fields::Unit,
            ..
        }) => {
            let fields_data = FieldsData::Unit(name);
            quote! {
                #(#attrs)*
                struct #fields_data;
            }
            .into()
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let variants = variants.into_iter().map(|v| {
                let name = v.ident;
                match v.fields {
                    Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                        let fields = unnamed.into_iter().map(|f| f.ty).collect();
                        FieldsData::UnNamedFields {
                            name,
                            fields,
                            pub_option: None,
                        }
                    }
                    Fields::Named(FieldsNamed { named, .. }) => {
                        let fields = named
                            .into_iter()
                            .map(|f| (f.ident.unwrap(), f.ty))
                            .collect();
                        FieldsData::NamedFields {
                            name,
                            fields,
                            pub_option: None,
                        }
                    }
                    Fields::Unit => FieldsData::Unit(name),
                }
            });
            quote! {
                #(#attrs)*
                enum #name { #(#variants,)* }
            }
            .into()
        }
        _ => todo!(),
    }
}

enum FieldsData {
    UnNamedFields {
        pub_option: Option<()>,
        name: Ident,
        fields: Vec<Type>,
    },
    NamedFields {
        pub_option: Option<()>,
        name: Ident,
        fields: Vec<(Ident, Type)>,
    },
    Unit(Ident),
}

impl ToTokens for FieldsData {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            FieldsData::UnNamedFields {
                pub_option,
                name,
                fields,
            } => {
                let pub_option = if pub_option.is_some() {
                    quote! {pub}
                } else {
                    quote! {}
                };

                let fields = fields.into_iter().map(|f| {
                    quote! {#pub_option #f}
                });

                quote! {
                    #name( #(#fields,)* )
                }
                .to_tokens(tokens);
            }
            FieldsData::NamedFields {
                pub_option,
                name,
                fields,
            } => {
                let pub_option = if pub_option.is_some() {
                    quote! {pub}
                } else {
                    quote! {}
                };
                let fields = fields.into_iter().map(|(ident, ty)| {
                    quote! { #pub_option #ident : #ty }
                });
                quote! {
                    #name { #(#fields,)* }
                }
                .to_tokens(tokens);
            }
            FieldsData::Unit(name) => {
                quote! {
                    #name
                }
                .to_tokens(tokens);
            }
        }
    }
}
