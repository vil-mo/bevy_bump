use super::{spacial_index::SpacialIndexRegistry, LayerGroup};
use bevy::{ecs::world::DeferredWorld, prelude::*};
use std::marker::PhantomData;

#[derive(Reflect, Component, Copy, Clone, Default, PartialEq, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

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

/// Don't forget user can remove necessary components before applying commands
fn register_hurtbox<Layer: LayerGroup>(
    to_register: Query<
        Entity,
        (
            With<RegisterHurtbox<Layer>>,
            With<HurtboxLayer<Layer>>,
            With<HurtboxMonitorable<Layer>>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in to_register.iter() {
        commands
            .entity(entity)
            .insert(SpacialIndexRegistry::<Layer>::not_valid());
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
