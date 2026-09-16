use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Attribute,
    Data::Struct,
    DataStruct, DeriveInput,
    Fields::{self},
    FieldsNamed, Generics, Ident, Visibility,
    parse::Parse,
};

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    let field_info = syn::parse::<FieldInfo>(item).unwrap();
    quote! {#field_info}.into()
}

struct FieldInfo {
    attrs: Vec<Attribute>,
    generics: Generics,
    vis: Visibility,
    name: Ident,
    fields: Vec<(Ident, syn::Type)>,
}

impl Parse for FieldInfo {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = input.parse::<DeriveInput>().unwrap();
        match derive_input {
            DeriveInput {
                attrs,
                generics,
                vis,
                ident: name,
                data:
                    Struct(DataStruct {
                        fields: Fields::Named(FieldsNamed { ref named, .. }),
                        ..
                    }),
                ..
            } => {
                let fields = named
                    .iter()
                    .map(|f| (f.ident.clone().unwrap(), f.ty.clone()))
                    .collect();

                Ok(FieldInfo {
                    attrs,
                    generics,
                    vis,
                    name,
                    fields,
                })
            }
            _ => unimplemented!(),
        }
    }
}

impl FieldInfo {
    fn get_methods(&self) -> proc_macro2::TokenStream {
        let methods = self.fields.iter().map(|f| {
            let method_name = Ident::new(&format!("get_{}", &f.0.to_string()), Span::call_site());
            let field_type = &f.1;
            let field_name = &f.0;
            quote! {
                pub fn #method_name(&self) -> & #field_type{
                    &self.#field_name
                }
            }
        });

        let generics = &self.generics;
        let name = &self.name;

        quote! {
            impl #generics #name #generics{
                #(#methods)*
            }
        }
    }

    fn struct_stream(&self) -> proc_macro2::TokenStream {
        let attrs = &self.attrs;
        let generics = &self.generics;
        let vis = &self.vis;
        let name = &self.name;
        let field_name = self.fields.iter().map(|f| &f.0).collect::<Vec<_>>();
        let field_type = self.fields.iter().map(|f| &f.1).collect::<Vec<_>>();

        quote! {
             #(#attrs)*
             #vis  struct #name  #generics{
                 #(#field_name : #field_type,)*
             }
        }
    }
}

impl ToTokens for FieldInfo {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let struct_stream = self.struct_stream();
        let get_methods = self.get_methods();

        tokens.extend(quote! {
            #struct_stream
            #get_methods
        })
    }
}
