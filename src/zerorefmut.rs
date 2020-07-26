use crate::backend::{Storage, StorageMut};
use crate::ZeroGuard;

pub struct ZeroRefMut<'a, S, D> {
    _mark: core::marker::PhantomData<(fn(S) -> S, &'a mut D)>,
}

impl<'a, 'd, S> ZeroRefMut<'a, S, S::Data> where S: Storage<'d>, 'd: 'a {
    pub(crate) fn new(_zero: &'a mut ZeroGuard<'d, S>) -> Self {
        ZeroRefMut { _mark: core::marker::PhantomData }
    }
}

// we have no address, we are always unpin
impl<'a, S, D> core::marker::Unpin for ZeroRefMut<'a, S, D> {}

macro_rules! zero_ref_mut_impls {
    ($Ref:ty) => {
        impl<'a, S> core::ops::DerefMut for $Ref
        where
            S: StorageMut<'a>,
        {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe { S::get_mut() }
            }
        }

        // FIXME Borrow?
    };
}

zero_ref_impls!(ZeroRefMut<'a, S, S::Data>);
zero_ref_mut_impls!(ZeroRefMut<'a, S, S::Data>);
