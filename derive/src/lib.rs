//! Derive macros for OxiCode
//!
//! This crate provides derive macros for the `Encode` and `Decode` traits in the
//! [OxiCode](https://crates.io/crates/oxicode) binary serialization library.
//!
//! # Usage
//!
//! Add `oxicode` to your `Cargo.toml` with the `derive` feature enabled:
//!
//! ```toml
//! [dependencies]
//! oxicode = { version = "0.1", features = ["derive"] }
//! ```
//!
//! Then derive `Encode` and `Decode` on your types:
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Point {
//!     x: f32,
//!     y: f32,
//! }
//!
//! #[derive(Encode, Decode)]
//! enum Message {
//!     Quit,
//!     Move { x: i32, y: i32 },
//!     Write(String),
//! }
//! ```
//!
//! # Supported Types
//!
//! The derive macros support:
//!
//! - Structs with named fields
//! - Structs with unnamed fields (tuple structs)
//! - Unit structs
//! - Enums with any combination of named, unnamed, and unit variants
//! - Generic types with full lifetime and type parameter support
//! - Where clauses and bounds
//!
//! # Generics
//!
//! The derive macros automatically add appropriate trait bounds to generic type parameters:
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Container<T> {
//!     value: T,
//! }
//!
//! // This generates:
//! // impl<T: Encode> Encode for Container<T> { ... }
//! // impl<T: Decode> Decode for Container<T> { ... }
//! ```
//!
//! # Limitations
//!
//! - Unions are not supported due to safety concerns
//! - For complex scenarios requiring custom serialization logic, implement the traits manually
//!
//! # Examples
//!
//! ## Basic struct
//!
//! ```ignore
//! use oxicode::{encode_to_vec, decode_from_slice, Encode, Decode};
//!
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! struct User {
//!     id: u64,
//!     name: String,
//!     active: bool,
//! }
//!
//! let user = User {
//!     id: 42,
//!     name: "Alice".to_string(),
//!     active: true,
//! };
//!
//! let bytes = encode_to_vec(&user)?;
//! let (decoded, _): (User, _) = decode_from_slice(&bytes)?;
//!
//! assert_eq!(user, decoded);
//! ```
//!
//! ## Enum with variants
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Debug, PartialEq, Encode, Decode)]
//! enum Status {
//!     Active,
//!     Inactive,
//!     Pending { reason: String },
//!     Suspended(u32),
//! }
//! ```
//!
//! ## Generic types
//!
//! ```ignore
//! use oxicode::{Encode, Decode};
//!
//! #[derive(Encode, Decode)]
//! struct Pair<T, U> {
//!     first: T,
//!     second: U,
//! }
//!
//! #[derive(Encode, Decode)]
//! enum Result<T, E> {
//!     Ok(T),
//!     Err(E),
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

/// Derive macro for the `Encode` trait
///
/// Supports structs and enums with full generic and lifetime support.
///
/// # Example
///
/// ```ignore
/// use oxicode::Encode;
///
/// #[derive(Encode)]
/// struct Point {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Encode)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
/// }
/// ```
#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Add Encode bounds to generic type parameters
    let mut generics_with_bounds = generics.clone();
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param.bounds.push(syn::parse_quote!(oxicode::Encode));
        }
    }
    let (impl_generics_with_bounds, _, _) = generics_with_bounds.split_for_impl();

    let encode_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|f| &f.ident);
                quote! {
                    #(self.#field_names.encode(encoder)?;)*
                    Ok(())
                }
            }
            Fields::Unnamed(fields) => {
                let field_indices = (0..fields.unnamed.len()).map(Index::from);
                quote! {
                    #(self.#field_indices.encode(encoder)?;)*
                    Ok(())
                }
            }
            Fields::Unit => quote! { Ok(()) },
        },
        Data::Enum(data_enum) => {
            let variant_encodings = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_name = &variant.ident;
                let variant_idx = idx as u32;

                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                        quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                (#variant_idx as u32).encode(encoder)?;
                                #(#field_names.encode(encoder)?;)*
                                Ok(())
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_names: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| {
                                syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site())
                            })
                            .collect();
                        quote! {
                            Self::#variant_name(#(#field_names),*) => {
                                (#variant_idx as u32).encode(encoder)?;
                                #(#field_names.encode(encoder)?;)*
                                Ok(())
                            }
                        }
                    }
                    Fields::Unit => quote! {
                        Self::#variant_name => (#variant_idx as u32).encode(encoder)
                    },
                }
            });

            quote! {
                match self {
                    #(#variant_encodings,)*
                }
            }
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(input, "Encode cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics_with_bounds oxicode::Encode for #name #ty_generics #where_clause {
            fn encode<__E: oxicode::enc::Encoder>(&self, encoder: &mut __E) -> Result<(), oxicode::Error> {
                #encode_body
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for the `Decode` trait
///
/// Supports structs and enums with full generic and lifetime support.
///
/// # Example
///
/// ```ignore
/// use oxicode::Decode;
///
/// #[derive(Decode)]
/// struct Point {
///     x: f32,
///     y: f32,
/// }
///
/// #[derive(Decode)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
/// }
/// ```
#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (_impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Add Decode bounds to generic type parameters
    let mut generics_with_bounds = generics.clone();
    for param in &mut generics_with_bounds.params {
        if let syn::GenericParam::Type(type_param) = param {
            type_param.bounds.push(syn::parse_quote!(oxicode::Decode));
        }
    }
    let (impl_generics_with_bounds, _, _) = generics_with_bounds.split_for_impl();

    let decode_body = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_decodes = fields.named.iter().map(|f| {
                    let field_name = &f.ident;
                    let field_ty = &f.ty;
                    quote! {
                        #field_name: <#field_ty>::decode(decoder)?
                    }
                });
                quote! {
                    Ok(Self {
                        #(#field_decodes,)*
                    })
                }
            }
            Fields::Unnamed(fields) => {
                let field_decodes = fields.unnamed.iter().map(|f| {
                    let field_ty = &f.ty;
                    quote! { <#field_ty>::decode(decoder)? }
                });
                quote! {
                    Ok(Self(#(#field_decodes,)*))
                }
            }
            Fields::Unit => quote! { Ok(Self) },
        },
        Data::Enum(data_enum) => {
            let variant_decodings = data_enum.variants.iter().enumerate().map(|(idx, variant)| {
                let variant_name = &variant.ident;
                let variant_idx = idx as u32;

                match &variant.fields {
                    Fields::Named(fields) => {
                        let field_decodes = fields.named.iter().map(|f| {
                            let field_name = &f.ident;
                            let field_ty = &f.ty;
                            quote! {
                                #field_name: <#field_ty>::decode(decoder)?
                            }
                        });
                        quote! {
                            #variant_idx => Ok(Self::#variant_name { #(#field_decodes,)* })
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_decodes = fields.unnamed.iter().map(|f| {
                            let field_ty = &f.ty;
                            quote! { <#field_ty>::decode(decoder)? }
                        });
                        quote! {
                            #variant_idx => Ok(Self::#variant_name(#(#field_decodes,)*))
                        }
                    }
                    Fields::Unit => quote! {
                        #variant_idx => Ok(Self::#variant_name)
                    },
                }
            });

            quote! {
                let variant = u32::decode(decoder)?;
                match variant {
                    #(#variant_decodings,)*
                    _ => Err(oxicode::Error::InvalidData {
                        message: "Invalid enum variant"
                    })
                }
            }
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(input, "Decode cannot be derived for unions")
                .to_compile_error()
                .into();
        }
    };

    let expanded = quote! {
        impl #impl_generics_with_bounds oxicode::Decode for #name #ty_generics #where_clause {
            fn decode<__D: oxicode::de::Decoder<Context = ()>>(decoder: &mut __D) -> Result<Self, oxicode::Error> {
                #decode_body
            }
        }
    };

    TokenStream::from(expanded)
}
