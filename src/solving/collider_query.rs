use super::collider::{Collider, ColliderCast};
use crate::solving::layer::CollisionLayer;
use bevy::prelude::*;

pub trait ColliderQuery<'a, TO, L>
where
    TO: Collider + 'a,
    L: CollisionLayer,
{
    fn all(&self) -> impl Iterator<Item = (&'a TO, L)>;
}
/// SolidBodyQuery's provide data about colliders of solids
pub trait ColliderQueryCast<'a, TO, FROM, L>: ColliderQuery<'a, TO, L>
where
    TO: Collider + 'a,
    FROM: ColliderCast<TO>,
    L: CollisionLayer,
{
    fn cast(&self, collider: &FROM, offset: Vec2) -> impl Iterator<Item = (&'a TO, L)>;
}

pub trait ColliderQueryIntersect<'a, TO, FROM, L>: ColliderQuery<'a, TO, L>
where
    TO: Collider + 'a,
    FROM: ColliderCast<TO>,
    L: CollisionLayer,
{
    fn intersect(&self, collider: &FROM, offset: Vec2) -> impl Iterator<Item = (&'a TO, L)>;
}
