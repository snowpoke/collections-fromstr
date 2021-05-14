use if_chain::if_chain;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Fields, GenericArgument, PathArguments, Type, Lit, LitChar, Meta};
use std::str::FromStr;

/// Derives `FromStr` for collections. The input will be split along a separator that is specified in a helper attribute like `#[item_separator=","]`.
pub(crate) fn derive_from_str_inner(input: TokenStream1) -> TokenStream1 {
    // get name of the struct that the tags will be added to
    let collection_struct = parse_macro_input!(input as syn::ItemStruct);
    let struct_ident = collection_struct.ident;

    // get the inner type and its item type
    let fields = match collection_struct.fields {
        Fields::Named(fields) => fields.named,
        Fields::Unnamed(fields) => fields.unnamed,
        Fields::Unit => Punctuated::new(),
    };

    let (inner_type_full, item_type) = fields.iter().find_map(|field|
    if_chain!{
        if let Type::Path(type_path) = &field.ty;
        if let PathArguments::AngleBracketed(args_bracket) = &type_path.path.segments.last().unwrap().arguments;
        if let Some(GenericArgument::Type(item_type)) = &args_bracket.args.first();
        then { 
            Some((type_path, item_type))
        } else {
            None
        }
    }
    ).expect("No collection iterating over a single item was found among the fields.");

    // we get the separator from the `item_separator` attribute
    let item_separator_attr = collection_struct
        .attrs
        .iter()
        .find(|attr| attr.path.segments.last().unwrap().ident == "item_separator")
        .expect("Missing `item_separator` attribute.");
    let separator = if_chain! {
        if let Ok(Meta::NameValue(separator_meta)) = item_separator_attr.parse_meta();
        if let Lit::Str(separator_lit_str) = separator_meta.lit;
        then {
            // if the separator is just one character, we should treat it as such
            if let Ok(c) = char::from_str(&separator_lit_str.value()){
                let lit_char = LitChar::new(c, Span::call_site());
                quote!(#lit_char)
            } else if separator_lit_str.value().is_empty() {
                let span = separator_lit_str.span();
                return quote_spanned!(span=> compile_error!("The separator must not be empty.");).into()
            } else {
                quote!(#separator_lit_str)
            }
        }else{
            let span = item_separator_attr.span();
            return quote_spanned!(span=> compile_error!("Unexpected syntax in `item_separator` attribute.");).into()
        }
    };

    // return token stream for FromStr implementation
    quote!(
        impl std::str::FromStr for #struct_ident {
            type Err = <#item_type as std::str::FromStr>::Err;
            fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                if s.is_empty() {
                    Ok(<#inner_type_full>::default())
                } else {
                    s.split(#separator)
                        .map(<#item_type as std::str::FromStr>::from_str)
                        .collect::<std::result::Result<#inner_type_full, Self::Err>>()
                }
                .map(<Self as std::convert::From<#inner_type_full>>::from)
            }
        }
    )
    .into()
}