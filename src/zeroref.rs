use crate::{Zero, Storage};

#[derive(Clone, Copy)]
pub struct ZeroRef<'a, S, D> {
    _mark: core::marker::PhantomData<(fn(S) -> S, &'a D)>,
}

impl<'a, 'd, S> ZeroRef<'a, S, S::Data> where S: Storage<'d>, 'd: 'a {
    pub(crate) fn new(_zero: &'a Zero<'d, S>) -> Self {
        ZeroRef { _mark: core::marker::PhantomData }
    }
}

// we have no address, we are always unpin
impl<'a, S, D> core::marker::Unpin for ZeroRef<'a, S, D> {}

macro_rules! zero_ref_impls {
    ($Ref:ty) => {
        impl<'a, S> core::ops::Deref for $Ref
        where
            S: Storage<'a>,
        {
            type Target = S::Data;
            fn deref(&self) -> &Self::Target {
                unsafe { S::get_ref() }
            }
        }

        // FIXME Borrow?

        // special -- does not delegate to underlying type
        impl<'a, S> core::fmt::Pointer for $Ref
        where
            S: Storage<'a>,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter)
                   -> Result<(), core::fmt::Error>
            {
                core::fmt::Pointer::fmt(&&**self, f)
            }
        }

        zero_ref_impls!(@FMT, $Ref, core::fmt::Binary);
        zero_ref_impls!(@FMT, $Ref, core::fmt::Debug);
        zero_ref_impls!(@FMT, $Ref, core::fmt::Display);
        zero_ref_impls!(@FMT, $Ref, core::fmt::LowerExp);
        zero_ref_impls!(@FMT, $Ref, core::fmt::LowerHex);
        zero_ref_impls!(@FMT, $Ref, core::fmt::Octal);
        zero_ref_impls!(@FMT, $Ref, core::fmt::UpperExp);
        zero_ref_impls!(@FMT, $Ref, core::fmt::UpperHex);

        zero_ref_impls!(@CMP, $Ref, $crate::ZeroRef<'b, T, T::Data>);
        zero_ref_impls!(@CMP, $Ref, $crate::ZeroRefMut<'b, T, T::Data>);
        zero_ref_impls!(@CMP, $Ref, $crate::Zero<'b, T>);

        impl<'a, S> core::cmp::Ord for $Ref
        where
            S: Storage<'a>,
            S::Data: core::cmp::Ord,
        {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering
            {
                (**self).cmp(&**other)
            }
            // other methods must use defaults, as we can't wrap
            // a &T back into a ZeroRef
        }

        impl<'a, S> core::cmp::Eq for $Ref
        where
            S: Storage<'a>,
            S::Data: core::cmp::Eq,
        {}

        impl<'a, S, U> core::convert::AsRef<U> for $Ref
        where
            S: Storage<'a>,
            S::Data: AsRef<U>,
            U: ?Sized,
        {
            fn as_ref(&self) -> &U {
                (&**self).as_ref()
            }
        }

        // FIXME Fn, FnMut and FnOnce are nightly-only

        impl<'a, S> core::hash::Hash for $Ref
        where
            S: Storage<'a>,
            S::Data: core::hash::Hash,
        {
            fn hash<H>(&self, state: &mut H) where H: core::hash::Hasher {
                (&**self).hash(state)
            }
            // hash_slice barely even makes sense here
        }

        // FIXME ToSocketAddrs is in std
    };
    (@FMT, $Ref:ty, $Trait:path) => {
        impl<'a, S> $Trait for $Ref
        where
            S: Storage<'a>,
            S::Data: $Trait,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter)
                   -> Result<(), core::fmt::Error>
            {
                <S::Data as $Trait>::fmt(&**self, f)
            }
        }
    };
    (@CMP, $Ref:ty, $Other:ty) => {
        impl<'a, 'b, S, T> core::cmp::PartialOrd<$Other>
            for $Ref
        where
            S: Storage<'a>,
            T: Storage<'b>,
            S::Data: core::cmp::PartialOrd<T::Data>,
        {
            fn partial_cmp(&self, other: &$Other)
                           -> Option<core::cmp::Ordering>
            {
                (**self).partial_cmp(&**other)
            }
            fn lt(&self, other: &$Other) -> bool {
                (**self).lt(&**other)
            }
            fn le(&self, other: &$Other) -> bool {
                (**self).le(&**other)
            }
            fn gt(&self, other: &$Other) -> bool {
                (**self).gt(&**other)
            }
            fn ge(&self, other: &$Other) -> bool {
                (**self).ge(&**other)
            }
        }

        impl<'a, 'b, S, T> core::cmp::PartialEq<$Other>
            for $Ref
        where
            S: Storage<'a>,
            T: Storage<'b>,
            S::Data: core::cmp::PartialEq<T::Data>,
        {
            fn eq(&self, other: &$Other) -> bool {
                (**self).eq(&**other)
            }
            fn ne(&self, other: &$Other) -> bool {
                (**self).ne(&**other)
            }
        }
    };
}

zero_ref_impls!(ZeroRef<'a, S, S::Data>);
