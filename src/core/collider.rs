use crate::core::layer::CollisionLayer;
use crate::core::response::CollisionResponse;
use bevy::prelude::*;
use crate::core::broad_phase::BroadPhase;


/// Any shape that interactions are checked against
/// Should be able to store position of collider
pub trait Collider {
    /// Needed for correct normal calculations
    fn position(&self) -> Vec2;
    
    /// Normal with the line, which ends are at `from` and `self.position()`
    fn normal(&self, from: Vec2) -> Direction2d;
}


pub trait ColliderInteraction<T: Collider>: Collider {
    fn intersect(&self, other: &T) -> bool;
    fn cast(&self, other: &T, offset: Vec2) -> Option<f32>;
}

impl<T: Collider + ?Sized> Collider for Box<T> {
    fn position(&self) -> Vec2 {
        self.position()
    }
    fn normal(&self, from: Vec2) -> Direction2d {
        self.normal(from)
    }
}

pub trait CollisionGroup: 'static {
    type Actor: ColliderInteraction<Self::Target>;
    type Target: Collider;
}


#[derive(Debug, Copy, Clone)]
pub struct ColliderInformation<'a, C: Collider, L: CollisionLayer> {
    pub collider: &'a C,
    pub layer: L,
}


impl<'a, C, L> AsRef<C> for ColliderInformation<'a, C, L>
where
    C: Collider,
    L: CollisionLayer,
{
    fn as_ref(&self) -> &C {
        self.collider
    }
}

pub struct Actor<'a, Group: CollisionGroup, L: CollisionLayer, BF: BroadPhase<'a, Group, L>> {
    collider: Group::Actor,
    layer: L,
    response: CollisionResponse<'a, Group, L, BF>,
}
