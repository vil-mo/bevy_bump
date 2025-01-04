use crate::{components::HurtboxShape, ColliderGroup};
use bevy::{math::bounding::Aabb2d, prelude::*};
use std::marker::PhantomData;

use super::spatial_index::SpatialIndex;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterHurtbox<T: ColliderGroup>(PhantomData<T>);

impl<T: ColliderGroup> RegisterHurtbox<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub(super) fn register_hurtbox<T: ColliderGroup>(
    to_register: Query<
        Entity,
        (
            With<RegisterHurtbox<T>>,
            With<HurtboxShape<T>>,
            With<Transform>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in to_register.iter() {
        commands
            .entity(entity)
            .add(|entity: Entity, world: &mut World| {
                // Until command is appled to the world, user can remove necessary components
                // So we need to check before inserting

                let mut entity_mut = world.entity_mut(entity);
                if !entity_mut.contains::<RegisterHurtbox<T>>() {
                    return;
                }
                entity_mut.remove::<RegisterHurtbox<T>>();

                if entity_mut.contains::<SpatialIndexRegistry<T>>() {
                    return;
                }

                if entity_mut.contains::<HurtboxShape<T>>() && entity_mut.contains::<Transform>() {
                    entity_mut.insert(SpatialIndexRegistry::<T>::not_valid());
                }
            });
    }
}

#[derive(Component, Debug, Clone)]
pub struct SpatialIndexRegistry<Group> {
    current_shape_bounding: Aabb2d,
    current_position: Vec2,
    last_shape_bounding: Aabb2d,
    last_position: Vec2,

    marker: PhantomData<Group>,
}

impl<Group: ColliderGroup> SpatialIndexRegistry<Group> {
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
    trigger: Trigger<OnAdd, SpatialIndexRegistry<Group>>,
    mut index: ResMut<SpatialIndex<Group>>,
    mut hurtboxes: Query<(
        &mut SpatialIndexRegistry<Group>,
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
        .expect(&hurtbox_registering_error!(
            ", but there is a problem computing global position"
        ))
        .translation()
        .xy();

    *registry = SpatialIndexRegistry {
        current_shape_bounding,
        current_position,
        last_shape_bounding: current_shape_bounding,
        last_position: current_position,
        marker: PhantomData,
    };
    index.add_entity(entity, registry.aabb());
}

fn on_remove_spacial_index_registry<Group: ColliderGroup>(
    trigger: Trigger<OnRemove, SpatialIndexRegistry<Group>>,
    mut index: ResMut<SpatialIndex<Group>>,
    mut hurtboxes: Query<&SpatialIndexRegistry<Group>>,
) {
    let entity = trigger.entity();

    let registry = hurtboxes.get_mut(entity).unwrap();

    index.remove_entity(entity, registry.aabb());
}

fn update_spacial_index_registry<Group: ColliderGroup>(
    mut hurtboxes: Query<(
        Entity,
        &mut SpatialIndexRegistry<Group>,
        Ref<HurtboxShape<Group>>,
    )>,
    mut spacial_index: ResMut<SpatialIndex<Group>>,
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
