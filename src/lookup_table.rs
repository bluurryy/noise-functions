mod table2;
mod table4;

pub(crate) use entry::{entry2, entry3, Entry};
pub(crate) use table2::{Index2, Table2};
pub(crate) use table4::{Index3, Table3};

#[cfg(feature = "nightly-simd")]
pub(crate) use table4::Index3x4;

macro_rules! const_assert {
    ($($tt:tt)*) => {
        const _: () = assert!($($tt)*);
    };
}

const_assert!(i32::BITS <= usize::BITS, "We cast i32 to usize for indexing.");

#[cfg(feature = "nightly-simd")]
mod entry {
    use core::simd::{f32x2, f32x4, LaneCount, Simd, SupportedLaneCount};

    const_assert!(
        core::mem::size_of::<f32x2>() == 8 && core::mem::align_of::<f32x2>() <= 8,
        "We assume this for the lookup code. Please let me know if this assertion ever fails."
    );

    const_assert!(
        core::mem::size_of::<f32x4>() == 16 && core::mem::align_of::<f32x4>() <= 16,
        "We assume this for the lookup code. Please let me know if this assertion ever fails."
    );

    #[repr(transparent)]
    pub(crate) struct Entry<const N: usize>(pub Simd<f32, N>)
    where
        LaneCount<N>: SupportedLaneCount;

    impl<const N: usize> Entry<N>
    where
        LaneCount<N>: SupportedLaneCount,
    {
        pub(crate) fn as_array(&self) -> &[f32; N] {
            self.0.as_array()
        }
    }

    pub(crate) const fn entry2(x: f32, y: f32) -> Entry<2> {
        Entry(f32x2::from_array([x, y]))
    }

    pub(crate) const fn entry3(x: f32, y: f32, z: f32) -> Entry<4> {
        Entry(f32x4::from_array([x, y, z, 0.0]))
    }
}

#[cfg(not(feature = "nightly-simd"))]
mod entry {
    #[repr(transparent)]
    pub(crate) struct Entry<const N: usize>([f32; N]);

    impl<const N: usize> Entry<N> {
        pub(crate) fn as_array(&self) -> &[f32; N] {
            &self.0
        }
    }

    pub(crate) const fn entry2(x: f32, y: f32) -> Entry<2> {
        Entry([x, y])
    }

    pub(crate) const fn entry3(x: f32, y: f32, z: f32) -> Entry<4> {
        Entry([x, y, z, 0.0])
    }
}
