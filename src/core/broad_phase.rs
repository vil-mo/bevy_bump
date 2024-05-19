use super::collider::{Collider, ColliderInformation, ColliderInteraction, CollisionGroup};
use crate::core::layer::CollisionLayer;
use crate::core::response::CollisionInformation;
use bevy::prelude::*;

pub trait BroadPhase<'a, Group, L>
where
    Group: CollisionGroup,
    L: CollisionLayer,
{
    fn cast(
        &self,
        actor: &Group::Actor,
        offset: Vec2,
    ) -> impl Iterator<Item = ColliderInformation<'a, Group::Target, L>>;

    fn intersect(
        &self,
        actor: &Group::Actor,
    ) -> impl Iterator<Item = ColliderInformation<'a, Group::Target, L>>;
}

trait CollidersIterator<'a, Group, L>:
    Iterator<Item = ColliderInformation<'a, Group::Target, L>>
where
    Group: CollisionGroup,
    L: CollisionLayer,
{
    fn collisions(
        self,
        actor: &Group::Actor,
        desired_offset: Vec2,
    ) -> impl Iterator<Item = CollisionInformation>;
}

impl<'a, Group, L, T> CollidersIterator<'a, Group, L> for T
where
    Group: CollisionGroup,
    L: CollisionLayer,
    T: Iterator<Item = ColliderInformation<'a, Group::Target, L>>,
{
    fn collisions(
        self,
        actor: &Group::Actor,
        desired_offset: Vec2,
    ) -> impl Iterator<Item = CollisionInformation> {
        let offset_normal = desired_offset.normalize();

        self.filter_map(move |collider| {
            actor.cast(collider.collider, desired_offset).map(|dist| {
                let center = collider.collider.position();

                CollisionInformation {
                    point: center + offset_normal * dist,
                    normal: collider.collider.normal(center),
                }
            })
        })
    }
}
