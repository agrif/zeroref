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

#[cfg(test)]
mod test {
    crate::zeroref! {
        static storage OWNED: u32;
    }

    #[test]
    fn zero_sized() {
        let a = 42;
        let mut z = OWNED.claim(a);
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref()), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref_mut()), 0);
    }

    #[test]
    fn owned() {
        let mut z = OWNED.claim(42);
        assert_eq!(*z, 42);
        assert_eq!(*z.zero_ref(), 42);
        *z += 2;
        assert_eq!(*z, 44);
        assert_eq!(*z.zero_ref(), 44);
        let mut zmut = z.zero_ref_mut();
        *zmut += 2;
        assert_eq!(*zmut, 46);
        assert_eq!(*z, 46);
        assert_eq!(*z.zero_ref(), 46);
        let a = z.get();
        assert_eq!(a, 46);
    }
}
