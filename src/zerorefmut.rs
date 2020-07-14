use crate::{Zero, Storage, StorageMut};

pub struct ZeroRefMut<'a, S, D> {
    _mark: core::marker::PhantomData<(fn(S) -> S, &'a mut D)>,
}

impl<'a, 'd, S> ZeroRefMut<'a, S, S::Data> where S: Storage<'d>, 'd: 'a {
    pub(crate) fn new(_zero: &'a mut Zero<'d, S>) -> Self {
        ZeroRefMut { _mark: core::marker::PhantomData }
    }
}

// we have no address, we are always unpin
impl<'a, S, D> core::marker::Unpin for ZeroRefMut<'a, S, D> {}

impl<'a, S> core::ops::Deref for ZeroRefMut<'a, S, S::Data>
where
    S: Storage<'a>,
{
    type Target = S::Data;
    fn deref(&self) -> &Self::Target {
        unsafe { S::get_ref().unwrap() }
    }
}

impl<'a, S> core::ops::DerefMut for ZeroRefMut<'a, S, S::Data>
where
    S: StorageMut<'a>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { S::get_mut().unwrap() }
    }
}
