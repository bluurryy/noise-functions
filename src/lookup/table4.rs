use core::ops::Index;

use crate::private_prelude::*;

#[cfg(feature = "nightly-simd")]
mod index3x4 {
    use super::*;

    /// 4 lanes of [`Index4<N>`]
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub(crate) struct Index3x4<const TABLE_SIZE: usize>(i32x4);

    impl<const TABLE_SIZE: usize> Index3x4<TABLE_SIZE> {
        #[allow(dead_code)]
        #[inline(always)]
        pub fn new(hash: i32x4) -> Self {
            Self(hash & i32x4::splat(Index3::<TABLE_SIZE>::MASK))
        }
    }

    impl<const TABLE_SIZE: usize> Index<usize> for Index3x4<TABLE_SIZE> {
        type Output = Index3<TABLE_SIZE>;

        fn index(&self, i: usize) -> &Self::Output {
            // SAFETY: Index3x4 ensures every element of satisfies Index3's invariants,
            //         Index3 and i32 have the same repr
            unsafe { &*(&self.0.as_array()[i] as *const i32).cast::<Index3<TABLE_SIZE>>() }
        }
    }
}

#[cfg(feature = "nightly-simd")]
pub(crate) use index3x4::Index3x4;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Index3<const TABLE_SIZE: usize>(i32);

impl<const TABLE_SIZE: usize> Index3<TABLE_SIZE> {
    pub const MASK: i32 = ((TABLE_SIZE * 4) as i32 - 1) & !3;

    #[inline(always)]
    pub fn new(hash: i32) -> Self {
        Self(hash & Self::MASK)
    }

    #[inline(always)]
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }
}

pub(crate) struct Table3<const TABLE_SIZE: usize>([Entry<4>; TABLE_SIZE]);

impl<const TABLE_SIZE: usize> Table3<TABLE_SIZE> {
    pub const fn new(values: [Entry<4>; TABLE_SIZE]) -> Self {
        assert!(TABLE_SIZE.is_power_of_two());
        Self(values)
    }
}

impl<const TABLE_SIZE: usize> Index<Index3<TABLE_SIZE>> for Table3<TABLE_SIZE> {
    type Output = Entry<4>;

    #[inline(always)]
    fn index(&self, index: Index3<TABLE_SIZE>) -> &Self::Output {
        // Safety:
        // index has been masked by Index3::MASK which makes it a multiple of 4 and inbounds
        unsafe { &*self.0.as_ptr().cast::<f32>().add(index.into_usize()).cast::<Entry<4>>() }
    }
}

const _: () = {
    assert!(core::mem::size_of::<Entry<4>>() == 16);
    assert!(core::mem::align_of::<Entry<4>>() <= 16);
};
