#![no_std]

mod macros;

mod storage;
pub use storage::*;

mod zero;
pub use zero::*;

mod zeroref;
pub use crate::zeroref::*;

mod zerorefmut;
pub use zerorefmut::*;

#[cfg(test)]
mod tests;
