use bevy::prelude::*;

/// A shape against which collisions are checked
/// Should be able to store global position of collider
pub trait Collider: Clone + 'static {
    /// Needed access for correct normal calculations
    fn position(&self) -> Vec2;

    /// After calling this method, `self.position` should return `to`
    fn set_position(&mut self, to: Vec2);
}

/// Trait allows `Self` to perform collisions with `T`
pub trait ColliderInteraction<T: Collider>: Collider {
    /// Returns true if `self` intersects with `other`
    fn intersect(&self, other: &T) -> bool;

    /// If `self` were to move along the `offset,
    /// returns distance until collision to other
    /// and normal of the collision
    fn cast(&self, other: &T, offset: Vec2) -> Option<(f32, Direction2d)>;
}

impl<T: Collider> Collider for Option<T> {
    /// Returns [`Vec2::NAN`] if `None`
    fn position(&self) -> Vec2 {
        match self {
            Some(collider) => collider.position(),
            None => Vec2::NAN,
        }
    }

    /// Sets position [b]only[/b] if `Some`
    fn set_position(&mut self, to: Vec2) {
        if let Some(collider) = self {
            collider.set_position(to);
        }
    }
}

impl<C: Collider, T: ColliderInteraction<C>> ColliderInteraction<C> for Option<T> {
    fn intersect(&self, other: &C) -> bool {
        match self {
            Some(actor) => actor.intersect(other),
            None => false,
        }
    }

    fn cast(&self, other: &C, offset: Vec2) -> Option<(f32, Direction2d)> {
        match self {
            Some(actor) => actor.cast(other, offset),
            None => None,
        }
    }
}

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

impl Collider for Vec2 {
    fn position(&self) -> Vec2 {
        *self
    }

    fn set_position(&mut self, to: Vec2) {
        *self = to;
    }
}

/// TODO: Macro for Collider implementations (look below)
impl<C: Collider> ColliderInteraction<Option<C>> for Vec2
where
    Self: ColliderInteraction<C>,
{
    fn intersect(&self, other: &Option<C>) -> bool {
        match other {
            Some(other) => self.intersect(other),
            None => false,
        }
    }

    fn cast(&self, other: &Option<C>, offset: Vec2) -> Option<(f32, Direction2d)> {
        match other {
            Some(other) => self.cast(other, offset),
            None => None,
        }
    }
}
