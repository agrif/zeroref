#![no_std]

mod macros;

mod storage;
pub use storage::{Storage, StorageMut, StorageOwned};

#[macro_use]
mod zeroref;
pub use crate::zeroref::ZeroRef;

#[macro_use]
mod zerorefmut;
pub use zerorefmut::ZeroRefMut;

mod zero;
pub use zero::Zero;

#[cfg(test)]
mod tests;
