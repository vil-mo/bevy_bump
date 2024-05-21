/// Type
pub trait PhysicsLayer {
    fn collides(&self, other: &Self) -> bool;
}

#[cfg(feature = "enumset")]
use enumset::{EnumSet, EnumSetType};

#[cfg(feature = "enumset")]
impl<T: EnumSetType> PhysicsLayer for EnumSet<T> {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        !self.is_disjoint(*other)
    }
}

impl PhysicsLayer for () {
    #[inline(always)]
    fn collides(&self, _: &Self) -> bool {
        true
    }
}

impl PhysicsLayer for u8 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl PhysicsLayer for u16 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl PhysicsLayer for u32 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl PhysicsLayer for u64 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl PhysicsLayer for u128 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl PhysicsLayer for usize {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}
