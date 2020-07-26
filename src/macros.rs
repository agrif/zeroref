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

#[cfg(test)]
mod test {
    mod mod1 {
        pub mod mod2 {
            crate::zeroref! {
                static storage REF: &u32;
                pub static storage PREF: &u32;
                pub (super) static storage PSREF: &u32;

                static storage REFMUT: &mut u32;
                pub static storage PREFMUT: &mut u32;
                pub (super) static storage PSREFMUT: &mut u32;

                static storage OWNED: u32;
                pub static storage POWNED: u32;
                pub (super) static storage PSOWNED: u32;
            }

            #[test]
            fn private() {
                let _x = &REF;
                let _x = &REFMUT;
                let _x = &OWNED;
            }
        }

        #[test]
        fn qualified() {
            let _x = &mod2::PSREF;
            let _x = &mod2::PSREFMUT;
            let _x = &mod2::PSOWNED;
        }
    }

    #[test]
    fn public() {
        let _x = &mod1::mod2::PREF;
        let _x = &mod1::mod2::PREFMUT;
        let _x = &mod1::mod2::POWNED;
    }
}
