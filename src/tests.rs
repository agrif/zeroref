use crate::named_static;
use crate::zeroref;

named_static! {
    static RAWREF: u32 = 50;
    static mut RAWMUT: u32 = 10;
}

#[test]
fn rawref() {
    use crate::backend::NamedStatic;
    assert_eq!(RAWREF::get_ref(), &50);
}

#[test]
fn rawmut() {
    use crate::backend::{NamedStatic, NamedStaticMut};
    assert_eq!(RAWMUT::get_ref(), &10);
    unsafe {
        *RAWMUT::get_mut() = 12;
    }
    assert_eq!(RAWMUT::get_ref(), &12);
}

zeroref! {
    static storage REF: &u32;
    static storage MUTREF: &mut u32;
    static storage BOX: u32;
}

#[test]
fn zero_sized() {
    let mut a = 42;
    {
        let z = REF.claim(&a);
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref()), 0);
    }
    {
        let mut z = MUTREF.claim(&mut a);
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref()), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref_mut()), 0);
    }
    {
        let mut z = BOX.claim(a);
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref()), 0);
        assert_eq!(core::mem::size_of_val(&z.zero_ref_mut()), 0);
    }
}

#[test]
fn stack_ref() {
    let a = 42;
    let z = REF.claim(&a);
    assert_eq!(*z, 42);
    assert_eq!(*z.zero_ref(), 42);
}

#[test]
fn stack_mut_ref() {
    let mut a = 42;
    {
        let mut z = MUTREF.claim(&mut a);
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

#[test]
fn boxed() {
    let mut z = BOX.claim(42);
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
    let a = z.get();
    assert_eq!(a, 46);
}
