use bevy::{
    app::{App, Plugin},
    math::bounding::Aabb2d,
};
use spatial_index::SpatialIndex;

use crate::{bounded::Bounded, ColliderGroup};

pub mod components;
pub mod query;
pub mod spatial_index;

pub trait SpatialIndexColliderGroup: ColliderGroup<Hitbox: Bounded<Aabb2d>, Hurtbox: Bounded<Aabb2d>> {}

impl<T: ColliderGroup<Hitbox: Bounded<Aabb2d>, Hurtbox: Bounded<Aabb2d>>> SpatialIndexColliderGroup for T {}

pub struct SpatialIndexPlugin<Group> {
    pub pixels_per_chunk: f32,
    marker: std::marker::PhantomData<fn() -> Group>,
}

impl<Group> Default for SpatialIndexPlugin<Group> {
    fn default() -> Self {
        Self::new(SpatialIndex::<Group>::PIXELS_PER_CHUNK_DEFAULT)
    }
}

impl<Group> SpatialIndexPlugin<Group> {
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpatialIndexPlugin {
            pixels_per_chunk,
            marker: std::marker::PhantomData,
        }
    }
}

impl<Group: ColliderGroup> Plugin for SpatialIndexPlugin<Group> {
    fn build(&self, _app: &mut App) {
        // app.add_resource(SpatialIndex::<Group>::new(self.pixels_per_chunk));
        // TODO
    }
}
