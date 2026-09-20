use proc_macro2::TokenStream;
use quote::quote;
use syn::{Field, Ident, Type, punctuated::Punctuated, token::Comma};

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl for<'a> Iterator<Item = TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_name_as_string = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name: self.#field_name.expect(concat!("field not set: ", #field_name_as_string))
        }
    })
}

pub(crate) fn builder_field_definition(
    fields: &Punctuated<Field, Comma>,
) -> impl for<'a> Iterator<Item = TokenStream> {
    fields.iter().map(|f| {
        let (name, f_type) = get_name_and_type(f);
        quote! {pub #name: Option<#f_type>}
    })
}

pub(crate) fn builder_init_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> {
    fields.iter().map(|f| {
        let field_name = &f.ident;
        quote! { #field_name: None }
    })
}

pub(crate) fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            pub fn #field_name(mut self, input: #field_type) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

fn get_name_and_type<'a>(f: &'a Field) -> (&'a Option<Ident>, &'a Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;

    (field_name, field_type)
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{
        Field, FieldModifiers, Ident, Path, PathSegment, Type, TypePath, punctuated::Punctuated,
    };

    use crate::fields::get_name_and_type;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };

        let mut pun = Punctuated::new();
        pun.push(p);

        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
            attrs: vec![],
        });

        let f = Field {
            attrs: vec![],
            vis: syn::Visibility::Inherited,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
            default: None,
            modifiers: FieldModifiers::default(),
        };

        let (actual_name, _) = get_name_and_type(&f);

        assert_eq!(
            actual_name.as_ref().unwrap().to_string(),
            "example".to_string()
        );
    }
}
