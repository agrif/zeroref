use crate::backend::{Storage, StorageMut, NamedStaticMut};

#[derive(Clone, Copy, Default)]
pub struct OwnedStorage<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> OwnedStorage<S, T> {
    pub const fn new() -> Self {
        OwnedStorage(core::marker::PhantomData)
    }
}

storage_forward!(OwnedStorage<S, T>, T);

unsafe impl<'d, S, M, T> Storage<'d> for OwnedStorage<S, T>
where
    S: NamedStaticMut<Data=(M, Option<T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    type Claim = T;
    type Data = T;
    type Mutex = M;
    fn get_mutex() -> &'d M {
        &S::get_ref().0
    }
    unsafe fn store(value: Self::Claim) {
        S::get_mut().1 = Some(value);
    }
    unsafe fn unstore() -> Option<Self::Claim> {
        S::get_mut().1.take()
    }
    unsafe fn get_ref() -> &'d Self::Data {
        S::get_ref().1.as_ref().unwrap()
    }
}

unsafe impl<'d, S, M, T> StorageMut<'d> for OwnedStorage<S, T>
where
    S: NamedStaticMut<Data=(M, Option<T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    unsafe fn get_mut() -> &'d mut Self::Data {
        S::get_mut().1.as_mut().unwrap()
    }
}
