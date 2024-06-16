pub mod bodies;
pub mod collision_check;
pub mod layer;
pub mod world;

use crate::core::collider::{Collider, ColliderInteraction};
use crate::core::response::CollisionResponse;
use crate::{core::ColliderGroup, utils::Bounded};
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use layer::CollisionLayer;

pub trait LayerGroup: CollisionLayer + Clone + Send + Sync + 'static {
    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + Bounded<Aabb2d> + Send + Sync;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: Collider + Bounded<Aabb2d> + Send + Sync;
    type Response: CollisionResponse + Send + Sync;
}

impl<T: LayerGroup> ColliderGroup for T
where
    <Self as LayerGroup>::Hitbox: Bounded<Aabb2d> + Send + Sync,
    <Self as LayerGroup>::Hurtbox: Bounded<Aabb2d> + Send + Sync,
{
    type Hitbox = T::Hitbox;
    type Hurtbox = T::Hurtbox;
}

trait Sealed {}
impl Sealed for App {}

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
