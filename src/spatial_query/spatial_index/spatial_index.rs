use bevy::ecs::entity::MapEntities;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Resource)]
pub struct SpatialIndex<Group> {
    chunks: HashMap<IVec2, Vec<Entity>>,
    pixels_per_chunk: f32,
    marker: std::marker::PhantomData<fn() -> Group>,
}

impl<Group> Default for SpatialIndex<Group> {
    fn default() -> Self {
        Self::new(Self::PIXELS_PER_CHUNK_DEFAULT)
    }
}

impl<Group> MapEntities for SpatialIndex<Group> {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for (_, chunk) in self.chunks.iter_mut() {
            for entity in chunk.iter_mut() {
                *entity = entity_mapper.map_entity(*entity);
            }
        }
    }
}

/// Coordinates of the chunk in which the `global` lies in
fn global_to_chunk(pixels_per_chunk: f32, global: Vec2) -> IVec2 {
    IVec2 {
        x: (global.x / pixels_per_chunk).floor() as i32,
        y: (global.y / pixels_per_chunk).floor() as i32,
    }
}

/// Bottom left corner of the chunk
fn chunk_to_global(pixels_per_chunk: f32, chunk: IVec2) -> Vec2 {
    Vec2 {
        x: chunk.x as f32 * pixels_per_chunk,
        y: chunk.y as f32 * pixels_per_chunk,
    }
}

impl<Group> SpatialIndex<Group> {
    pub const PIXELS_PER_CHUNK_DEFAULT: f32 = 100.;

    /// Creates a new spacial index.
    /// # Arguments
    /// * `pixels_per_chunk` - Spacial index is divided into chunks of size `pixels_per_chunk`.
    ///   Every hurtbox, Aabb2d of which is intersected with the chunk, is added to the chunk.
    ///   This is done to reduce the number of collision checks, only neccessary chunks are iterated.
    ///   Generally, this should match size of the colliders for best performance,
    ///   but this really depends on lots of factors.   
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpatialIndex {
            chunks: HashMap::default(),
            pixels_per_chunk,
            marker: std::marker::PhantomData,
        }
    }

    /// Coordinates of the chunk in which the `global` lies in.
    #[inline]
    pub fn global_to_chunk(&self, global: Vec2) -> IVec2 {
        global_to_chunk(self.pixels_per_chunk, global)
    }

    /// Bottom left corner of the chunk.
    #[inline]
    pub fn chunk_to_global(&self, chunk: IVec2) -> Vec2 {
        chunk_to_global(self.pixels_per_chunk, chunk)
    }

    /// The size of the chunk in pixels.
    #[inline]
    pub fn pixels_per_chunk(&self) -> f32 {
        self.pixels_per_chunk
    }

    /// Iterates over all chunks that intersect with the given `aabb`.
    pub fn iter_chunks_on_aabb(&self, aabb: Aabb2d) -> impl Iterator<Item = &Vec<Entity>> {
        let min = self.global_to_chunk(aabb.min);
        let max = self.global_to_chunk(aabb.max);

        (min.x..=max.x)
            .flat_map(move |x| (min.y..=max.y).map(move |y| (x, y)))
            .filter_map(|(x, y)| self.chunks.get(&IVec2::new(x, y)))
    }

    fn foreach_chunk_on_aabb_mut(&mut self, aabb: Aabb2d, mut f: impl FnMut(&mut Vec<Entity>)) {
        let min = self.global_to_chunk(aabb.min);
        let max = self.global_to_chunk(aabb.max);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let chunk = self.chunks.entry(IVec2::new(x, y)).or_default();
                f(chunk);
            }
        }
    }

    pub(super) fn add_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| chunk.push(entity));
    }

    pub(super) fn change_entity(&mut self, entity: Entity, old_aabb: Aabb2d, new_aabb: Aabb2d) {
        self.remove_entity(entity, old_aabb);
        self.add_entity(entity, new_aabb);
    }

    pub(super) fn remove_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| {
            if let Some(entity_index) = chunk.iter().position(|entry| *entry == entity) {
                chunk.swap_remove(entity_index);
            }
        });
    }
}
