pub mod base_dimension;
pub mod expression;
pub mod prefixes;

use proc_macro2::Span;
use syn::*;

use self::{
    base_dimension::BaseDimension,
    expression::{BinaryOperator, Expr, Operator},
    prefixes::Prefix,
};
use crate::dimension_math::BaseDimensions;

#[cfg(feature = "rational-dimensions")]
pub use diman_lib::ratio::Ratio as Exponent;

#[cfg(not(feature = "rational-dimensions"))]
pub use i64 as Exponent;

#[derive(Clone)]
pub enum Factor<C> {
    Concrete(C),
    Other(Ident),
}

impl<C1: Clone> Factor<C1> {
    pub fn map_concrete<C2>(&self, f: impl Fn(C1) -> C2) -> Factor<C2> {
        match self {
            Factor::Concrete(c1) => Factor::Concrete(f(c1.clone())),
            Factor::Other(x) => Factor::Other(x.clone()),
        }
    }
}

#[derive(Clone)]
pub enum Definition<Base, C> {
    Base(Base),
    Expression(Expr<Factor<C>, Exponent>),
}

pub type DimensionFactor = Factor<One>;

#[derive(Clone)]
pub struct DimensionEntry {
    pub name: Ident,
    pub rhs: Definition<(), One>,
}

impl DimensionEntry {
    pub fn is_base_dimension(&self) -> bool {
        matches!(self.rhs, Definition::Base(()))
    }
}

pub struct BaseAttribute {
    pub attribute_span: Span,
    pub dimension: Ident,
}

#[derive(Clone)]
pub struct Alias {
    pub name: Ident,
}

#[derive(Clone)]
pub struct One;

#[derive(Clone)]
pub struct Symbol(pub Ident);

#[derive(Clone)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: Expr<Factor<f64>, Exponent>,
    pub dimension_annotation: Option<Ident>,
}

#[derive(Clone)]
pub struct UnitTemplate {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub aliases: Vec<Alias>,
    pub prefixes: Vec<Prefix>,
    pub dimension_annotation: Option<Ident>,
    pub definition: Definition<Ident, f64>,
}

#[derive(Clone)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub dimension_annotation: Option<Ident>,
    pub definition: Definition<Ident, f64>,
    pub autogenerated_from: Option<Ident>,
}

impl UnitTemplate {
    fn format_name(&self, prefix: Option<&Prefix>, alias: Option<&Alias>) -> Ident {
        let name = match alias {
            None => self.name.clone(),
            Some(alias) => alias.name.clone(),
        };
        match prefix {
            None => name,
            Some(prefix) => Ident::new(&format!("{}{}", prefix.name(), name), name.span()),
        }
    }

    fn format_symbol(&self, prefix: Option<&Prefix>, alias: Option<&Alias>) -> Option<Symbol> {
        // Prevent generating lots of non-unique symbols by only setting the symbol for the non-aliased units
        if alias.is_some() {
            None
        } else {
            match prefix {
                None => self.symbol.clone(),
                Some(prefix) => self.symbol.as_ref().map(|symbol| {
                    Symbol(Ident::new(
                        &format!("{}{}", prefix.short(), symbol.0),
                        self.name.span(),
                    ))
                }),
            }
        }
    }

    fn get_definition(
        &self,
        prefix: Option<&Prefix>,
        alias: Option<&Alias>,
    ) -> Definition<Ident, f64> {
        let factor = prefix.map(|prefix| prefix.factor()).unwrap_or(1.0);
        if alias.is_none() && prefix.is_none() {
            self.definition.clone()
        } else {
            Definition::Expression(Expr::Binary(BinaryOperator {
                lhs: Box::new(Expr::Value(expression::Factor::Value(Factor::Concrete(
                    factor,
                )))),
                rhs: expression::Factor::Value(Factor::Other(self.name.clone())),
                operator: Operator::Mul,
            }))
        }
    }

    fn expand_prefix_and_alias(&self, prefix: Option<&Prefix>, alias: Option<&Alias>) -> UnitEntry {
        let name = self.format_name(prefix, alias);
        let symbol = self.format_symbol(prefix, alias);
        let definition = self.get_definition(prefix, alias);
        let autogenerated_from = if prefix.is_some() || alias.is_some() {
            Some(self.name.clone())
        } else {
            None
        };
        UnitEntry {
            name,
            symbol,
            definition,
            dimension_annotation: self.dimension_annotation.clone(),
            autogenerated_from,
        }
    }

    fn expand(mut self) -> Vec<UnitEntry> {
        let mut prefixes: Vec<_> = self.prefixes.drain(..).map(Some).collect();
        prefixes.push(None);
        let mut aliases: Vec<_> = self.aliases.drain(..).map(Some).collect();
        aliases.push(None);
        prefixes
            .iter()
            .flat_map(|prefix| {
                aliases
                    .iter()
                    .map(|alias| self.expand_prefix_and_alias(prefix.as_ref(), alias.as_ref()))
            })
            .collect()
    }
}

pub struct Unresolved<U> {
    pub dimension_types: Vec<Ident>,
    pub quantity_types: Vec<Ident>,
    pub dimensions: Vec<DimensionEntry>,
    pub units: Vec<U>,
    pub constants: Vec<ConstantEntry>,
}

pub type UnresolvedTemplates = Unresolved<UnitTemplate>;
pub type UnresolvedDefs = Unresolved<UnitEntry>;

impl UnresolvedTemplates {
    pub fn expand_templates(self) -> UnresolvedDefs {
        let units = self
            .units
            .into_iter()
            .flat_map(|template| template.expand())
            .collect();
        UnresolvedDefs {
            dimension_types: self.dimension_types,
            quantity_types: self.quantity_types,
            dimensions: self.dimensions,
            units,
            constants: self.constants,
        }
    }
}

pub struct Dimension {
    pub name: Ident,
    pub dimensions: BaseDimensions,
}

#[derive(Clone)]
pub struct Unit {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub magnitude: f64,
    pub symbol: Option<Symbol>,
    pub is_base_unit: bool,
}

pub struct Constant {
    pub name: Ident,
    pub dimensions: BaseDimensions,
    pub magnitude: f64,
}

pub struct Defs {
    pub dimension_type: Ident,
    pub quantity_type: Ident,
    pub dimensions: Vec<Dimension>,
    pub units: Vec<Unit>,
    pub constants: Vec<Constant>,
    pub base_dimensions: Vec<BaseDimension>,
}

impl Defs {
    pub(crate) fn base_dimensions(&self) -> impl Iterator<Item = &Ident> {
        self.base_dimensions.iter().map(|x| &x.0)
    }
}
