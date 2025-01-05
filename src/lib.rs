use bevy::{
    app::{App, Plugin},
    prelude::SystemSet,
};
use collider::ColliderInteraction;
use spatial_query::filter::SystemSpatialQueryFilter;

pub mod bounded;
pub mod collider;
pub mod components;
pub mod spatial_query;
pub mod spatial_index;
// pub mod implementations;

pub mod prelude {}

/// Trait allows for easier to read generic code
pub trait ColliderGroup: Send + Sync + Sized + 'static {
    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + Send + Sync + 'static;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: Send + Sync + 'static;

    type Implementation: CollisionImplementation<Self>;

    type Filter: SystemSpatialQueryFilter<Self>;
}

pub trait CollisionImplementation<Group: ColliderGroup<Implementation = Self>>: Send + Sync + 'static {
}

pub struct WithColliderGroup<Group: ColliderGroup>(pub Group::Implementation);

impl<Group: ColliderGroup<Implementation: Plugin>> Plugin for WithColliderGroup<Group> {
    fn build(&self, app: &mut App) {
        Group::Implementation::build(&self.0, app);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionDetectionSet {
    First,
    Colliding,
    Last,
}
