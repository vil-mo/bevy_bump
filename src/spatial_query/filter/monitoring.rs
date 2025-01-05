use super::{SpatialQueryFilter, SystemSpatialQueryFilter};
use crate::ColliderGroup;
use bevy::{ecs::system::SystemParam, prelude::*};
use std::marker::PhantomData;

pub struct Monitoring;

impl SpatialQueryFilter for Monitoring {
    type HitboxParam<'a> = bool;
    type HurtboxParam<'a> = ();

    #[inline]
    fn filter(hitbox_data: bool, _hurtbox_data: ()) -> bool {
        hitbox_data
    }
}

impl<Group: ColliderGroup> SystemSpatialQueryFilter<Group> for Monitoring {
    type HitboxSystemParam = Query<'static, 'static, &'static HitboxMonitoring<Group>>;
    type HurtboxSystemParam = ();

    fn hitbox_filter_param(
        hitbox: Entity,
        system_param: &mut <Self::HitboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> bool {
        system_param.get(hitbox).copied().unwrap_or_default().0
    }

    fn hurtbox_filter_param(_hurtbox: Entity, _system_param: &mut ()) -> () {}
}

#[derive(Component, Deref)]
pub struct HitboxMonitoring<Group: ColliderGroup>(#[deref] pub bool, PhantomData<Group>);

impl<Group: ColliderGroup> Default for HitboxMonitoring<Group> {
    #[inline]
    fn default() -> Self {
        Self::new(true)
    }
}

impl<Group: ColliderGroup> Clone for HitboxMonitoring<Group> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Group: ColliderGroup> Copy for HitboxMonitoring<Group> {}

impl<Group: ColliderGroup> HitboxMonitoring<Group> {
    #[inline]
    pub fn new(monitoring: bool) -> Self {
        Self(monitoring, PhantomData)
    }
}
