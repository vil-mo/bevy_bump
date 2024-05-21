use bevy::math::bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume};
use bevy::math::Vec2;

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
