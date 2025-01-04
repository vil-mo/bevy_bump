use bevy::app::{App, Plugin};
use spatial_index::SpatialIndex;

pub mod query;
pub mod components;
pub mod spatial_index;

pub struct SpatialIndexPlugin<Group> {
    pub pixels_per_chunk: f32,

    marker: std::marker::PhantomData<fn() -> Group>,
}

impl<Group> Default for SpatialIndexPlugin<Group> {
    fn default() -> Self {
        Self::new(SpatialIndex::PIXELS_PER_CHUNK_DEFAULT)
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

impl<Group> Plugin for SpatialIndexPlugin<Group> {
    fn build(&self, app: &mut App) {
        app.add_resource(SpatialIndex::<Group>::new(self.pixels_per_chunk));
        // TODO
    }
}
