use bevy::{app::App, prelude::SystemSet};
use collider::ColliderInteraction;

pub mod bounded;
pub mod collider;
pub mod spatial_query;
pub mod components;
pub mod implementations;
pub mod layer;

pub mod prelude {}

/// Trait allows for easier to read generic code
pub trait ColliderGroup: Send + Sync + Sized + 'static {
    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + Send + Sync + 'static;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: Send + Sync + 'static;

    type Implementation: CollisionImplimentation<Self>;
}

pub trait CollisionImplimentation<Group: ColliderGroup<Implementation = Self>> {
    fn build(self, app: &mut App);
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionDetectionSet {
    First,
    Colliding,
    Last,
}

