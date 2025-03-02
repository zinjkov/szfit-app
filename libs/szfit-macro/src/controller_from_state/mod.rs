use proc_macro::TokenStream;
use syn::{DeriveInput, Type};

pub fn controller_from_state_impl(ast: DeriveInput) -> TokenStream {
    let ident = ast.ident;
    let ident_name = ident.to_string();
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("expected a struct with named fields"),
    };

    let field_inits = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = get_field_type(&field);
        return quote::quote! { #field_name: catalog.get_one::<#field_type>()? };
    });

    let expanded = quote::quote! {
        impl<S> axum::extract::FromRequestParts<S> for #ident
        where
            dill::Catalog: axum::extract::FromRef<S>,
            S: Send + Sync,
        {
            type Rejection = crate::app_response::WebError;

            fn from_request_parts<'life0, 'life1, 'async_trait>(
                parts: &'life0 mut axum::http::request::Parts,
                state: &'life1 S)
            -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Rejection>>
                        + Send
                        + 'async_trait>,>
            where
                'life0: 'async_trait,
                'life1: 'async_trait,
                Self: 'async_trait,
            {
                Box::pin(async move {
                    use axum::extract::FromRef;
                    let catalog = dill::Catalog::from_ref(state);
                    Ok(Self {
                         #(#field_inits),*
                    })
                })
            }
        }
    };
    TokenStream::from(expanded)
}

fn get_field_type(field: &syn::Field) -> &Type {
    let field_type = &field.ty;
    if let Type::Path(variant) = field_type {
        let path = &variant.path;
        let last_segment = path.segments.last().unwrap();
        let args = &last_segment.arguments;
        if let syn::PathArguments::AngleBracketed(args) = args {
            let arg = args.args.last().unwrap();
            if let syn::GenericArgument::Type(t) = arg {
                return &t;
            }
        }
    }
    &field.ty
}
