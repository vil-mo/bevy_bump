use crate::bounded::{Bounded, Point, RoundedRectangle};
use bevy::math::{
    bounding::{Aabb2d, BoundingVolume},
    primitives::{Circle, Rectangle},
    Dir2, Vec2,
};

/// Trait allows `Self` to perform collisions with `T`
pub trait ColliderInteraction<T> {
    /// Returns true if `self` intersects with `other`
    fn intersect(&self, self_position: Vec2, other: &T, other_position: Vec2) -> bool;

    /// If `self` were to move along the `offset,
    /// returns distance until collision to other
    /// and normal of the collision
    fn cast(
        &self,
        self_position: Vec2,
        other: &T,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)>;
}

#[derive(Debug)]
pub struct Collider<'a, S> {
    pub shape: &'a S,
    pub position: Vec2,
}

impl<'a, S> Clone for Collider<'a, S> {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape,
            position: self.position,
        }
    }
}

impl<'a, S> Copy for Collider<'a, S> {}

impl<'a, S> Collider<'a, S> {
    pub fn new(shape: &'a S, position: Vec2) -> Self {
        Self { shape, position }
    }

    pub fn intersect<O>(&self, other: Collider<'a, O>) -> bool
    where
        S: ColliderInteraction<O>,
    {
        self.shape
            .intersect(self.position, other.shape, other.position)
    }
    pub fn cast<O>(
        &self,
        other: Collider<'a, O>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)>
    where
        S: ColliderInteraction<O>,
    {
        self.shape.cast(
            self.position,
            other.shape,
            other.position,
            offset_dir,
            offset_len,
        )
    }
}

impl<'a, S: Bounded<Aabb2d>> Bounded<Aabb2d> for Collider<'a, S> {
    fn bounding(&self) -> Aabb2d {
        let shape_bounding = self.shape.bounding();
        Aabb2d {
            min: shape_bounding.min + self.position,
            max: shape_bounding.max + self.position,
        }
    }
}

impl<C, T: ?Sized + ColliderInteraction<C>> ColliderInteraction<C> for Box<T> {
    fn intersect(&self, self_position: Vec2, other: &C, other_position: Vec2) -> bool {
        (**self).intersect(self_position, other, other_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &C,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        (**self).cast(self_position, other, other_position, offset_dir, offset_len)
    }
}

//
//
//
//
//
//
//
//
//

impl ColliderInteraction<Point> for Point {
    fn intersect(&self, self_position: Vec2, _other: &Point, other_position: Vec2) -> bool {
        self_position == other_position
    }

    fn cast(
        &self,
        self_position: Vec2,
        _other: &Point,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let diff = self_position - other_position;
        if diff.x / diff.y == offset_dir.x / offset_dir.y {
            let len = diff.length();
            if len < offset_len {
                return Some((len, -offset_dir));
            }
        }

        None
    }
}

impl ColliderInteraction<Rectangle> for Rectangle {
    fn intersect(&self, self_position: Vec2, other: &Rectangle, other_position: Vec2) -> bool {
        let self_aabb = Aabb2d {
            min: self_position - self.half_size,
            max: self_position + self.half_size,
        };
        let other_aabb = Aabb2d {
            min: other_position - other.half_size,
            max: other_position + other.half_size,
        };

        self_aabb.contains(&other_aabb)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Rectangle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let rect = Rectangle {
            half_size: other.half_size + self.half_size,
        };

        Point.cast(self_position, &rect, other_position, offset_dir, offset_len)
    }
}

impl ColliderInteraction<Rectangle> for Point {
    fn intersect(&self, self_position: Vec2, other: &Rectangle, other_position: Vec2) -> bool {
        let aabb = Aabb2d {
            min: other_position - other.half_size,
            max: other_position + other.half_size,
        };

        aabb.min.x <= self_position.x
            && self_position.x <= aabb.max.x
            && aabb.min.y <= self_position.y
            && self_position.y <= aabb.max.y
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Rectangle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let aabb = Aabb2d {
            min: other_position - other.half_size,
            max: other_position + other.half_size,
        };

        let (min_x, max_x) = if offset_dir.x.is_sign_positive() {
            (aabb.min.x, aabb.max.x)
        } else {
            (aabb.max.x, aabb.min.x)
        };
        let (min_y, max_y) = if offset_dir.y.is_sign_positive() {
            (aabb.min.y, aabb.max.y)
        } else {
            (aabb.max.y, aabb.min.y)
        };

        // Calculate the minimum/maximum time for each axis based on how much the direction goes that
        // way. These values can get arbitrarily large, or even become NaN, which is handled by the
        // min/max operations below
        let tmin_x = (min_x - self_position.x) / offset_dir.x;
        let tmin_y = (min_y - self_position.y) / offset_dir.y;
        let tmax_x = (max_x - self_position.x) / offset_dir.x;
        let tmax_y = (max_y - self_position.y) / offset_dir.y;

        // An axis that is not relevant to the ray direction will be NaN. When one of the arguments
        // to min/max is NaN, the other argument is used.
        // An axis for which the direction is the wrong way will return an arbitrarily large
        // negative value.
        let tmin = tmin_x.max(tmin_y).max(0.);
        let tmax = tmax_y.min(tmax_x).min(offset_len);

        if tmin <= tmax {
            let normal = if tmin == tmin_x {
                if offset_dir.x.is_sign_positive() {
                    Dir2::NEG_X
                } else {
                    Dir2::X
                }
            } else if tmin == tmin_y {
                if offset_dir.y.is_sign_positive() {
                    Dir2::NEG_Y
                } else {
                    Dir2::Y
                }
            } else {
                -offset_dir
            };

            Some((tmin, normal))
        } else {
            None
        }
    }
}

impl ColliderInteraction<Point> for Rectangle {
    #[inline]
    fn intersect(&self, self_position: Vec2, other: &Point, other_position: Vec2) -> bool {
        other.intersect(other_position, self, self_position)
    }

    #[inline]
    fn cast(
        &self,
        self_position: Vec2,
        other: &Point,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        other
            .cast(other_position, self, self_position, -offset_dir, offset_len)
            .map(|(len, normal)| (len, -normal))
    }
}

impl ColliderInteraction<Circle> for Circle {
    fn intersect(&self, self_position: Vec2, other: &Circle, other_position: Vec2) -> bool {
        (self.radius + other.radius).powi(2) >= self_position.distance_squared(other_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Circle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let circle = Circle::new(other.radius + self.radius);
        Point.cast(
            self_position,
            &circle,
            other_position,
            offset_dir,
            offset_len,
        )
    }
}

impl ColliderInteraction<Circle> for Point {
    fn intersect(&self, self_position: Vec2, other: &Circle, other_position: Vec2) -> bool {
        other.radius.powi(2) >= self_position.distance_squared(other_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Circle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let diff = self_position - other_position;
        let projected = diff.dot(*offset_dir);
        let closest_point = diff - projected * *offset_dir;
        let distance_squared = other.radius.powi(2) - closest_point.length_squared();
        if distance_squared < 0. || projected.powi(2).copysign(-projected) < -distance_squared {
            None
        } else {
            let toi = -projected - distance_squared.sqrt();
            if toi > offset_len {
                None
            } else if toi > 0. {
                let normal = (diff + offset_dir * toi) / other.radius;
                Some((toi, Dir2::new_unchecked(normal)))
            } else {
                Some((0., -offset_dir))
            }
        }
    }
}

impl ColliderInteraction<Point> for Circle {
    fn intersect(&self, self_position: Vec2, other: &Point, other_position: Vec2) -> bool {
        other.intersect(other_position, self, self_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Point,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        other
            .cast(other_position, self, self_position, -offset_dir, offset_len)
            .map(|(len, normal)| (len, -normal))
    }
}

impl ColliderInteraction<Rectangle> for Circle {
    fn intersect(&self, self_position: Vec2, other: &Rectangle, other_position: Vec2) -> bool {
        let rounded = RoundedRectangle {
            rect: Rectangle {
                half_size: other.half_size + Vec2::ONE * self.radius,
            },
            radius: self.radius,
        };
        Point.intersect(self_position, &rounded, other_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &Rectangle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let rounded = RoundedRectangle {
            rect: Rectangle {
                half_size: other.half_size + Vec2::ONE * self.radius,
            },
            radius: self.radius,
        };

        Point.cast(
            self_position,
            &rounded,
            other_position,
            offset_dir,
            offset_len,
        )
    }
}

impl ColliderInteraction<Circle> for Rectangle {
    fn intersect(&self, self_position: Vec2, other: &Circle, other_position: Vec2) -> bool {
        other.intersect(other_position, self, self_position)
    }

    fn cast(
            &self,
            self_position: Vec2,
            other: &Circle,
            other_position: Vec2,
            offset_dir: Dir2,
            offset_len: f32,
        ) -> Option<(f32, Dir2)> {
        other.cast(other_position, self, self_position, -offset_dir, offset_len)
            .map(|(len, normal)| (len, -normal))
    }
}

impl ColliderInteraction<RoundedRectangle> for Point {
    fn intersect(
        &self,
        self_position: Vec2,
        other: &RoundedRectangle,
        other_position: Vec2,
    ) -> bool {
        // TODO: this is wrong
        other.rect.intersect(other_position, self, self_position)
    }

    fn cast(
        &self,
        self_position: Vec2,
        other: &RoundedRectangle,
        other_position: Vec2,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> Option<(f32, Dir2)> {
        let Some((rect_dist, rect_norm)) = self.cast(
            self_position,
            &other.rect,
            other_position,
            offset_dir,
            offset_len,
        ) else {
            return None;
        };

        let collision_relative_to_other_pos = offset_dir * rect_dist - other_position;
        let before_radius = other.rect.half_size - Vec2::ONE * other.radius;
        let abs_collision = collision_relative_to_other_pos.abs();
        if abs_collision.x > before_radius.x && abs_collision.y > before_radius.y {
            self.cast(
                self_position,
                &Circle::new(other.radius),
                other_position,
                offset_dir,
                offset_len,
            )
        } else {
            Some((rect_dist, rect_norm))
        }
    }
}

// TODO: rounded rectangle implementations