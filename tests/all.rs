#![allow(incomplete_features)]
#![feature(generic_const_exprs, adt_const_params)]
#![feature(const_fn_floating_point_arithmetic)]

pub mod example_system;
pub mod utils;

mod float;

mod type_aliases;

#[cfg(feature = "glam")]
mod glam;

#[cfg(feature = "mpi")]
mod mpi;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "rand")]
mod rand;

#[test]
#[cfg(feature = "f32")]
fn compile_fail_float() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/float_*.rs");
}

#[test]
#[cfg(feature = "glam-vec2")]
#[cfg(feature = "f32")]
fn compile_fail_glam() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/glam_*.rs");
}
