use bevy::math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume};
use bevy::math::Vec2;
use std::cmp::{Ordering, Ordering::*};

/// A trait similar to [`Bounded2d`] except it is generic and
/// doesn't require user to provide implementations for translated and rotated shapes
pub trait Bounded<T: BoundingVolume> {
    fn bounding(&self) -> T;
}

impl<T: Bounded2d> Bounded<Aabb2d> for T {
    fn bounding(&self) -> Aabb2d {
        self.aabb_2d(Vec2::ZERO, 0.0)
    }
}

impl<T: Bounded2d> Bounded<BoundingCircle> for T {
    fn bounding(&self) -> BoundingCircle {
        self.bounding_circle(Vec2::ZERO, 0.0)
    }
}

/// Sorts a list iteratively using comparisons. In an ascending sort order, when a smaller value is encountered, it is moved lower in the list until it is larger than the item before it.
///
/// This is relatively slow for large lists, but very efficient in cases where the list is already mostly sorted.
pub fn insertion_sort<T>(items: &mut [T], comparison: fn(&T, &T) -> Ordering) {
    for i in 1..items.len() {
        let mut j = i;
        while j > 0 && comparison(&items[j - 1], &items[j]) == Greater {
            items.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Based on the [`[T]::binary_search_by`], but returns first index that is equal or greater than value or equal to it
/// or the size of the slice if there is no such index.
/// Function should be like `(argument) argument (comparison sign) constant`
pub fn binary_search<T, F>(slice: &[T], mut f: F) -> usize
where
    F: FnMut(&T) -> Ordering,
{
    // INVARIANTS:
    // - 0 <= left <= left + size = right <= self.len()
    // - f returns Less for everything in self[..left]
    // - f returns Greater for everything in self[right..]
    let mut size = slice.len();
    let mut left = 0;
    let mut right = size;

    while left < right {
        let mid = left + size / 2;

        // SAFETY: the while condition means `size` is strictly positive, so
        // `size/2 < size`. Thus, `left + size/2 < left + size`, which
        // coupled with the `left + size <= self.len()` invariant means
        // we have `left + size/2 < self.len()`, and this is in-bounds.
        let cmp = f(unsafe { slice.get_unchecked(mid) });

        // SAFETY: same as above
        // `size/2 - 1 < size`. Thus, `left + size/2 - 1 < left + size`
        // and `left + size/2 - 1 != 0
        if cmp != Less && (mid == 0 || unsafe { f(slice.get_unchecked(mid - 1)) } == Less) {
            return mid;
        }

        // IDK if it is still true with changes in code, but I'll let it be as it is
        // This control flow produces conditional moves, which results in
        // fewer branches and instructions than if/else or matching on
        // cmp::Ordering.
        // This is x86 asm for u8: https://rust.godbolt.org/z/698eYffTx.
        left = if cmp == Less { mid + 1 } else { left };
        right = if cmp != Less { mid } else { right };

        size = right - left;
    }

    return right;
}

#[cfg(test)]
mod tests {
    use super::{binary_search, insertion_sort};
    #[test]
    fn binary_search_comparing_less_than_result() {
        let a = [0, 2, 4, 6, 6, 6, 8, 10];
        assert_eq!(binary_search(&a, |v| v.cmp(&5)), 3);
    }

    #[test]
    fn binary_search_comparing_equal_to_result() {
        let a = [0, 2, 4, 6, 6, 6, 8, 10];
        assert_eq!(binary_search(&a, |v| v.cmp(&6)), 3);
        assert_eq!(binary_search(&a, |v| v.cmp(&10)), 7);
        assert_eq!(binary_search(&a, |v| v.cmp(&0)), 0);
    }

    #[test]
    fn binary_search_comparing_greater_than_anything() {
        let a = [0, 2, 4, 6, 6, 6, 8, 10];
        assert_eq!(binary_search(&a, |v| v.cmp(&50)), 8);
    }

    #[test]
    fn binary_search_comparing_less_than_anything() {
        let a = [0, 2, 4, 6, 6, 6, 8, 10];
        assert_eq!(binary_search(&a, |v| v.cmp(&-50)), 0);
    }

    #[test]
    fn insertion_sort_test() {
        let mut a = [10, 4, 2, 13, -2, 45, 5];
        insertion_sort(&mut a, Ord::cmp);
        assert_eq!(a, [-2, 2, 4, 5, 10, 13, 45]);
    }
}
