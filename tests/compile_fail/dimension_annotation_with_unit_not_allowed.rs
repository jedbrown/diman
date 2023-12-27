#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]

use ::diman::unit_system;
unit_system!(
    quantity_type Quantity,
    dimension_type Dimension,
    dimension Mass,
    #[base(Mass)]
    unit kilograms,
    unit foo: kilograms = kilograms,
);

fn main() {
}
