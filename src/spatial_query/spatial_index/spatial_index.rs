use super::components::HurtboxShape;
use crate::core::ColliderGroup;
use crate::bounded::Bounded;
use bevy::math::bounding::Aabb2d;
use bevy::utils::HashMap;
use bevy::{
    ecs::entity::{EntityMapper, MapEntities},
    prelude::*,
};
use std::marker::PhantomData;

const PIXELS_PER_CHUNK_DEFAULT: f32 = 100.;

pub struct SpacialIndexPlugin<Group: ColliderGroup> {
    pub pixels_per_chunk: f32,

    marker: PhantomData<Group>,
}

impl<Group: ColliderGroup> Default for SpacialIndexPlugin<Group> {
    fn default() -> Self {
        Self::new(PIXELS_PER_CHUNK_DEFAULT)
    }
}

impl<Group: ColliderGroup> SpacialIndexPlugin<Group> {
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpacialIndexPlugin {
            pixels_per_chunk,
            marker: PhantomData,
        }
    }
}

impl<Group: ColliderGroup> Plugin for SpacialIndexPlugin<Group> {
    fn build(&self, app: &mut App) {
        app.add_resource(SpacialIndex::<Group>::new(self.pixels_per_chunk));
    }
}

#[derive(Resource)]
pub struct SpacialIndex<Group: ColliderGroup> {
    chunks: HashMap<IVec2, Vec<Entity>>,
    pixels_per_chunk: f32,
    marker: PhantomData<Group>,
}

impl<Group: ColliderGroup> Default for SpacialIndex<Group> {
    fn default() -> Self {
        Self::new(PIXELS_PER_CHUNK_DEFAULT)
    }
}

impl<Group: ColliderGroup> MapEntities for SpacialIndex<Group> {
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

impl<Group: ColliderGroup> SpacialIndex<Group> {
    /// Creates a new spacial index.
    /// # Arguments
    /// * `pixels_per_chunk` - Spacial index is divided into chunks of size `pixels_per_chunk`.
    ///   Every hurtbox, Aabb2d of which is intersected with the chunk, is added to the chunk.
    ///   This is done to reduce the number of collision checks, only neccessary chunks are iterated.
    ///   Generally, this should match size of the colliders for best performance,
    ///   but this really depends on lots of factors.   
    pub fn new(pixels_per_chunk: f32) -> Self {
        SpacialIndex {
            chunks: HashMap::default(),
            pixels_per_chunk,
            marker: PhantomData,
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

    /// Returns the size of the chunk in pixels.
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

    fn foreach_chunk_on_aabb_mut(
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

    fn add_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| chunk.push(entity));
    }

    fn change_entity(&mut self, entity: Entity, old_aabb: Aabb2d, new_aabb: Aabb2d) {
        self.remove_entity(entity, old_aabb);
        self.add_entity(entity, new_aabb);
    }

    fn remove_entity(&mut self, entity: Entity, aabb: Aabb2d) {
        self.foreach_chunk_on_aabb_mut(aabb, |chunk| {
            if let Some(entity_index) = chunk.iter().position(|entry| *entry == entity) {
                chunk.swap_remove(entity_index);
            }
        });
    }
}

#[derive(Component, Debug, Clone)]
pub struct SpacialIndexRegistry<Group: ColliderGroup> {
    current_shape_bounding: Aabb2d,
    current_position: Vec2,
    last_shape_bounding: Aabb2d,
    last_position: Vec2,

    marker: PhantomData<Group>,
}

impl<Group: ColliderGroup> SpacialIndexRegistry<Group> {
    /// Creates a new instance with invalid fields.
    /// The
    pub fn not_valid() -> Self {
        Self {
            current_shape_bounding: Aabb2d::new(Vec2::NAN, Vec2::NAN),
            current_position: Vec2::NAN,
            last_shape_bounding: Aabb2d::new(Vec2::NAN, Vec2::NAN),
            last_position: Vec2::NAN,
            marker: PhantomData,
        }
    }

    fn global_last_aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.last_shape_bounding.min + self.last_position,
            max: self.last_shape_bounding.max + self.last_position,
        }
    }

    #[inline]
    pub fn global_aabb(&self) -> Aabb2d {
        Aabb2d {
            min: self.current_shape_bounding.min + self.current_position,
            max: self.current_shape_bounding.max + self.current_position,
        }
    }

    #[inline]
    pub fn current_shape_bounding(&self) -> Aabb2d {
        self.current_shape_bounding
    }

    #[inline]
    pub fn current_position(&self) -> Vec2 {
        self.current_position
    }

    fn update(&mut self, hurtbox: &HurtboxShape<Group>, new_position: Vec2) {
        let new_shape_bounding = hurtbox.bounding();

        self.last_position = self.current_position;
        self.last_shape_bounding = self.current_shape_bounding;

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

fn on_add_spacial_index_registry<Group: ColliderGroup>(
    trigger: Trigger<OnAdd, SpacialIndexRegistry<Group>>,
    mut index: ResMut<SpacialIndex<Group>>,
    mut hurtboxes: Query<(
        &mut SpacialIndexRegistry<Group>,
        Option<&HurtboxShape<Group>>,
    )>,
    transform_helper: TransformHelper,
) {
    let entity = trigger.entity();

    let (mut registry, shape) = hurtboxes.get_mut(entity).unwrap();
    let shape = shape.expect(&hurtbox_registering_error!(
        " without `HurtboxShape` present"
    ));

    let current_shape_bounding = shape.bounding();
    let current_position = transform_helper
        .compute_global_transform(entity)
        .expect(", but there is a problem computing global position")
        .translation()
        .xy();

    *registry = SpacialIndexRegistry {
        current_shape_bounding,
        current_position,
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
    )>,
    mut spacial_index: ResMut<SpacialIndex<Layer>>,
    transform_helper: TransformHelper,
) {
    for (entity, mut registry, hurtbox) in hurtboxes.iter_mut() {
        let Ok(new_position) = transform_helper.compute_global_transform(entity) else {
            warn!("Unable to compute global position of registered hurtbox of {entity}. Skipping hurtbox update.");
            continue;
        };
        let new_position = new_position.translation().xy();

        let position_change = registry.current_position - new_position;
        if hurtbox.is_changed() || position_change != Vec2::ZERO {
            registry.update(&hurtbox, new_position);
            spacial_index.change_entity(entity, registry.last_aabb(), registry.aabb());
        }
    }
}
