use diman_derive_verify::Verify;
use syn::{punctuated::Punctuated, *};

use crate::expression::Expr;

pub enum Prefix {
    Ident(Ident),
    Lit(Lit),
}

pub struct Prefixes(pub Punctuated<Prefix, Token![,]>);

pub struct DimensionInt(pub Lit);

pub struct Factor(pub Lit);

pub struct Symbol(pub Lit);

#[derive(Verify)]
#[verified(crate::types::DimensionEntry)]
pub struct DimensionEntry {
    pub ident: Ident,
    pub value: DimensionInt,
}

#[derive(Verify)]
#[verified(crate::types::Dimensions)]
pub struct Dimensions {
    pub fields: Vec<DimensionEntry>,
}

pub type QuantityExpression = Expr<Ident>;

#[derive(Verify)]
#[verified(crate::types::UnitFactor)]
pub enum UnitFactor {
    UnitOrQuantity(Ident),
    Number(Factor),
}

pub type UnitExpression = Expr<UnitFactor>;

#[derive(Verify)]
#[verified(crate::types::QuantityDefinition)]
pub enum QuantityDefinition {
    Dimensions(Dimensions),
    Expression(QuantityExpression),
}

#[derive(Verify)]
#[verified(crate::types::UnitEntry)]
pub struct UnitEntry {
    pub name: Ident,
    pub symbol: Option<Symbol>,
    pub prefixes: Prefixes,
    pub rhs: UnitExpression,
}

#[derive(Verify)]
#[verified(crate::types::QuantityEntry)]
pub struct QuantityEntry {
    pub name: Ident,
    pub rhs: QuantityDefinition,
}

#[derive(Verify)]
#[verified(crate::types::ConstantEntry)]
pub struct ConstantEntry {
    pub name: Ident,
    pub rhs: UnitExpression,
}

pub enum Entry {
    Quantity(QuantityEntry),
    Unit(UnitEntry),
    Constant(ConstantEntry),
}

#[derive(Verify)]
#[verified(crate::types::UnresolvedDefs)]
pub struct Defs {
    pub dimension_type: Type,
    pub quantity_type: Type,
    pub quantities: Vec<QuantityEntry>,
    pub units: Vec<UnitEntry>,
    pub constants: Vec<ConstantEntry>,
}
