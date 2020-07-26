#![no_std]

mod macros;

pub mod backend;

#[macro_use]
mod zeroref;
pub use crate::zeroref::ZeroRef;

#[macro_use]
mod zerorefmut;
pub use zerorefmut::ZeroRefMut;

mod zeroguard;
pub use zeroguard::ZeroGuard;

#[cfg(test)]
mod tests;
