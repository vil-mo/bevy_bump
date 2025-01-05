use super::{spatial_index::SpatialIndex, SpatialIndexColliderGroup};
use crate::{bounded::Bounded, components::HurtboxShape};
use bevy::{math::bounding::Aabb2d, prelude::*};
use std::marker::PhantomData;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterHurtbox<T>(PhantomData<T>);

impl<T> RegisterHurtbox<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

pub(super) fn register_hurtbox<Group: SpatialIndexColliderGroup>(
    to_register: Query<
        Entity,
        (
            With<RegisterHurtbox<Group>>,
            With<HurtboxShape<Group>>,
            With<Transform>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in to_register.iter() {
        commands
            .entity(entity)
            .queue(|entity: Entity, world: &mut World| {
                // Until command is appled to the world, user can remove necessary components
                // So we need to check before inserting
                let mut entity_mut = world.entity_mut(entity);

                if !entity_mut.contains::<HurtboxShape<Group>>()
                    || !entity_mut.contains::<Transform>()
                {
                    return;
                }

                if entity_mut.take::<RegisterHurtbox<Group>>().is_none() {
                    return;
                }

                entity_mut.insert(SpatialIndexRegistry::<Group>::not_valid());
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

impl<Group: SpatialIndexColliderGroup> SpatialIndexRegistry<Group> {
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

fn on_insert_spacial_index_registry<Group: SpatialIndexColliderGroup>(
    trigger: Trigger<OnInsert, SpatialIndexRegistry<Group>>,
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
    index.add_entity(entity, registry.global_aabb());
}

fn on_replace_spacial_index_registry<Group: SpatialIndexColliderGroup>(
    trigger: Trigger<OnReplace, (SpatialIndexRegistry<Group>, HurtboxShape<Group>, Transform)>,
    mut index: ResMut<SpatialIndex<Group>>,
    mut hurtboxes: Query<&SpatialIndexRegistry<Group>>,
) {
    let entity = trigger.entity();
    let Ok(registry) = hurtboxes.get_mut(entity) else {
        return;
    };
    index.remove_entity(entity, registry.global_aabb());
}

fn update_spatial_index_registry<Group: SpatialIndexColliderGroup>(
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
            spacial_index.change_entity(entity, registry.global_last_aabb(), registry.global_aabb());
        }
    }
}
