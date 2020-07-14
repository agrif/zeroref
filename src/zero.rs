use crate::{Storage, StorageMut, StorageOwned, ZeroRef, ZeroRefMut};

pub struct Zero<'d, S> where S: Storage<'d> {
    _mark: core::marker::PhantomData<S::Claim>,
}

impl<'d, S> Zero<'d, S> where S: Storage<'d> {
    pub(crate) fn new(value: S::Claim) -> Option<Self> {
        unsafe {
            S::claim_and_store(value)
                .map(|_| Zero { _mark: core::marker::PhantomData })
        }
    }

    pub fn borrow(&self) -> ZeroRef<'_, S, S::Data> {
        ZeroRef::new(self)
    }
}

impl<'d, S> Zero<'d, S> where S: StorageMut<'d> {
    pub fn borrow_mut(&mut self) -> ZeroRefMut<'_, S, S::Data> {
        ZeroRefMut::new(self)
    }
}

impl<'d, S> Zero<'d, S> where S: StorageOwned<'d> {
    pub fn get(self) -> S::Data {
        unsafe { S::unstore().unwrap() }
    }
}

// we have no address, we are always unpin
impl<'d, S> core::marker::Unpin for Zero<'d, S> where S: Storage<'d> {}

impl<'d, S> Drop for Zero<'d, S> where S: Storage<'d> {
    fn drop(&mut self) {
        unsafe { S::unclaim() }
    }
}

impl<'d, S> core::ops::Deref for Zero<'d, S> where S: Storage<'d> {
    type Target = S::Data;
    fn deref(&self) -> &Self::Target {
        unsafe { S::get_ref().unwrap() }
    }
}

impl<'d, S> core::ops::DerefMut for Zero<'d, S> where S: StorageMut<'d> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { S::get_mut().unwrap() }
    }
}
