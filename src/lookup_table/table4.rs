use core::ops::Index;

#[cfg(feature = "nightly-simd")]
mod index4x4 {
    use core::{ops::Index, simd::i32x4};

    use super::Index4;

    /// 4 lanes of [`Index4<N>`]
    #[repr(transparent)]
    #[derive(Clone, Copy)]
    pub(crate) struct Index4x4<const TABLE_SIZE: usize>(i32x4);

    impl<const TABLE_SIZE: usize> Index4x4<TABLE_SIZE> {
        #[allow(dead_code)]
        #[inline(always)]
        pub fn new(hash: i32x4) -> Self {
            Self(hash & i32x4::splat(Index4::<TABLE_SIZE>::MASK))
        }
    }

    impl<const TABLE_SIZE: usize> Index<usize> for Index4x4<TABLE_SIZE> {
        type Output = Index4<TABLE_SIZE>;

        fn index(&self, i: usize) -> &Self::Output {
            // SAFETY: Index4x4 ensures every element of satisfies Index4's invariants,
            //         Index4 and i32 have the same repr
            unsafe { &*(&self.0.as_array()[i] as *const i32).cast::<Index4<TABLE_SIZE>>() }
        }
    }
}

#[cfg(feature = "nightly-simd")]
pub(crate) use index4x4::Index4x4;

use super::Entry;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct Index4<const TABLE_SIZE: usize>(i32);

impl<const TABLE_SIZE: usize> Index4<TABLE_SIZE> {
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

pub(crate) struct Table4<const TABLE_SIZE: usize>([Entry<4>; TABLE_SIZE]);

impl<const TABLE_SIZE: usize> Table4<TABLE_SIZE> {
    pub const fn new(values: [Entry<4>; TABLE_SIZE]) -> Self {
        assert!(TABLE_SIZE % 4 == 0 && TABLE_SIZE.is_power_of_two());
        Self(values)
    }
}

impl<const TABLE_SIZE: usize> Index<Index4<TABLE_SIZE>> for Table4<TABLE_SIZE> {
    type Output = Entry<4>;

    #[inline(always)]
    fn index(&self, index: Index4<TABLE_SIZE>) -> &Self::Output {
        // Safety:
        // index has been masked by Index3::MASK which makes it a multiple of 4 and inbounds
        unsafe { &*self.0.as_ptr().cast::<f32>().add(index.into_usize()).cast::<Entry<4>>() }
    }
}

impl<const TABLE_SIZE: usize> Index<i32> for Table4<TABLE_SIZE> {
    type Output = Entry<4>;

    #[inline(always)]
    fn index(&self, index: i32) -> &Self::Output {
        &self[Index4::new(index)]
    }
}

const _: () = {
    assert!(core::mem::size_of::<Entry<4>>() == 16);
    assert!(core::mem::align_of::<Entry<4>>() <= 16);
};
