use crate::Zero;

pub unsafe trait Storage<'d>: Sized {
    type Claim: 'd;
    type Data: 'd;

    unsafe fn claim_and_store(value: Self::Claim) -> Option<()>;
    unsafe fn unclaim();
    unsafe fn get_ref() -> Option<&'d Self::Data>;

    fn claim(&self, value: Self::Claim) -> Option<Zero<'d, Self>> {
        Zero::new(value)
    }
}

pub unsafe trait StorageMut<'d>: Storage<'d> {
    unsafe fn get_mut() -> Option<&'d mut Self::Data>;
}

pub unsafe trait StorageOwned<'d>: StorageMut<'d> {
    unsafe fn unstore() -> Option<Self::Data>;
}
