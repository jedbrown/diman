pub(super) mod expression;
pub mod types;

use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Brace, Bracket, Paren},
    *,
};

use self::types::{
    ConstantEntry, Defs, DimensionEntry, DimensionInt, Dimensions, Entry, Factor, Prefix, Prefixes,
    QuantityDefinition, QuantityEntry, Symbol, UnitEntry, UnitExpression, UnitFactor,
};

impl Parse for Symbol {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for Factor {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for DimensionInt {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl Parse for Prefix {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::Ident(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Self::Lit(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for Prefixes {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: Bracket = bracketed!( content in input );
        Ok(Prefixes(content.parse_terminated(Prefix::parse)?))
    }
}

impl Parse for DimensionEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let value: DimensionInt = input.parse()?;
        Ok(Self { ident, value })
    }
}

impl Parse for UnitFactor {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Ident) {
            Ok(Self::UnitOrQuantity(input.parse()?))
        } else if lookahead.peek(Lit) {
            Ok(Self::Number(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for QuantityDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Brace) {
            Ok(Self::Dimensions(input.parse()?))
        } else {
            Ok(Self::Expression(input.parse()?))
        }
    }
}

impl Parse for UnitEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let name;
        let symbol;
        let mut prefixes = Prefixes(Punctuated::new());
        if lookahead.peek(Ident) {
            name = input.parse()?;
            symbol = None;
        } else if lookahead.peek(Paren) {
            let content;
            let _: token::Paren = parenthesized! { content in input };
            name = content.parse()?;
            let _: Token![,] = content.parse()?;
            symbol = Some(content.parse()?);
            let lookahead = content.lookahead1();
            if lookahead.peek(Token![,]) {
                let _: Token![,] = content.parse()?;
                prefixes = content.parse()?;
            } else if !content.is_empty() {
                return Err(lookahead.error());
            }
        } else {
            return Err(lookahead.error());
        }
        let _: Token![=] = input.parse()?;
        let rhs: UnitExpression = input.parse()?;
        Ok(Self {
            name,
            symbol,
            prefixes,
            rhs,
        })
    }
}

impl Parse for Dimensions {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        let _: token::Brace = braced!(content in input);
        let fields: Punctuated<DimensionEntry, Token![,]> =
            content.parse_terminated(DimensionEntry::parse)?;
        Ok(Self {
            fields: fields.into_iter().collect(),
        })
    }
}

impl Parse for QuantityEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _: Token![=] = input.parse()?;
        let rhs = input.parse()?;
        Ok(Self { name, rhs })
    }
}

impl Parse for ConstantEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;
        let _: Token![=] = input.parse()?;
        let rhs: UnitExpression = input.parse()?;
        Ok(Self { name, rhs })
    }
}

impl Parse for Entry {
    fn parse(input: ParseStream) -> Result<Self> {
        let keyword: Ident = input.parse()?;
        match keyword.to_string().as_str() {
            "def" => Ok(Self::Quantity(input.parse()?)),
            "unit" => Ok(Self::Unit(input.parse()?)),
            "constant" => Ok(Self::Constant(input.parse()?)),
            ident => Err(Error::new(
                keyword.span(),
                format!(
                    "Unexpected identifier: {}, expected \"def\", \"unit\" or \"constant\"",
                    ident
                ),
            )),
        }
    }
}

impl Parse for Defs {
    fn parse(input: ParseStream) -> Result<Self> {
        let quantity_type: Type = input.parse()?;
        let _: Token![,] = input.parse()?;
        let dimension_type: Type = input.parse()?;
        let _: Token![,] = input.parse()?;
        let content;
        let _: token::Bracket = bracketed!(content in input);
        let mut quantities = vec![];
        let mut units = vec![];
        let mut constants = vec![];
        for item in content
            .parse_terminated::<_, Token![,]>(Entry::parse)?
            .into_iter()
        {
            match item {
                Entry::Quantity(q) => quantities.push(q),
                Entry::Unit(u) => units.push(u),
                Entry::Constant(c) => constants.push(c),
            }
        }
        Ok(Self {
            dimension_type,
            quantity_type,
            quantities,
            units,
            constants,
        })
    }
}
