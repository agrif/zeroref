use crate::zeroref;

zeroref! {
    static storage REF: &u32;
    static storage MUTREF: &mut u32;
    static storage BOX: u32;
}

#[test]
fn zero_sized() {
    let mut a = 42;
    {
        let z = REF.claim(& a).unwrap();
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.borrow()), 0);
    }
    {
        let mut z = MUTREF.claim(&mut a).unwrap();
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.borrow()), 0);
        assert_eq!(core::mem::size_of_val(&z.borrow_mut()), 0);
    }
    {
        let mut z = BOX.claim(a).unwrap();
        assert_eq!(core::mem::size_of_val(&z), 0);
        assert_eq!(core::mem::size_of_val(&z.borrow()), 0);
        assert_eq!(core::mem::size_of_val(&z.borrow_mut()), 0);
    }
}

#[test]
fn stack_ref() {
    let a = 42;
    let z = REF.claim(&a).unwrap();
    assert_eq!(*z, 42);
    assert_eq!(*z.borrow(), 42);
}

#[test]
fn stack_mut_ref() {
    let mut a = 42;
    {
        let mut z = MUTREF.claim(&mut a).unwrap();
        assert_eq!(*z, 42);
        assert_eq!(*z.borrow(), 42);
        *z += 2;
        assert_eq!(*z, 44);
        assert_eq!(*z.borrow(), 44);
        let mut zmut = z.borrow_mut();
        *zmut += 2;
        assert_eq!(*zmut, 46);
        assert_eq!(*z, 46);
        assert_eq!(*z.borrow(), 46);
    }
    assert_eq!(a, 46);
}

#[test]
fn boxed() {
    let mut z = BOX.claim(42).unwrap();
    assert_eq!(*z, 42);
    assert_eq!(*z.borrow(), 42);
    *z += 2;
    assert_eq!(*z, 44);
    assert_eq!(*z.borrow(), 44);
    let mut zmut = z.borrow_mut();
    *zmut += 2;
    assert_eq!(*zmut, 46);
    assert_eq!(*z, 46);
    assert_eq!(*z.borrow(), 46);
    let a = z.get();
    assert_eq!(a, 46);
}
