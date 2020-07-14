#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __zeroref_internal {
    (@REF, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@TYPE, $(#[$attr])* ($($vis)*) $N);
        __zeroref_internal!(@BACKING, $N, &'static $T);
        __zeroref_internal!(@STORAGE, $N, &'d $T, $T, @REF);
        zeroref!($($t)*);
    };
    (@REFMUT, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@TYPE, $(#[$attr])* ($($vis)*) $N);
        __zeroref_internal!(@BACKING, $N, &'static mut $T);
        __zeroref_internal!(@STORAGE, $N, &'d mut $T, $T, @REF);
        __zeroref_internal!(@STORAGEMUT, $N, @REF);
        zeroref!($($t)*);
    };
    (@BOX, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@TYPE, $(#[$attr])* ($($vis)*) $N);
        __zeroref_internal!(@BACKING, $N, $T);
        __zeroref_internal!(@STORAGE, $N, $T, $T, @BOX);
        __zeroref_internal!(@STORAGEMUT, $N, @BOX);
        __zeroref_internal!(@STORAGEOWNED, $N);
        zeroref!($($t)*);
    };
    (@TYPE, $(#[$attr:meta])* ($($vis:tt)*) $N:ident) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        $($vis)* struct $N { __private_field: () }
        #[doc(hidden)]
        $($vis)* static $N: $N = $N { __private_field: () };
    };
    (@BACKING, $N:ident, $C:ty) => {
        impl $N {
            unsafe fn __backing() ->
                (&'static mut ::core::option::Option<$C>,
                 &'static mut ::core::sync::atomic::AtomicBool)
            {
                static mut DATA: ::core::option::Option<$C> =
                    ::core::option::Option::None;
                static mut LOCK: ::core::sync::atomic::AtomicBool
                    = ::core::sync::atomic::AtomicBool::new(false);
                (&mut DATA, &mut LOCK)
            }
        }
    };
    (@STORAGE, $N:ident, $C:ty, $D:ty, @$get_ref:tt) => {
        unsafe impl<'d> $crate::Storage<'d> for $N {
            type Claim = $C;
            type Data = $D;
            unsafe fn claim_and_store(value: Self::Claim)
                                      -> ::core::option::Option<()> {
                use ::core::sync::atomic::Ordering;
                use ::core::option::Option::*;
                let (data, lock) = Self::__backing();
                if lock.swap(true, Ordering::SeqCst) == false {
                    // was unclaimed before, but we claimed it
                    *data = Some(::core::mem::transmute(value));
                    // make sure that write is done
                    ::core::sync::atomic::compiler_fence(Ordering::SeqCst);
                    Some(())
                } else {
                    None
                }
            }
            unsafe fn unclaim() {
                use ::core::sync::atomic::Ordering;
                let (data, lock) = Self::__backing();
                if lock.swap(false, Ordering::SeqCst) == true {
                    // we were locked, now we're not
                    *data = ::core::option::Option::None;
                    // make sure that write is done
                    ::core::sync::atomic::compiler_fence(Ordering::SeqCst);
                }
            }
            unsafe fn get_ref() -> ::core::option::Option<&'d Self::Data> {
                let (data, _lock) = Self::__backing();
                __zeroref_internal!(@STORAGE_GET_REF, data, $D, @$get_ref)
            }
        }
    };
    (@STORAGE_GET_REF, $data:expr, $D:ty, @REF) => {
        $data.as_ref().map(|v| ::core::ptr::read(v) as & $D)
    };
    (@STORAGE_GET_REF, $data:expr, $D:ty, @BOX) => {
        $data.as_ref()
    };
    (@STORAGEMUT, $N:ident, @$get_mut:tt) => {
        unsafe impl<'d> $crate::StorageMut<'d> for $N {
            unsafe fn get_mut() -> ::core::option::Option<&'d mut Self::Data> {
                let (data, _lock) = Self::__backing();
                __zeroref_internal!(@STORAGE_GET_MUT, data, @$get_mut)
            }
        }
    };
    (@STORAGE_GET_MUT, $data:expr, @REF) => {
        $data.as_ref().map(|v| ::core::ptr::read(v))
    };
    (@STORAGE_GET_MUT, $data:expr, @BOX) => {
        $data.as_mut()
    };
    (@STORAGEOWNED, $N:ident) => {
        unsafe impl<'d> $crate::StorageOwned<'d> for $N {
            unsafe fn unstore() -> ::core::option::Option<Self::Data> {
                let (data, _lock) = Self::__backing();
                data.take()
            }
        }
    };
    () => ()
}

#[macro_export(local_inner_macros)]
macro_rules! zeroref {
    // ref storage
    ($(#[$attr:meta])* static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* () $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* (pub) $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* (pub ($($vis)+)) $N $T; $($t)*);
    };

    // mut ref storage
    ($(#[$attr:meta])* static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* () $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* (pub) $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* (pub ($($vis)+)) $N $T; $($t)*);
    };

    // box-like storage
    ($(#[$attr:meta])* static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* () $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* (pub) $N $T; $($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* (pub ($($vis)+)) $N $T; $($t)*);
    };

    () => ()
}
