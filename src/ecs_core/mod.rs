pub mod collision_check;
pub mod components;
pub mod layer;
pub mod spacial_index;

use crate::{
    core::{collider::ColliderInteraction, ColliderGroup},
    utils::Bounded,
};
use bevy::{ecs::entity::Entity, math::bounding::Aabb2d};
use layer::CollisionLayer;

pub trait LayerGroup: CollisionLayer + Send + Sync + 'static {
    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + Bounded<Aabb2d> + Send + Sync + 'static;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: Bounded<Aabb2d> + Send + Sync + 'static;
}

impl<T: LayerGroup> ColliderGroup for T {
    type HurtboxData = Entity;
    type Hitbox = T::Hitbox;
    type Hurtbox = T::Hurtbox;
}
