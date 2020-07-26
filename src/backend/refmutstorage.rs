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

#[cfg(test)]
mod test {
    crate::zeroref! {
        static storage REFMUT: &mut u32;
    }

    #[test]
    fn zero_sized() {
        let mut a = 42;
        let mut z = REFMUT.claim(&mut a);
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref()), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref_mut()), 0);
    }

    #[test]
    fn stack_mut_ref() {
        let mut a = 42;
        {
            let mut z = REFMUT.claim(&mut a);
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
        }
        assert_eq!(a, 46);
    }
}
