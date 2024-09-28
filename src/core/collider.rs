use crate::utils::Bounded;
use bevy::math::{bounding::Aabb2d, Dir2, Vec2};

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
        offset: Vec2,
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
    pub fn cast<O>(&self, other: Collider<'a, O>, offset: Vec2) -> Option<(f32, Dir2)>
    where
        S: ColliderInteraction<O>,
    {
        self.shape
            .cast(self.position, other.shape, other.position, offset)
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

// impl<T: Collider> Collider for Option<T> {}
//
// impl<C: Collider, T: ColliderInteraction<C>> ColliderInteraction<C> for Option<T> {
//     fn intersect(&self, self_position: Vec2, other: &C, other_position: Vec2) -> bool {
//         match self {
//             Some(actor) => actor.intersect(self_position, other, other_position),
//             None => false,
//         }
//     }
//
//     fn cast(
//         &self,
//         other: &C,
//         offset: Vec2,
//     ) -> Option<(f32, Dir2)> {
//         match self {
//             Some(actor) => actor.cast(self_position, other, other_position, offset),
//             None => None,
//         }
//     }
// }
//
// impl<C: Collider, T: ColliderInteraction<C>> ColliderInteraction<Option<C>> for T {
//     fn intersect(&self, other: &Option<C>) -> bool {
//         match other {
//             Some(other) => self.intersect(other),
//             None => false,
//         }
//     }
//
//     fn cast(&self, other: &Option<C>, offset: Vec2) -> Option<(f32, Direction2d)> {
//         match other {
//             Some(other) => self.cast(other, offset),
//             None => None,
//         }
//     }
// }

// impl Collider for Vec2 {}

// TODO: Macro for Collider implementations (look below)
// impl<C: Collider> ColliderInteraction<Option<C>> for Vec2
// where
//     Self: ColliderInteraction<C>,
// {
//     fn intersect(&self, other: &Option<C>) -> bool {
//         match other {
//             Some(other) => self.intersect(other),
//             None => false,
//         }
//     }
//
//     fn cast(&self, other: &Option<C>, offset: Vec2) -> Option<(f32, Dir2)> {
//         match other {
//             Some(other) => self.cast(other, offset),
//             None => None,
//         }
//     }
// }
