extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// fn ty_is_option() -> bool {

// }

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = syn::Ident::new(&bname, name.span());
    let fields =
        if let syn::Data::Struct(
            syn::DataStruct {
                fields: syn::Fields::Named(
                    syn::FieldsNamed {ref named, ..}),
                ..}) = ast.data { named } else { unimplemented!() };

    let optionized = fields.iter().map(|f| {
        let original_name = f.ident.clone();
        let original_ty = f.ty.clone();

        quote! {
            #original_name: std::option::Option<#original_ty>
        }

    });

    let default_values = fields.iter().map(|f| {
        let original_name = f.ident.clone();

        quote! {
           #original_name: None
        }
    });

    let methods = fields.iter().map(|f| {
        let original_name = f.ident.clone();
        let original_ty = f.ty.clone();

        quote! {
            pub fn #original_name(&mut self, #original_name: #original_ty) -> &mut Self {
                self.#original_name = Some(#original_name);
                self
            }
        }
    });

    let build_fields = fields.iter().map(|f| {
        let name = f.ident.clone();
        quote! {
           #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not here"))?
        }
    });

    let expanded = quote! {
        // Builder construction for target struct
        impl #name {
            fn builder() -> #bident {
                #bident {
                    #(#default_values),*
                }
            }
        }

        // Builder itself
        struct #bident {
            #(#optionized),*
        }

        // Builder impl
        impl #bident {
            #(#methods)*

            pub fn build(&self) -> Result<#name, Box<dyn std::error::Error>> {
                Ok(#name {
                    #(#build_fields,)*
                })
            }
        }
    };

    expanded.into()
}
