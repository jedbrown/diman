use quote::quote;
use syn::DeriveInput;

const ALLOWED_TYPES: &[&str] = &["i8", "i32", "i64"];

pub(crate) fn dimension_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let methods_impl: proc_macro2::TokenStream = dimension_methods_impl(&input).into();
    let output = quote! {
        #[derive(PartialEq, Eq, Clone, Debug, ::std::marker::ConstParamTy)]
        #input

        #methods_impl
    };
    output.into()
}

pub(crate) fn dimension_methods_impl(input: &DeriveInput) -> proc_macro::TokenStream {
    let type_name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();
    let panic_unexpected_type =
        || panic!("Found unexpected field type while deriving diman_dimension methods.");
    let mut field_names = vec![];
    if let syn::Data::Struct(s) = &input.data {
        if let syn::Fields::Named(fields) = &s.fields {
            for f in fields.named.iter() {
                if let syn::Type::Path(ref type_name) = f.ty {
                    if type_name.path.segments.len() != 1 {
                        panic_unexpected_type();
                    } else {
                        let only_segment_in_path = &type_name.path.segments[0];
                        let type_name = only_segment_in_path.ident.to_string();
                        if !ALLOWED_TYPES.contains(&type_name.as_str()) {
                            panic_unexpected_type();
                        }
                    }
                } else {
                    panic_unexpected_type();
                }
                field_names.push(f.ident.as_ref().unwrap().clone());
            }
        }
    }

    let none_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: 0,
            }
        })
        .collect();

    let mul_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: self.#ident + other.#ident,
            }
        })
        .collect();

    let div_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: self.#ident - other.#ident,
            }
        })
        .collect();

    let inv_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: -self.#ident,
            }
        })
        .collect();

    let powi_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: self.#ident * other,
            }
        })
        .collect();

    let sqrt_safety_gen: proc_macro2::TokenStream = field_names.iter().map(|ident| {
        quote! {
            if self.#ident % 2 != 0 {
                panic!("Cannot take square root of quantity with a dimension that is not divisible by 2 in all components.");
            }
        }
    }).collect();

    let sqrt_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: self.#ident / 2,
            }
        })
        .collect();

    let cbrt_safety_gen: proc_macro2::TokenStream = field_names.iter().map(|ident| {
        quote! {
            if self.#ident % 3 != 0 {
                panic!("Cannot take cubic root of quantity with a dimension that is not divisible by 3 in all components.");
            }
        }
    }).collect();

    let cbrt_gen: proc_macro2::TokenStream = field_names
        .iter()
        .map(|ident| {
            quote! {
                #ident: self.#ident / 3,
            }
        })
        .collect();

    let gen = quote! {
        impl #impl_generics #type_name #type_generics #where_clause {
            pub const fn none() -> Self {
                Self {
                    #none_gen
                }
            }

            pub const fn dimension_mul(self, other: Self) -> Self {
                Self {
                    #mul_gen
                }
            }

            pub const fn dimension_div(self, other: Self) -> Self {
                Self {
                    #div_gen
                }
            }

            pub const fn dimension_inv(self) -> Self {
                Self {
                    #inv_gen
                }
            }

            pub const fn dimension_powi(self, other: i32) -> Self {
                Self {
                    #powi_gen
                }
            }

            pub const fn dimension_sqrt(self) -> Self {
                #sqrt_safety_gen
                Self {
                    #sqrt_gen
                }
            }

            pub const fn dimension_cbrt(self) -> Self {
                #cbrt_safety_gen
                Self {
                    #cbrt_gen
                }
            }
        }
    };
    gen.into()
}
