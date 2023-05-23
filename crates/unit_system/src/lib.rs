#![feature(proc_macro_diagnostic)]

mod expression;
mod parse;
mod types;
mod verify;
mod resolve;
mod dimension_math;
// mod storage_types;
// mod codegen;

use syn::*;
// use types::Defs;
use verify::Verify;

#[proc_macro]
pub fn unit_system(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let defs = parse_macro_input!(item as parse::types::Defs);
    let defs: types::UnresolvedDefs = defs.verify().unwrap();
    let resolved: types::ResolvedDefs = defs.resolve().unwrap_or_else(|_| panic!("Failed to resolve definitions. Recursive definitions?"));
    // defs.code_gen().into()
    proc_macro::TokenStream::new()
}
