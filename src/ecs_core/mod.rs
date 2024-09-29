pub mod collision_check;
pub mod components;
pub mod layer;
pub mod spacial_index;

use crate::{
    core::{collider::ColliderInteraction, ColliderGroup},
    utils::Bounded,
};
use bevy::{app::Update, ecs::{entity::Entity, schedule::ScheduleLabel}, math::bounding::Aabb2d, prelude::SystemSet};
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

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionDetectionSet {
    First,
    Colliding,
    Last,
}

/// Implements ScheduleLabel
const COLLISION_DETECTION_SCHEDULE: Update = Update;