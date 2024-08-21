use crate::ecs_core::LayerGroup;
use bevy::prelude::*;
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

#[derive(Component, Deref)]
pub struct HurtboxShape<Layer: LayerGroup>(pub Layer::Hurtbox);

#[derive(Component, Deref)]
pub struct HurtboxLayer<Layer: LayerGroup>(pub Layer);

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
pub struct RegisterHurtbox;

