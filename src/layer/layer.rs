/// Type
pub trait CollisionLayer {
    fn collides(&self, other: &Self) -> bool;
}

#[cfg(feature = "enumset_layer")]
use enumset::{EnumSet, EnumSetType};

#[cfg(feature = "enumset_layer")]
impl<T: EnumSetType> CollisionLayer for EnumSet<T> {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        !self.is_disjoint(*other)
    }
}

impl CollisionLayer for () {
    #[inline(always)]
    fn collides(&self, _: &Self) -> bool {
        true
    }
}

impl CollisionLayer for u8 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u16 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u32 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u64 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u128 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for usize {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}
