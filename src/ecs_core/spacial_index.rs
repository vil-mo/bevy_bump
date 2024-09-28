use super::{components::HurtboxShape, LayerGroup};
use crate::utils::Bounded;
use bevy::math::bounding::Aabb2d;
use bevy::utils::HashMap;
use bevy::{
    ecs::entity::{EntityMapper, MapEntities},
    prelude::*,
};
use std::marker::PhantomData;

// TODO: Make private
pub fn register_index<Layer: LayerGroup>(app: &mut App, pixels_per_chunk: f32) {
    app.insert_resource(SpacialIndex::<Layer>::new(pixels_per_chunk));

    app.add_systems(PostUpdate, update_spacial_index_registry::<Layer>);
    app.observe(on_add_spacial_index_registry::<Layer>)
        .observe(on_remove_spacial_index_registry::<Layer>);
}

/// Entities with [`ColliderAabb`]s sorted along an axis by their extents.
#[derive(Resource)]
pub struct SpacialIndex<Layer: LayerGroup> {
    pub(crate) chunks: HashMap<IVec2, Vec<Entity>>,
    pub(crate) pixels_per_chunk: f32,
    pd: PhantomData<Layer>,
}

impl<Layer: LayerGroup> MapEntities for SpacialIndex<Layer> {
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

impl<Layer: LayerGroup> SpacialIndex<Layer> {
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpacialIndex {
            // TODO: fix placeholder numbers
            chunks: HashMap::default(),
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

        (min.x..=max.x)
            .flat_map(move |x| (min.y..=max.y).map(move |y| (x, y)))
            .filter_map(|(x, y)| self.chunks.get(&IVec2::new(x, y)))
    }

    pub(crate) fn foreach_chunk_on_aabb_mut(
        &mut self,
        aabb: Aabb2d,
        mut f: impl FnMut(&mut Vec<Entity>),
    ) {
        let min = self.global_to_chunk(aabb.min);
        let max = self.global_to_chunk(aabb.max);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                let chunk = self.chunks.entry(IVec2::new(x, y)).or_default();
                f(chunk);
            }
        }
    }

    pub(crate) fn add_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| chunk.push(entity));
    }

    pub(crate) fn change_entity(&mut self, entity: Entity, old_aabb: Aabb2d, new_aabb: Aabb2d) {
        self.remove_entity(entity, old_aabb);
        self.add_entity(entity, new_aabb);
    }

    pub(crate) fn remove_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| {
            if let Some(entity_index) = chunk.iter().position(|entry| *entry == entity) {
                chunk.swap_remove(entity_index);
            }
        });
    }
}

#[derive(Component, Debug, Clone)]
pub struct SpacialIndexRegistry<Layer: LayerGroup> {
    pub(crate) current_shape_bounding: Aabb2d,
    pub(crate) current_position: Vec2,
    last_shape_bounding: Aabb2d,
    last_position: Vec2,

    last_local_position: Vec2,
    marker: PhantomData<Layer>,
}

impl<Layer: LayerGroup> SpacialIndexRegistry<Layer> {
    pub fn not_valid() -> Self {
        Self {
            current_shape_bounding: Aabb2d::new(Vec2::NAN, Vec2::NAN),
            current_position: Vec2::NAN,
            last_shape_bounding: Aabb2d::new(Vec2::NAN, Vec2::NAN),
            last_position: Vec2::NAN,
            last_local_position: Vec2::NAN,
            marker: PhantomData,
        }
    }

    #[inline]
    pub fn aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.current_shape_bounding.min + self.current_position,
            max: self.current_shape_bounding.max + self.current_position,
        }
    }

    #[inline]
    fn last_aabb(&self) -> Aabb2d {
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

macro_rules! hurtbox_registering_error {
    ($x:expr) => {
        format!(
            "Trying to register entity as hurtbox{}. \
            Unable to deduce entity's position in SpacialIndex. \
            Use `RegisterHurtbox` component to register hurtbox as soon as possible, \
            instead of trying to do it manually.",
            $x
        )
    };
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
        " without `HurtboxShape` present"
    ));
    let transform = transform.expect(&hurtbox_registering_error!(" without `Transform` present"));

    let last_local_position = transform.translation.xy();
    let current_shape_bounding = shape.bounding();
    let current_position = transform_helper
        .compute_global_transform(entity)
        .expect(
            ", but hierarchy is malformed. \
            See `bevy::transform::helper::ComputeGlobalTransformError::MalformedHierarchy`",
        )
        .translation()
        .xy();

    *registry = SpacialIndexRegistry {
        current_shape_bounding,
        current_position,
        last_local_position,

        last_shape_bounding: current_shape_bounding,
        last_position: current_position,
        marker: PhantomData,
    };
    index.add_entity(entity, registry.aabb());
}

fn on_remove_spacial_index_registry<Layer: LayerGroup>(
    trigger: Trigger<OnRemove, SpacialIndexRegistry<Layer>>,
    mut index: ResMut<SpacialIndex<Layer>>,
    mut hurtboxes: Query<&SpacialIndexRegistry<Layer>>,
) {
    let entity = trigger.entity();

    let registry = hurtboxes.get_mut(entity).unwrap();

    index.remove_entity(entity, registry.aabb());
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

            spacial_index.change_entity(entity, registry.last_aabb(), registry.aabb());
        }
    }
}
