#![no_std]

mod macros;

mod staticref;
pub use staticref::{StaticRef, StaticRefMut};

mod storage;
pub use storage::{Storage, StorageMut, Ref, MutRef, Owned};

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
