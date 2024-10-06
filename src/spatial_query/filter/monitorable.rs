use super::{SpatialQueryFilter, SystemSpatialQueryFilter};
use crate::ColliderGroup;
use bevy::{ecs::system::SystemParam, prelude::*};
use std::marker::PhantomData;

pub struct Monitorable;

impl SpatialQueryFilter for Monitorable {
    type HitboxFilterParam<'a> = ();
    type HurtboxFilterParam<'a> = bool;

    #[inline]
    fn filter(_hitbox_data: (), hurtbox_data: bool) -> bool {
        hurtbox_data
    }
}

impl<Group: ColliderGroup> SystemSpatialQueryFilter<Group> for Monitorable {
    type HitboxSystemParam = ();
    type HitboxQueryFilter = ();

    type HurtboxSystemParam = Query<'static, 'static, &'static HurtboxMonitorable<Group>>;
    type HurtboxQueryFilter = ();

    fn hitbox_filter_param<'a>(_hitbox: Entity, _system_param: &mut ()) -> () {}

    fn hurtbox_filter_param(
        hurtbox: Entity,
        system_param: &mut <Self::HurtboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> bool {
        system_param.get(hurtbox).copied().unwrap_or_default().0
    }
}

#[derive(Component, Deref)]
pub struct HurtboxMonitorable<Group: ColliderGroup>(#[deref] pub bool, PhantomData<Group>);

impl<Group: ColliderGroup> Default for HurtboxMonitorable<Group> {
    #[inline]
    fn default() -> Self {
        Self::new(true)
    }
}

impl<Group: ColliderGroup> Clone for HurtboxMonitorable<Group> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Group: ColliderGroup> Copy for HurtboxMonitorable<Group> {}

impl<Group: ColliderGroup> HurtboxMonitorable<Group> {
    #[inline]
    pub fn new(monitorable: bool) -> Self {
        Self(monitorable, PhantomData)
    }
}
