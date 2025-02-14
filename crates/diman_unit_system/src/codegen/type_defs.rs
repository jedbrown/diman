use crate::{
    storage_types::StorageType,
    types::{Defs, Dimensions},
};
use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

impl Defs {
    pub(crate) fn type_definition(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        let span = quantity_type.span();
        quote_spanned! {span =>
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Default)]
                #[repr(transparent)]
                pub struct #quantity_type<S: 'static, const D: #dimension_type>(pub(crate) S);
        }
    }

    pub(crate) fn type_functions(&self) -> TokenStream {
        let Self {
            quantity_type,
            dimension_type,
            ..
        } = &self;
        quote! {
            impl<S> #quantity_type<S, { #dimension_type::none() }> {
                /// Get the value of a dimensionless quantity
                pub fn value(self) -> S {
                    self.0
                }

                /// Get a reference to the value of a dimensionless quantity
                pub fn value_ref(&self) -> &S {
                    &self.0
                }
            }

            impl<S, const D: #dimension_type> #quantity_type<S, D> {
                /// Return the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked(self) -> S {
                    self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }

            impl<S> std::ops::Deref for #quantity_type<S, { #dimension_type::none() }> {
                type Target = S;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }

    pub fn get_dimension_expr(&self, dim: &Dimensions) -> TokenStream {
        let dimension_type = &self.dimension_type;
        let field_updates: TokenStream = dim
            .fields
            .iter()
            .map(|field| {
                let ident = &field.ident;
                let value = &field.value;
                quote! { #ident: #value, }
            })
            .collect();
        let span = self.quantity_type.span();
        quote_spanned! {span =>
                #dimension_type {
                    #field_updates
                    ..#dimension_type::none()
                }
        }
    }

    pub fn vector_definitions(&self) -> TokenStream {
        self.vector_types()
            .iter()
            .map(|vector_type| {
                self.definitions_for_storage_type(vector_type, &vector_type.module_name, false)
            })
            .collect()
    }

    pub fn float_definitions(&self) -> TokenStream {
        self.float_types()
            .iter()
            .map(|float_type| {
                self.definitions_for_storage_type(float_type, &float_type.module_name, true)
            })
            .collect()
    }

    pub fn definitions_for_storage_type<T: StorageType>(
        &self,
        type_: &T,
        module_name: &TokenStream,
        gen_constants: bool,
    ) -> TokenStream {
        let Self {
            dimension_type,
            quantity_type,
            ..
        } = &self;
        // TODO: The use statements here are quite hacky and will probably
        // not work if dimension is declared in a different place from
        // the macro invocation.
        let quantities = self.quantity_definitions_for_storage_type(type_);
        let constants = if gen_constants {
            self.constant_definitions_for_storage_type(type_)
        } else {
            quote! {}
        };
        quote! {
            pub mod #module_name {
                use super::#dimension_type;
                use super::#quantity_type;
                #quantities
                #constants
            }
        }
    }

    pub fn quantity_definitions_for_storage_type<T: StorageType>(&self, type_: &T) -> TokenStream {
        self.quantities
            .iter()
            .map(|quantity| {
                let dimension = self.get_dimension_expr(&quantity.dimension);
                let quantity_type = &self.quantity_type;
                let quantity_name = &quantity.name;
                let type_ = type_.name();
                let span = self.dimension_type.span();
                quote_spanned! {span =>
                    pub type #quantity_name = #quantity_type::<#type_, { #dimension }>;
                }
            })
            .collect()
    }

    pub fn constant_definitions_for_storage_type<T: StorageType>(&self, type_: &T) -> TokenStream {
        self
            .constants
            .iter()
            .map(|constant| {
                let dimension = self.get_dimension_expr(&constant.dimension);
                let quantity_type = &self.quantity_type;
                let constant_name = &constant.name;
                let value = constant.factor;
                let float_type = type_.base_storage();
                let type_ = type_.name();
                quote! {
                    pub const #constant_name: #quantity_type::<#type_, { #dimension }> = #quantity_type::<#type_, { #dimension }>(#value as #float_type);
                }
            })
            .collect()
    }
}
