use quote::quote;

use crate::{reflect_generics, reflect_meta};

pub fn build(item: &syn::ItemImpl) -> proc_macro2::TokenStream {
    let impl_for = &item.self_ty;
    let impl_meta = reflect_meta::build(&item.attrs);
    let impl_generics = reflect_generics::build(&item.generics);
    let impl_trait = item.trait_.as_ref().map(|(_, path, _)| quote!(#path));

    match &impl_trait {
        None => quote! {
            ::ayr_reflect::Impl::new()
                .with_path(&(::ayr_reflect::Path::from(module_path!())))
                .with_type(&(::ayr_reflect::type_of!(#impl_for)))
                .with_meta(&#impl_meta)
                .with_generics(&#impl_generics)
                .build()
        },
        Some(of) => quote! {
            ::ayr_reflect::Impl::new()
                .with_path(&(::ayr_reflect::Path::from(module_path!())))
                .with_type(&(::ayr_reflect::type_of!(#impl_for)))
                .with_meta(&#impl_meta)
                .with_generics(&#impl_generics)
                .with_of(&(::ayr_reflect::Path::from(stringify!(#of))))
                .build()
        },
    }
}
