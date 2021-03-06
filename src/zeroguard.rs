use crate::backend::{Storage, StorageMut};
use crate::{ZeroRef, ZeroRefMut};

pub struct ZeroGuard<'d, S> where S: Storage<'d> {
    _mark: core::marker::PhantomData<
            lock_api::MutexGuard<'d, S::Mutex, S::Claim>>,
}

impl<'d, S> ZeroGuard<'d, S> where S: Storage<'d> {
    pub(crate) unsafe fn new() -> Self {
        ZeroGuard { _mark: core::marker::PhantomData }
    }

    pub fn zero_ref(&self) -> ZeroRef<'_, S, S::Data> {
        ZeroRef::new(self)
    }

    pub fn get(self) -> S::Claim {
        unsafe { S::unstore().unwrap() }
    }
}

impl<'d, S> ZeroGuard<'d, S> where S: StorageMut<'d> {
    pub fn zero_ref_mut(&mut self) -> ZeroRefMut<'_, S, S::Data> {
        ZeroRefMut::new(self)
    }
}

impl<'d, S> Drop for ZeroGuard<'d, S> where S: Storage<'d> {
    fn drop(&mut self) {
        use lock_api::RawMutex;
        unsafe {
            S::unstore();
            S::get_mutex().unlock();
        }
    }
}

// we have no address, we are always unpin
impl<'d, S> core::marker::Unpin for ZeroGuard<'d, S> where S: Storage<'d> {}

zero_ref_impls!(ZeroGuard<'a, S>);
zero_ref_mut_impls!(ZeroGuard<'a, S>);
