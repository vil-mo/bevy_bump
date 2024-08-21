pub mod collision_check;
pub mod components;
pub mod layer;
pub mod spacial_index;

use crate::core::collider::ColliderInteraction;
use crate::{core::ColliderGroup, utils::Bounded};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
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

// pub trait CollisionAppExtension: Sealed {
//     fn register_bodies<T: LayerGroup>(&mut self);
//     fn register_hitboxes<T: LayerGroup>(&mut self);
// }
//
// impl CollisionAppExtension for App {
//     fn register_bodies<T: LayerGroup>(&mut self) {
//
//     }
//
//     fn register_hitboxes<T: LayerGroup>(&mut self) {
//
//     }
// }
