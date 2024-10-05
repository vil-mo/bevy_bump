use bevy::prelude::{Component, Deref};
use std::marker::PhantomData;

use crate::ColliderGroup;

use super::Filter;

pub struct Monitorable;

impl Filter for Monitorable {
    type FilterParam = HurtboxMonitorable<ColliderGroup>;
    fn filter(param: Self::FilterParam) -> bool {
        true
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
