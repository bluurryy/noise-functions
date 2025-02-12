use core::ops::Index;

use super::Entry;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Index2<const TABLE_SIZE: usize>(i32);

impl<const TABLE_SIZE: usize> Index2<TABLE_SIZE> {
    pub const MASK: i32 = ((TABLE_SIZE * 2) as i32 - 1) & !1;
    pub const MASK_64: i64 = ((TABLE_SIZE * 2) as i64 - 1) & !1;

    #[inline(always)]
    pub fn new(hash: i32) -> Self {
        Self(hash & Self::MASK)
    }

    #[inline(always)]
    pub fn new_64(hash: i64) -> Self {
        Self((hash & Self::MASK_64) as i32)
    }

    #[inline(always)]
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
}

pub(crate) struct Table2<const TABLE_SIZE: usize>(pub [Entry<2>; TABLE_SIZE]);

impl<const TABLE_SIZE: usize> Table2<TABLE_SIZE> {
    pub const fn new(values: [Entry<2>; TABLE_SIZE]) -> Self {
        assert!(TABLE_SIZE % 2 == 0 && TABLE_SIZE.is_power_of_two());
        Self(values)
    }
}

impl<const TABLE_SIZE: usize> Index<Index2<TABLE_SIZE>> for Table2<TABLE_SIZE> {
    type Output = Entry<2>;

    #[inline(always)]
    fn index(&self, index: Index2<TABLE_SIZE>) -> &Self::Output {
        // Safety:
        // index has been masked by Index2::MASK which makes it a multiple of 2 and inbounds
        unsafe { &*self.0.as_ptr().cast::<f32>().add(index.into_usize()).cast::<Entry<2>>() }
    }
}

impl<const TABLE_SIZE: usize> Index<i32> for Table2<TABLE_SIZE> {
    type Output = Entry<2>;

    #[inline(always)]
    fn index(&self, index: i32) -> &Self::Output {
        &self[Index2::new(index)]
    }
}

impl<const TABLE_SIZE: usize> Index<i64> for Table2<TABLE_SIZE> {
    type Output = Entry<2>;

    #[inline(always)]
    fn index(&self, index: i64) -> &Self::Output {
        &self[Index2::new_64(index)]
    }
}
