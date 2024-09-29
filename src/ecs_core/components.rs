use super::{spacial_index::SpacialIndexRegistry, LayerGroup};
use bevy::{
    ecs::world::DeferredWorld,
    prelude::*,
};
use std::marker::PhantomData;


#[derive(Component, Deref)]
pub struct HitboxShape<Layer: LayerGroup>(pub Layer::Hitbox);
#[derive(Component, Deref)]
pub struct HitboxLayer<Layer: LayerGroup>(pub Layer);
#[derive(Component, Deref)]
pub struct HitboxMonitoring<Layer: LayerGroup>(#[deref] pub bool, PhantomData<Layer>);

impl<Layer: LayerGroup> Default for HitboxMonitoring<Layer> {
    #[inline]
    fn default() -> Self {
        Self::new(true)
    }
}

impl<Layer: LayerGroup> HitboxMonitoring<Layer> {
    #[inline]
    pub fn new(monitoring: bool) -> Self {
        Self(monitoring, PhantomData)
    }
}

impl<Layer: LayerGroup> Clone for HitboxMonitoring<Layer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Layer: LayerGroup> Copy for HitboxMonitoring<Layer> {}

#[derive(Deref)]
pub struct HurtboxShape<Layer: LayerGroup>(pub Layer::Hurtbox);

impl<Layer: LayerGroup> Component for HurtboxShape<Layer> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(on_remove_hurtbox::<Layer>);
    }
}

#[derive(Deref)]
pub struct HurtboxLayer<Layer: LayerGroup>(pub Layer);

impl<Layer: LayerGroup> Component for HurtboxLayer<Layer> {
    const STORAGE_TYPE: bevy::ecs::component::StorageType =
        bevy::ecs::component::StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_remove(on_remove_hurtbox::<Layer>);
    }
}

#[derive(Component, Deref)]
pub struct HurtboxMonitorable<Layer: LayerGroup>(#[deref] pub bool, PhantomData<Layer>);

impl<Layer: LayerGroup> Default for HurtboxMonitorable<Layer> {
    #[inline]
    fn default() -> Self {
        Self::new(true)
    }
}

impl<Layer: LayerGroup> Clone for HurtboxMonitorable<Layer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Layer: LayerGroup> Copy for HurtboxMonitorable<Layer> {}

impl<Layer: LayerGroup> HurtboxMonitorable<Layer> {
    #[inline]
    pub fn new(monitorable: bool) -> Self {
        Self(monitorable, PhantomData)
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterHurtbox<Layer: LayerGroup>(PhantomData<Layer>);

impl<Layer: LayerGroup> RegisterHurtbox<Layer> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

fn register_hurtbox<Layer: LayerGroup>(
    to_register: Query<
        Entity,
        (
            With<RegisterHurtbox<Layer>>,
            With<HurtboxLayer<Layer>>,
            With<HurtboxShape<Layer>>,
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
                if !entity_mut.contains::<RegisterHurtbox<Layer>>() {
                    return;
                }
                entity_mut.remove::<RegisterHurtbox<Layer>>();

                if entity_mut.contains::<SpacialIndexRegistry<Layer>>() {
                    return;
                }

                if entity_mut.contains::<HurtboxLayer<Layer>>()
                    && entity_mut.contains::<HurtboxShape<Layer>>()
                    && entity_mut.contains::<Transform>()
                {
                    entity_mut.insert(SpacialIndexRegistry::<Layer>::not_valid());
                }
            });
    }
}

fn on_remove_hurtbox<Layer: LayerGroup>(
    mut world: DeferredWorld,
    entity: Entity,
    _id: bevy::ecs::component::ComponentId,
) {
    world
        .commands()
        .entity(entity)
        .remove::<SpacialIndexRegistry<Layer>>();
}
