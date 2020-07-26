pub trait NamedStatic {
    type Data;
    fn get_ref<'a>() -> &'a Self::Data;
}

pub trait NamedStaticMut: NamedStatic {
    unsafe fn get_mut<'a>() -> &'a mut Self::Data;
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __named_static_internal {
    (@COMMON, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        $($vis)* struct $N {
            _mark: ::core::marker::PhantomData<()>,
        }
        #[allow(dead_code)]
        impl $N {
            pub const fn new() -> Self {
                $N { _mark: ::core::marker::PhantomData }
            }
        }
        impl ::core::ops::Deref for $N {
            type Target = $T;
            fn deref(&self) -> &Self::Target {
                <Self as $crate::backend::NamedStatic>::get_ref()
            }
        }
    };
    (@REF, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        __named_static_internal!(@COMMON, $(#[$attr])* ($($vis)*) $N : $T = $v);
        impl $crate::backend::NamedStatic for $N {
            type Data = $T;
            fn get_ref<'a>() -> &'a Self::Data {
                static BACKING: $T = $v;
                &BACKING
            }
        }
    };
    (@MUT, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        __named_static_internal!(@COMMON, $(#[$attr])* ($($vis)*) $N : $T = $v);
        impl $crate::backend::NamedStatic for $N {
            type Data = $T;
            fn get_ref<'a>() -> &'a Self::Data {
                use $crate::backend::NamedStaticMut;
                unsafe { Self::get_mut() }
            }
        }
        impl $crate::backend::NamedStaticMut for $N {
            unsafe fn get_mut<'a>() -> &'a mut Self::Data {
                static mut BACKING: $T = $v;
                &mut BACKING
            }
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! named_static {
    ($(#[$attr:meta])* static $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@REF, $(#[$attr])* () $N : $T = $v);
        named_static!($($t)*);
    };
    ($(#[$attr:meta])* pub static $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@REF, $(#[$attr])* (pub) $N : $T = $v);
        named_static!($($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@REF, $(#[$attr])* (pub ($($vis)+)) $N : $T = $v);
        named_static!($($t)*);
    };
    ($(#[$attr:meta])* static mut $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@MUT, $(#[$attr])* () $N : $T = $v);
        named_static!($($t)*);
    };
    ($(#[$attr:meta])* pub static mut $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@MUT, $(#[$attr])* (pub) $N : $T = $v);
        named_static!($($t)*);
    };
    ($(#[$attr:meta])* pub ($($vis:tt)+) static mut $N:ident : $T:ty = $v:expr; $($t:tt)*) => {
        __named_static_internal!(@MUT, $(#[$attr])* (pub ($($vis)+)) $N : $T = $v);
        named_static!($($t)*);
    };
    () => ()
}

#[cfg(test)]
mod test {
    use super::{NamedStatic, NamedStaticMut};

    crate::named_static! {
        static REF: u32 = 50;
        static mut MUT: u32 = 10;
    }

    #[test]
    fn zero_sized() {
        assert_eq!(core::mem::size_of::<REF>(), 0);
        assert_eq!(core::mem::size_of::<MUT>(), 0);
    }

    #[test]
    fn staticref() {
        assert_eq!(REF::get_ref(), &50);
    }

    #[test]
    fn staticderef() {
        assert_eq!(*REF::new(), 50);
    }

    #[test]
    fn staticmut() {
        assert_eq!(MUT::get_ref(), &10);
        unsafe {
            *MUT::get_mut() = 12;
        }
        assert_eq!(MUT::get_ref(), &12);
    }
}
