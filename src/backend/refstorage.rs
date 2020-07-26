use crate::backend::{Storage, NamedStaticMut};

#[derive(Clone, Copy, Default)]
pub struct RefStorage<S, T>(core::marker::PhantomData<(S, T)>);

impl<S, T> RefStorage<S, T> {
    pub const fn new() -> Self {
        RefStorage(core::marker::PhantomData)
    }
}

storage_forward!(RefStorage<S, T>, &'d T);

unsafe impl<'d, S, M, T> Storage<'d> for RefStorage<S, T>
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
