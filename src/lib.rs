extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

macro_rules! panic_error {
    ($e:expr) => {
        (match $e {
            Ok(ok) => ok,
            Err(err) => panic!("{err}"),
        })
    };
}

#[proc_macro_derive(Model)]
pub fn make_model(stream: TokenStream) -> TokenStream {
    let ast = panic_error!(syn::parse::<syn::DeriveInput>(stream));
    let name = ast.ident;

    let ts = quote! {
        impl #name {
            pub fn Q() -> oxidar::db::ModelQuery<Self> {
                oxidar::db::ModelQuery::<Self>::new::<Self>(stringify!(#name))
            }
        }
    };

    return ts.into();
}

#[proc_macro_derive(ToTemplateVar, attributes(template_accessable))]
pub fn convert_to_template_var(stream: TokenStream) -> TokenStream {
    let ast = panic_error!(syn::parse::<syn::DeriveInput>(stream));
    let name = ast.ident;

    let mut fields = Vec::new();
    match ast.data {
        syn::Data::Struct(data_struct) => {
            data_struct.fields.iter().for_each(|x| {
                if x.attrs
                    .iter()
                    .any(|x| x.path().is_ident("template_accessable"))
                {
                    let key = &x.ident;

                    fields.push(quote! {
                        (stringify!(#key), oxidar::templates::TemplateVar::from(&self.#key)),
                    });
                }
            });
        }
        _ => panic!(concat!(
            "ToTemplateVar currently only works on structs, ",
            "not enums or unions. This feature will be added in the future."
        )),
    }

    quote! {
        impl oxidar::templates::ToTemplateVar for #name {
            fn to_template_var(&self) -> oxidar::templates::TemplateVar {
                oxidar::templates::TemplateVar::Indexable(std::collections::HashMap::from([
                    #(#fields)*
                ]))
            }
        }
    }
    .into()
}
