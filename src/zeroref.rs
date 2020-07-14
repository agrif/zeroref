use crate::{Zero, Storage};

pub struct ZeroRef<'a, S, D> {
    _mark: core::marker::PhantomData<(fn(S) -> S, &'a D)>,
}

impl<'a, 'd, S> ZeroRef<'a, S, S::Data> where S: Storage<'d>, 'd: 'a {
    pub(crate) fn new(_zero: &'a Zero<'d, S>) -> Self {
        ZeroRef { _mark: core::marker::PhantomData }
    }
}

// we have no address, we are always unpin
impl<'a, S, D> core::marker::Unpin for ZeroRef<'a, S, D> {}

impl<'a, S> core::ops::Deref for ZeroRef<'a, S, S::Data> where S: Storage<'a> {
    type Target = S::Data;
    fn deref(&self) -> &Self::Target {
        unsafe { S::get_ref().unwrap() }
    }
}
