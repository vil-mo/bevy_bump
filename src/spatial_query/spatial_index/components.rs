use crate::core::ColliderGroup;

use super::{spatial_index::SpacialIndexRegistry, LayerGroup};
use bevy::prelude::*;
use std::marker::PhantomData;
/// Layer of the hitbox. Stores [`LayerGroup::Layer`].
/// If [CollisionLayer::collides](crate::ecs_core::layer::CollisionLayer::collides) returns `false`,
/// the collision with the hurtbox will be ignored.
#[derive(Component, Deref)]
pub struct HitboxLayer<Group: LayerGroup>(pub Group::Layer);

#[derive(Component, Deref)]
pub struct HurtboxLayer<Group: LayerGroup>(pub Group);

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterHurtbox<Group: ColliderGroup>(PhantomData<Group>);

impl<Group: ColliderGroup> RegisterHurtbox<Group> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

fn register_hurtbox<Group: ColliderGroup>(
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
            .add(|entity: Entity, world: &mut World| {
                // Until command is appled to the world, user can remove necessary components
                // So we need to check before inserting

                let mut entity_mut = world.entity_mut(entity);
                if !entity_mut.contains::<RegisterHurtbox<Group>>() {
                    return;
                }
                entity_mut.remove::<RegisterHurtbox<Group>>();

                if entity_mut.contains::<SpacialIndexRegistry<Group>>() {
                    return;
                }

                if entity_mut.contains::<HurtboxLayer<Group>>()
                    && entity_mut.contains::<HurtboxShape<Group>>()
                    && entity_mut.contains::<Transform>()
                {
                    entity_mut.insert(SpacialIndexRegistry::<Group>::not_valid());
                }
            });
    }
}
