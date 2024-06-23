# Decimal

My attempt at a large number library, written over the course of 36 hours in Rust.  
It supports the basic arithmetic operations and even some more advanced ones like exponentiation and logarithms.  
The `BigNum` struct can represent numbers up to around 10^10^308 and down to around 10^-10^308.
My approach to this was to just store the number's log10 and the sign, inspired by Aarex's large number library (logarithmica_numerus_lite.js).

I tried to make most of the algorithms by myself, and I only checked Aarex's implementation for the modulo function.  
This is definitely inferior to others' libraries, so if you're looking for one, you should use something like [break_infinity](https://crates.io/crates/break_infinity) or [break_eternity](https://crates.io/crates/break_eternity).  

## Running

The crate offers a demo binary that can be run with `cargo r -r`.  
It calculates 2 to the power of the nth fibonacci number, where n increases every iteration.  
It takes 1478 iterations for the number to overflow into infinity.  
It also outputs the time taken to overflow. Most of the time, this is nearly instant, even on my 10-year-old CPU.

## Testing

To run all the included tests, use `cargo test`.

## Project Structure

- `src/lib.rs` contains the main library code.
- `src/bignum/*.rs` contains the implementation of the `BigNum` struct. The file names are self-explanatory.
- `src/main.rs` contains the code for the demo binary.