use proc_macro2::TokenStream;

use super::Codegen;

use quote::{quote, quote_spanned};

impl Codegen {
    pub(crate) fn gen_quantity(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
        let span = quantity_type.span();
        let functions = self.quantity_functions();
        quote_spanned! {span =>
            #[derive(Clone, Copy, Eq, Default)]
            #[repr(transparent)]
            pub struct #quantity_type<S, const D: #dimension_type>(pub(crate) S);
            #functions
        }
    }

    fn quantity_functions(&self) -> TokenStream {
        let dimension_type = &self.defs.dimension_type;
        let quantity_type = &self.defs.quantity_type;
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

                /// Return a reference to the value of a quantity, regardless of whether
                /// it is dimensionless or not. Use this carefully, since the
                /// result depends on the underlying base units
                pub fn value_unchecked_ref(&self) -> &S {
                    &self.0
                }

                /// Create a new quantity for the dimension with a given value.
                /// Use carefully, since the constructed quantity depends on the
                /// used base units.
                pub const fn new_unchecked(s: S) -> Self {
                    Self(s)
                }
            }

            impl<const D: #dimension_type, S> #quantity_type<S, D>
            where
                S: core::ops::Div<Magnitude, Output = S> + core::fmt::Debug,
            {
                pub fn value_in<const R: Magnitude>(self, _: Unit<D, R>) -> S {
                    dbg!(R.as_f64(), &self.value_unchecked_ref());
                    self.value_unchecked() / R
                }
            }

            impl<const D: #dimension_type> #quantity_type<f64, D> {
                pub fn round_in<const R: Magnitude>(self, unit: Unit<D, R>) -> f64 {
                    self.value_in(unit).round()
                }
            }

            impl<const D: #dimension_type> #quantity_type<f32, D> {
                pub fn round_in<const R: Magnitude>(self, unit: Unit<D, R>) -> f32 {
                    self.value_in(unit).round()
                }
            }


            impl<S> core::ops::Deref for #quantity_type<S, { #dimension_type::none() }> {
                type Target = S;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
        }
    }
}
