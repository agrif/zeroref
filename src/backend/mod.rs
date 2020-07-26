mod namedstatic;
pub use namedstatic::{NamedStatic, NamedStaticMut};

#[macro_use]
mod storage;
pub use storage::{Storage, StorageMut};

mod refstorage;
pub use refstorage::RefStorage;

mod refmutstorage;
pub use refmutstorage::RefMutStorage;

mod ownedstorage;
pub use ownedstorage::OwnedStorage;
