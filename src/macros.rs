#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __zeroref_internal {
    (@REF, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty) => {
        $crate::named_static! {
            #[doc(hidden)]
            $($vis)* static mut $N:
            (::spinning_top::RawSpinlock, Option<*const $T>)
                = (::lock_api::RawMutex::INIT, None);
        }
        $(#[$attr])*
        $($vis)* static $N: $crate::backend::RefStorage<$N, $T>
            = $crate::backend::RefStorage::new();
    };
    (@REFMUT, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty) => {
        $crate::named_static! {
            #[doc(hidden)]
            $($vis)* static mut $N:
            (::spinning_top::RawSpinlock, Option<*mut $T>)
                = (::lock_api::RawMutex::INIT, None);
        }
        $(#[$attr])*
        $($vis)* static $N: $crate::backend::RefMutStorage<$N, $T>
            = $crate::backend::RefMutStorage::new();
    };
    (@BOX, $(#[$attr:meta])* ($($vis:tt)*) $N:ident $T:ty) => {
        $crate::named_static! {
            #[doc(hidden)]
            $($vis)* static mut $N:
            (::spinning_top::RawSpinlock, Option<$T>)
                = (::lock_api::RawMutex::INIT, None);
        }
        $(#[$attr])*
        $($vis)* static $N: $crate::backend::OwnedStorage<$N, $T>
            = $crate::backend::OwnedStorage::new();
    };
}

#[macro_export(local_inner_macros)]
macro_rules! zeroref {
    // ref storage
    ($(#[$attr:meta])* static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* () $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* (pub) $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : & $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REF, $(#[$attr])* (pub ($($vis)+)) $N $T);
        zeroref!{ $($t)* }
    };

    // mut ref storage
    ($(#[$attr:meta])* static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* () $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* (pub) $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : & mut $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@REFMUT, $(#[$attr])* (pub ($($vis)+)) $N $T);
        zeroref!{ $($t)* }
    };

    // box-like storage
    ($(#[$attr:meta])* static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* () $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* (pub) $N $T);
        zeroref!{ $($t)* }
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static storage $N:ident : $T:ty; $($t:tt)*) => {
        __zeroref_internal!(@BOX, $(#[$attr])* (pub ($($vis)+)) $N $T);
        zeroref!{ $($t)* }
    };

    () => ()
}
