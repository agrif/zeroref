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
    (@TYPE, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        $(#[$attr])*
        $($vis)* struct $N {}
    };
    (@REF, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        __named_static_internal!(@TYPE, $(#[$attr])* ($($vis)*) $N : $T = $v);
        impl $crate::backend::NamedStatic for $N {
            type Data = $T;
            fn get_ref<'a>() -> &'a Self::Data {
                static BACKING: $T = $v;
                &BACKING
            }
        }
    };
    (@MUT, $(#[$attr:meta])* ($($vis:tt)*) $N:ident : $T:ty = $v:expr) => {
        __named_static_internal!(@TYPE, $(#[$attr])* ($($vis)*) $N : $T = $v);
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
        static RAWREF: u32 = 50;
        static mut RAWMUT: u32 = 10;
    }

    #[test]
    fn rawref() {
        assert_eq!(RAWREF::get_ref(), &50);
    }

    #[test]
    fn rawmut() {
        assert_eq!(RAWMUT::get_ref(), &10);
        unsafe {
            *RAWMUT::get_mut() = 12;
        }
        assert_eq!(RAWMUT::get_ref(), &12);
    }
}
