use extendr_api::prelude::*;

/// Adds two numbers.
/// @export
#[extendr]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod roverhead;
    fn add;
}
