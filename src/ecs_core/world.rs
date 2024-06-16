use crate::core::collider::Collider;
use crate::ecs_core::bodies::{Hitbox, Hurtbox, HurtboxAabb, Velocity};
use crate::ecs_core::LayerGroup;
use crate::utils::Bounded;
use bevy::utils::AHasher;
use bevy::{
    ecs::entity::{EntityMapper, MapEntities},
    prelude::*,
};
use plane_2d::Plane;
use std::hash::BuildHasherDefault;
use std::marker::PhantomData;

pub(crate) fn register_world<Layer: LayerGroup>(
    app: &mut App,
    pixels_per_chunk: f32,
    crowded_x: Vec2,
    crowded_y: Vec2,
) {
    app.insert_resource(CollisionWorld::<Layer>::new(
        pixels_per_chunk,
        crowded_x,
        crowded_y,
    ));

    app.add_systems(PostUpdate, update_world_entries::<Layer>);
}

#[derive(Default)]
pub(crate) struct Chunk {
    pub(crate) entries: Vec<Entity>,
}

/// Entities with [`ColliderAabb`]s sorted along an axis by their extents.
#[derive(Resource)]
pub struct CollisionWorld<Layer: LayerGroup> {
    pub(crate) chunks: Plane<Chunk, BuildHasherDefault<AHasher>>,
    pub(crate) pixels_per_chunk: f32,
    pd: PhantomData<Layer>,
}

impl<Layer: LayerGroup> MapEntities for CollisionWorld<Layer> {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for (_, chunk) in self.chunks.iter_all_mut() {
            for entity in chunk.entries.iter_mut() {
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

impl<Layer: LayerGroup> CollisionWorld<Layer> {
    pub fn new(pixels_per_chunk: f32, crowded_min: Vec2, crowded_max: Vec2) -> Self {
        let min = global_to_chunk(pixels_per_chunk, crowded_min);
        let max = global_to_chunk(pixels_per_chunk, crowded_max);

        CollisionWorld {
            chunks: Plane::default_hasher(min.x, min.y, max.x, max.y),
            pixels_per_chunk,
            pd: PhantomData,
        }
    }

    /// Coordinates of the chunk in which the `global` lies in
    #[inline]
    pub fn global_to_chunk(&self, global: Vec2) -> IVec2 {
        global_to_chunk(self.pixels_per_chunk, global)
    }

    /// Bottom left corner of the chunk
    #[inline]
    fn chunk_to_global(&self, chunk: IVec2) -> Vec2 {
        chunk_to_global(self.pixels_per_chunk, chunk)
    }
}

/// Updates [`CollisionWorld`] to keep them in sync with the [`Hurtbox`]es and [`Transform`].
fn update_world_entries<Layer: LayerGroup>(
    mut commands: Commands,

    hurtboxes_aabb: Query<&HurtboxAabb<Layer>>,
    mut removed_hurtboxes: RemovedComponents<Hurtbox<Layer>>,
    changed_hurtboxes: Query<(Entity, &HurtboxAabb<Layer>), Changed<Hurtbox<Layer>>>,
    added_hurtboxes: Query<(Entity, &Hurtbox<Layer>, &GlobalTransform), Added<Hurtbox<Layer>>>,

    mut world: ResMut<CollisionWorld<Layer>>,
) {
    // removed
    for entity in removed_hurtboxes.read() {
        use iter_n::iter2::*;
        let chunks = if let Ok(hurtbox_aabb) = hurtboxes_aabb.get(entity) {
            let min = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.last_aabb.min);
            let max = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.last_aabb.max);
            world
                .chunks
                .iter_rect_mut(min.x, min.y, max.x, max.y)
                .into_iter0()
        } else {
            world.chunks.iter_all_mut().into_iter1()
        };

        for (_, chunk) in chunks {
            if let Some(entity_index) = chunk.entries.iter().position(|entry| *entry == entity) {
                chunk.entries.swap_remove(entity_index);
            }
        }
    }

    // changed
    for (entity, hurtbox_aabb) in changed_hurtboxes.iter() {
        let min = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.last_aabb.min);
        let max = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.last_aabb.max);
        for (_, chunk) in world.chunks.iter_rect_mut(min.x, min.y, max.x, max.y) {
            if let Some(entity_index) = chunk.entries.iter().position(|entry| *entry == entity) {
                chunk.entries.swap_remove(entity_index);
            }
        }

        let min = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.current_aabb.min);
        let max = global_to_chunk(world.pixels_per_chunk, hurtbox_aabb.current_aabb.max);

        for (_, chunk) in world.chunks.iter_rect_mut(min.x, min.y, max.x, max.y) {
            chunk.entries.push(entity);
        }
    }

    // added
    for (entity, hurtbox, global_transform) in added_hurtboxes.iter() {
        let mut collider = hurtbox.collider.clone();
        collider.set_position(collider.position() + global_transform.translation().xy());
        let aabb = collider.bounding();

        let min = global_to_chunk(world.pixels_per_chunk, aabb.min);
        let max = global_to_chunk(world.pixels_per_chunk, aabb.max);

        for (_, chunk) in world.chunks.iter_rect_mut(min.x, min.y, max.x, max.y) {
            chunk.entries.push(entity);
        }

        commands.entity(entity).insert(HurtboxAabb::<Layer> {
            last_aabb: aabb,
            current_aabb: aabb,
            pd: PhantomData,
        });
    }
}
