mod bodies;
mod layer;
mod world;

use crate::core::collider::{Collider, ColliderInteraction, CollisionGroup};
use crate::ecs_core::layer::PhysicsLayer;
use crate::utils::Bounded;
use bevy::math::bounding::Aabb2d;

pub trait ECSCollisionGroup: 'static {
    /// Actor that is colliding
    type Actor: ColliderInteraction<Self::Target> + Bounded<Aabb2d> + Send + Sync;
    /// Bodies that generate collisions and usually stop actor's movement
    type Target: Collider + Bounded<Aabb2d> + Send + Sync;

    /// Layer of the bodies. Use `()` if you don't want group to use layers
    type Layer: PhysicsLayer + Send + Sync;
}

impl<T: ECSCollisionGroup> CollisionGroup for T {
    type Actor = T::Actor;
    type Target = T::Target;
}
