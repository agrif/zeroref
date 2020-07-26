use crate::backend::NamedStaticMut;
use crate::ZeroGuard;

use lock_api::RawMutex;

pub unsafe trait Storage<'d>: Sized {
    type Claim: 'd;
    type Data: 'd;
    type Mutex: 'd + lock_api::RawMutex;

    fn get_mutex() -> &'d Self::Mutex;
    unsafe fn store(value: Self::Claim);
    unsafe fn unstore() -> Option<Self::Claim>;
    unsafe fn get_ref() -> &'d Self::Data;

    fn try_claim(&self, value: Self::Claim) -> Option<ZeroGuard<'d, Self>> {
        if Self::get_mutex().try_lock() {
            unsafe {
                Self::store(value);
                Some(ZeroGuard::new())
            }
        } else {
            None
        }
    }

    fn claim(&self, value: Self::Claim) -> ZeroGuard<'d, Self> {
        Self::get_mutex().lock();
        unsafe {
            Self::store(value);
            ZeroGuard::new()
        }
    }
}

pub unsafe trait StorageMut<'d>: Storage<'d> {
    unsafe fn get_mut() -> &'d mut Self::Data;
}

#[derive(Clone, Copy, Default)]
pub struct Ref<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> Ref<S, T> {
    pub const fn new() -> Self {
        Ref(core::marker::PhantomData)
    }
}

impl<'d, S, T> Ref<S, T> where Self: Storage<'d, Claim=&'d T>, T: 'd {
    pub fn try_claim(&self, value: &'d T) -> Option<ZeroGuard<'d, Self>> {
        Storage::try_claim(self, value)
    }
    pub fn claim(&self, value: &'d T) -> ZeroGuard<'d, Self> {
        Storage::claim(self, value)
    }
}

unsafe impl<'d, S, M, T> Storage<'d> for Ref<S, T>
where
    S: NamedStaticMut<Data=(M, Option<*const T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    type Claim = &'d T;
    type Data = T;
    type Mutex = M;
    fn get_mutex() -> &'d M {
        &S::get_ref().0
    }
    unsafe fn store(value: Self::Claim) {
        S::get_mut().1 = Some(core::mem::transmute(value))
    }
    unsafe fn unstore() -> Option<Self::Claim> {
        S::get_mut().1.take().map(|p| p.as_ref().unwrap())
    }
    unsafe fn get_ref() -> &'d Self::Data {
        S::get_ref().1.unwrap().as_ref().unwrap()
    }
}

#[derive(Clone, Copy, Default)]
pub struct MutRef<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> MutRef<S, T> {
    pub const fn new() -> Self {
        MutRef(core::marker::PhantomData)
    }
}

impl<'d, S, T> MutRef<S, T>
where
    Self: Storage<'d, Claim=&'d mut T>,
    T: 'd,
{
    pub fn try_claim(&self, value: &'d mut T) -> Option<ZeroGuard<'d, Self>> {
        Storage::try_claim(self, value)
    }
    pub fn claim(&self, value: &'d mut T) -> ZeroGuard<'d, Self> {
        Storage::claim(self, value)
    }
}

unsafe impl<'d, S, M, T> Storage<'d> for MutRef<S, T>
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

unsafe impl<'d, S, M, T> StorageMut<'d> for MutRef<S, T>
where
    S: NamedStaticMut<Data=(M, Option<*mut T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    unsafe fn get_mut() -> &'d mut Self::Data {
        S::get_mut().1.unwrap().as_mut().unwrap()
    }
}

#[derive(Clone, Copy, Default)]
pub struct Owned<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> Owned<S, T> {
    pub const fn new() -> Self {
        Owned(core::marker::PhantomData)
    }
}

impl<'d, S, T> Owned<S, T> where Self: Storage<'d, Claim=T>, T: 'd {
    pub fn try_claim(&self, value: T) -> Option<ZeroGuard<'d, Self>> {
        Storage::try_claim(self, value)
    }
    pub fn claim(&self, value: T) -> ZeroGuard<'d, Self> {
        Storage::claim(self, value)
    }
}

unsafe impl<'d, S, M, T> Storage<'d> for Owned<S, T>
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

unsafe impl<'d, S, M, T> StorageMut<'d> for Owned<S, T>
where
    S: NamedStaticMut<Data=(M, Option<T>)>,
    T: 'd,
    M: lock_api::RawMutex + 'd,
{
    unsafe fn get_mut() -> &'d mut Self::Data {
        S::get_mut().1.as_mut().unwrap()
    }
}
