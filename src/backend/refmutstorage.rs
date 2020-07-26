use crate::backend::{Storage, StorageMut, NamedStaticMut};

#[derive(Clone, Copy, Default)]
pub struct RefMutStorage<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> RefMutStorage<S, T> {
    pub const fn new() -> Self {
        RefMutStorage(core::marker::PhantomData)
    }
}

storage_forward!(RefMutStorage<S, T>, &'d mut T);

unsafe impl<'d, S, M, T> Storage<'d> for RefMutStorage<S, T>
where
    S: NamedStaticMut<Data=(M, Option<*mut T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    type Claim = &'d mut T;
    type Data = T;
    type Mutex = M;
    fn get_mutex() -> &'d M {
        &S::get_ref().0
    }
    unsafe fn store(value: Self::Claim) {
        S::get_mut().1 = Some(core::mem::transmute(value))
    }
    unsafe fn unstore() -> Option<Self::Claim> {
        S::get_mut().1.take().map(|p| p.as_mut().unwrap())
    }
    unsafe fn get_ref() -> &'d Self::Data {
        S::get_ref().1.unwrap().as_ref().unwrap()
    }
}

unsafe impl<'d, S, M, T> StorageMut<'d> for RefMutStorage<S, T>
where
    S: NamedStaticMut<Data=(M, Option<*mut T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    unsafe fn get_mut() -> &'d mut Self::Data {
        S::get_mut().1.unwrap().as_mut().unwrap()
    }
}
