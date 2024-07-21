use crate::ecs_core::components::HurtboxShape;
use crate::ecs_core::LayerGroup;
use crate::utils::Bounded;
use bevy::ecs::reflect::ReflectMapEntitiesResource;
use bevy::math::bounding::Aabb2d;
use bevy::{
    ecs::entity::{EntityMapper, MapEntities},
    prelude::*,
};
use plane_2d::Plane;
use std::marker::PhantomData;

pub(crate) fn register_world<Layer: LayerGroup>(app: &mut App, pixels_per_chunk: f32) {
    app.insert_resource(SpacialIndex::<Layer>::new(pixels_per_chunk));

    // app.add_systems(PostUpdate, update_world_entries::<Layer>);
}

/// Entities with [`ColliderAabb`]s sorted along an axis by their extents.
#[derive(Resource, Reflect)]
#[reflect(Resource, MapEntitiesResource)]
pub struct SpacialIndex<Layer: LayerGroup> {
    pub(crate) chunks: Plane<Vec<Entity>>,
    pub(crate) pixels_per_chunk: f32,
    pd: PhantomData<Layer>,
}

impl<Layer: LayerGroup> MapEntities for SpacialIndex<Layer> {
    fn map_entities<M: EntityMapper>(&mut self, entity_mapper: &mut M) {
        for (_, chunk) in self.chunks.iter_all_mut() {
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

impl<Layer: LayerGroup> SpacialIndex<Layer> {
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpacialIndex {
            // TODO: fix placeholder numbers
            chunks: Plane::new(0, 0, 10, 10),
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
    pub fn chunk_to_global(&self, chunk: IVec2) -> Vec2 {
        chunk_to_global(self.pixels_per_chunk, chunk)
    }

    pub(crate) fn iter_chunks_on_aabb(&self, aabb: Aabb2d) -> impl Iterator<Item = &Vec<Entity>> {
        let min = self.global_to_chunk(aabb.min);
        let max = self.global_to_chunk(aabb.max);

        self.chunks
            .iter_rect(min.x, min.y, max.x, max.y)
            .map(|(_, chunk)| chunk)
    }

    pub(crate) fn iter_chunks_on_aabb_mut(
        &mut self,
        aabb: Aabb2d,
    ) -> impl Iterator<Item = &mut Vec<Entity>> {
        let min = self.global_to_chunk(aabb.min);
        let max = self.global_to_chunk(aabb.max);

        self.chunks
            .iter_rect_mut(min.x, min.y, max.x, max.y)
            .map(|(_, chunk)| chunk)
    }

    pub(crate) fn add_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        for chunk in self.iter_chunks_on_aabb_mut(aabb) {
            chunk.push(entity);
        }
    }

    pub(crate) fn change_entity(&mut self, entity: Entity, old_aabb: Aabb2d, new_aabb: Aabb2d) {
        self.remove_entity(entity, old_aabb);
        self.add_entity(entity, new_aabb);
    }

    pub(crate) fn remove_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        for chunk in self.iter_chunks_on_aabb_mut(aabb) {
            if let Some(entity_index) = chunk.iter().position(|entry| *entry == entity) {
                chunk.swap_remove(entity_index);
            }
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct SpacialIndexRegistry<Layer: LayerGroup> {
    pub current_shape_bounding: Aabb2d,
    pub current_position: Vec2,
    last_shape_bounding: Aabb2d,
    last_position: Vec2,

    last_local_position: Vec2,
    _pd: PhantomData<Layer>,
}

macro_rules! hurtbox_registering_error {
    ($x:expr) => {
            format!("Trying to register entity as hurtbox {}. Unable to deduce entity's position in SpacialIndex.", $x)
    }
}

fn on_add_spacial_index_registry<Layer: LayerGroup>(
    trigger: Trigger<OnAdd, SpacialIndexRegistry<Layer>>,
    mut index: ResMut<SpacialIndex<Layer>>,
    mut hurtboxes: Query<(
        &mut SpacialIndexRegistry<Layer>,
        Option<&HurtboxShape<Layer>>,
        Option<&Transform>,
    )>,
    transform_helper: TransformHelper,
) {
    let entity = trigger.entity();

    let (mut registry, shape, transform) = hurtboxes.get_mut(entity).unwrap();
    let shape = shape.expect(&hurtbox_registering_error!(
        "without `HurtboxShape` present"
    ));
    let transform = transform.expect(&hurtbox_registering_error!("without `Transform` present"));

    registry.last_local_position = transform.translation.xy();
    registry.current_shape_bounding = shape.bounding();
    registry.current_position = transform_helper
        .compute_global_transform(entity)
        .expect("without `Transform` present on one of the parents of entity - unable to calculate global position")
        .translation().xy();

    index.add_entity(entity, registry.current_aabb());
}

fn on_remove_spacial_index_registry<Layer: LayerGroup>(
    trigger: Trigger<OnRemove, SpacialIndexRegistry<Layer>>,
    mut index: ResMut<SpacialIndex<Layer>>,
    mut hurtboxes: Query<&SpacialIndexRegistry<Layer>>,
) {
    let entity = trigger.entity();

    let registry = hurtboxes.get_mut(entity).unwrap();

    index.remove_entity(entity, registry.current_aabb());
}

impl<Layer: LayerGroup> SpacialIndexRegistry<Layer> {
    #[inline]
    pub fn current_aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.current_shape_bounding.min + self.current_position,
            max: self.current_shape_bounding.max + self.current_position,
        }
    }

    #[inline]
    pub fn last_aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.last_shape_bounding.min + self.last_position,
            max: self.last_shape_bounding.max + self.last_position,
        }
    }

    fn update(&mut self, hurtbox: &HurtboxShape<Layer>, transform: &Transform) {
        let current_local_position = transform.translation.xy();
        let position_change = current_local_position - self.last_local_position;

        let new_position = self.current_position + position_change;
        let new_shape_bounding = hurtbox.bounding();

        self.last_position = self.current_position;
        self.last_shape_bounding = self.current_shape_bounding;
        self.last_local_position = current_local_position;

        self.current_position = new_position;
        self.current_shape_bounding = new_shape_bounding;
    }
}

fn update_spacial_index_registry<Layer: LayerGroup>(
    mut hurtboxes: Query<(
        Entity,
        &mut SpacialIndexRegistry<Layer>,
        Ref<HurtboxShape<Layer>>,
        &Transform,
    )>,
    mut spacial_index: ResMut<SpacialIndex<Layer>>,
) {
    for (entity, mut registry, hurtbox, transform) in hurtboxes.iter_mut() {
        let current_local_position = transform.translation.xy();
        let position_change = current_local_position - registry.last_local_position;

        if hurtbox.is_changed() || position_change != Vec2::ZERO {
            registry.update(&hurtbox, transform);

            spacial_index.change_entity(entity, registry.last_aabb(), registry.current_aabb());
        }
    }
}
