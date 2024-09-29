pub mod collision_check;
pub mod components;
pub mod layer;
pub mod spacial_index;

use crate::{
    core::ColliderGroup,
    utils::Bounded,
};
use bevy::{ecs::entity::Entity, math::bounding::Aabb2d, prelude::SystemSet};
use layer::CollisionLayer;

pub trait LayerGroup:
    ColliderGroup<
        HurtboxData = Entity,
        Hitbox: Bounded<Aabb2d> + Send + Sync,
        Hurtbox: Bounded<Aabb2d> + Send + Sync,
    > + CollisionLayer
    + Send
    + Sync
    + 'static
{
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionDetectionSet {
    First,
    Colliding,
    Last,
}
