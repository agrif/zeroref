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

macro_rules! storage_forward {
    ($T:tt < $($vars:tt),* >, $C:ty) => {
        impl <'d, $($vars),*> $T<$($vars),*>
        where
            Self: $crate::backend::Storage<'d, Claim=$C>,
            T: 'd,
        {
            pub fn try_claim(&self, value: $C)
                             -> Option<$crate::ZeroGuard<'d, Self>>
            {
                $crate::backend::Storage::try_claim(self, value)
            }
            pub fn claim(&self, value: $C) -> $crate::ZeroGuard<'d, Self> {
                $crate::backend::Storage::claim(self, value)
            }
        }
    };
}
