use bevy::math::{
    bounding::{Aabb2d, Bounded2d, BoundingCircle, BoundingVolume},
    primitives::{Primitive2d, Rectangle},
    Isometry2d,
};

pub struct Point;

impl Primitive2d for Point {}
impl Bounded2d for Point {
    fn aabb_2d(&self, isometry: impl Into<Isometry2d>) -> Aabb2d {
        let isometry = isometry.into();
        Aabb2d {
            min: isometry.translation,
            max: isometry.translation,
        }
    }

    fn bounding_circle(&self, isometry: impl Into<Isometry2d>) -> BoundingCircle {
        let isometry = isometry.into();
        BoundingCircle::new(isometry.translation, 0.0)
    }
}

/// Rectangle with rounded corners.
pub struct RoundedRectangle {
    pub rect: Rectangle,
    pub radius: f32,
}

impl Primitive2d for RoundedRectangle {}

impl Bounded<Aabb2d> for RoundedRectangle {
    fn bounding(&self) -> Aabb2d {
        self.rect.bounding()
    }
}

/// A trait similar to [`Bounded2d`] except it is generic and
/// doesn't require user to provide implementations for translated and rotated shapes
pub trait Bounded<T: BoundingVolume> {
    fn bounding(&self) -> T;
}

mod bounded_impls {
    // Manually implement Bounded for all primitives implementing Bounded2d
    // To avoid conflicting implementations

    use super::{Bounded, Point};
    use bevy::math::{
        bounding::{Aabb2d, Bounded2d, BoundingCircle},
        primitives::*,
        Vec2,
    };

    macro_rules! impl_bounded_for_bounded2d {
    ($($t:ident,)*) => {
            $(
                impl Bounded<Aabb2d> for $t {
                    fn bounding(&self) -> Aabb2d {
                        Bounded2d::aabb_2d(self, Vec2::ZERO)
                    }
                }

                impl Bounded<BoundingCircle> for $t {
                    fn bounding(&self) -> BoundingCircle {
                        Bounded2d::bounding_circle(self, Vec2::ZERO)
                    }
                }
            )*
        };
    }

    macro_rules! impl_bounded_for_polygon {
        ($($t:ident,)*) => {
            $(
                impl<const N: usize> Bounded<Aabb2d> for $t<N> {
                    fn bounding(&self) -> Aabb2d {
                        Bounded2d::aabb_2d(self, Vec2::ZERO)
                    }
                }

                impl<const N: usize> Bounded<BoundingCircle> for $t<N> {
                    fn bounding(&self) -> BoundingCircle {
                        Bounded2d::bounding_circle(self, Vec2::ZERO)
                    }
                }
            )*
        };
    }

    impl_bounded_for_bounded2d!(
        Arc2d,
        BoxedPolygon,
        BoxedPolyline2d,
        Capsule2d,
        Circle,
        CircularSector,
        CircularSegment,
        Ellipse,
        Line2d,
        Plane2d,
        Rectangle,
        RegularPolygon,
        Rhombus,
        Segment2d,
        Triangle2d,
        Point,
    );

    impl_bounded_for_polygon!(Polygon, Polyline2d,);
}
