# A Little Slice of Pi

This assignment is the implementation of a few methods of calculating fundamental constants, like `e` and `pi`.

There is the de-facto equation for `e` implemented in `e.rs`.

Pi is calculated using several methods.

Euler's method is implemented in `euler.rs`, Madhava's in `madhava.rs`, Bailey-Borwein-Plouffe formula in `bbp.rs`,
and Viete's in `viete.rs`. To assist with Euler's and Madhava's computations, a Newton's method of the square root
approximation has been implemented in `newton.rs`.

## Building

Since this is a standard Cargo project, `cargo build` and `cargo run` will suffice. The implementations can also have
their results tested against the standard library constants, when `cargo test` is run.
