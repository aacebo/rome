use quote::quote;

use crate::{reflect_field, reflect_generics, reflect_meta, reflect_visibility};

pub fn derive(input: &syn::DeriveInput, data: &syn::DataStruct) -> proc_macro2::TokenStream {
    let name = &input.ident;
    let ty = build(&syn::ItemStruct {
        attrs: input.attrs.clone(),
        fields: data.fields.clone(),
        generics: input.generics.clone(),
        ident: input.ident.clone(),
        semi_token: data.semi_token,
        struct_token: data.struct_token,
        vis: input.vis.clone(),
    });

    let fields = match &data.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let field_ident = syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: proc_macro2::Span::call_site(),
                });

                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    quote! {
        impl ::ayr_reflect::TypeOf for #name {
            fn type_of() -> ::ayr_reflect::Type {
                return #ty;
            }
        }

        impl ::ayr_reflect::ToType for #name {
            fn to_type(&self) -> ::ayr_reflect::Type {
                return #ty;
            }
        }

        impl ::ayr_reflect::ToValue for #name {
            fn to_value(self) -> ::ayr_reflect::Value {
                return ::ayr_reflect::Dynamic::from_object(self).to_value();
            }
        }

        impl ::ayr_reflect::AsValue for #name {
            fn as_value(&self) -> ::ayr_reflect::Value {
                return ::ayr_reflect::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::ayr_reflect::Dyn for #name { }

        impl ::ayr_reflect::Object for #name {
            fn field(&self, name: &::ayr_reflect::FieldName) -> ::ayr_reflect::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::ayr_reflect::value_of!(self.#fields.clone());
                    }
                )*

                return ::ayr_reflect::Value::Null;
            }
        }
    }
}

pub fn attr(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let ty = build(item);
    let fields = match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .map(|field| {
                let field_ident = &field.ident;
                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let field_ident = syn::Member::Unnamed(syn::Index {
                    index: i as u32,
                    span: proc_macro2::Span::call_site(),
                });

                quote!(#field_ident)
            })
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    quote! {
        impl ::ayr_reflect::TypeOf for #name {
            fn type_of() -> ::ayr_reflect::Type {
                return #ty;
            }
        }

        impl ::ayr_reflect::ToType for #name {
            fn to_type(&self) -> ::ayr_reflect::Type {
                return #ty;
            }
        }

        impl ::ayr_reflect::ToValue for #name {
            fn to_value(self) -> ::ayr_reflect::Value {
                return ::ayr_reflect::Dynamic::from_object(self).to_value();
            }
        }

        impl ::ayr_reflect::AsValue for #name {
            fn as_value(&self) -> ::ayr_reflect::Value {
                return ::ayr_reflect::Dynamic::from_object(self.clone()).as_value();
            }
        }

        impl ::ayr_reflect::Dyn for #name { }

        impl ::ayr_reflect::Object for #name {
            fn field(&self, name: &::ayr_reflect::FieldName) -> ::ayr_reflect::Value {
                #(
                    if name == stringify!(#fields) {
                        return ::ayr_reflect::value_of!(self.#fields.clone());
                    }
                )*

                return ::ayr_reflect::Value::Null;
            }
        }
    }
}

pub fn build(item: &syn::ItemStruct) -> proc_macro2::TokenStream {
    let name = &item.ident;
    let vis = reflect_visibility::build(&item.vis);
    let meta = reflect_meta::build(&item.attrs);
    let generics = reflect_generics::build(&item.generics);
    let layout = match &item.fields {
        syn::Fields::Named(_) => quote!(::ayr_reflect::Layout::Key),
        syn::Fields::Unnamed(_) => quote!(::ayr_reflect::Layout::Index),
        syn::Fields::Unit => quote!(::ayr_reflect::Layout::Unit),
    };

    let fields = match &item.fields {
        syn::Fields::Named(named_fields) => named_fields
            .named
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::build(field, i, true))
            .collect::<Vec<_>>(),
        syn::Fields::Unnamed(unnamed_fields) => unnamed_fields
            .unnamed
            .iter()
            .enumerate()
            .map(|(i, field)| reflect_field::build(field, i, false))
            .collect::<Vec<_>>(),
        syn::Fields::Unit => vec![],
    };

    quote! {
        ::ayr_reflect::StructType::new()
            .with_path(&(::ayr_reflect::Path::from(module_path!())))
            .with_name(stringify!(#name))
            .with_visibility(#vis)
            .with_meta(&#meta)
            .with_generics(&#generics)
            .with_fields(
                ::ayr_reflect::Fields::new()
                    .with_layout(#layout)
                    .with_fields(&[#(#fields,)*])
                    .build()
                    .as_ref()
            )
            .build()
            .to_type()
    }
}
