use extendr_api::prelude::*;

/// Adds two numbers.
/// @export
#[extendr]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Returns a copy with each element multiplied by two.
/// @export
#[extendr]
fn multiply_by_two_copy(integers: &[Rint]) -> Integers {
    let doubled = integers.iter().map(|i| i * 2.into()).collect::<Integers>();

    doubled
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod roverhead;
    fn add;
    fn multiply_by_two_copy;
}
