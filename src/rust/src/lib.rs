use extendr_api::prelude::*;

/// Returns a copy with each element multiplied by two.
/// @export
#[extendr]
fn multiply_copy(doubles: &[Rfloat]) -> Doubles {
    let doubled = doubles.iter().map(|i| i * 2.0.into()).collect::<Doubles>();

    doubled
}

/// Multiplies each element by two in place.
/// @export
#[extendr]
fn multiply(doubles: Doubles) {
    let mut doubles = doubles;
    doubles.iter_mut().for_each(|i| *i *= 2.0);
}

/// Says hello.
#[extendr]
fn hello() {
    rprintln!("Hello from Rust!");
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod roverhead;
    fn multiply_copy;
    fn multiply;
    fn hello;
}
