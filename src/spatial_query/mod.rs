use crate::{collider::Collider, ColliderGroup};
use bevy::math::Dir2;
use filter::SpatialQueryFilter;

pub mod filter;

pub trait SpatialQuery<Group: ColliderGroup> {
    type HurtboxData;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect<'a>(
        &'a mut self,
        hitbox: Collider<'a, Group::Hitbox>,
        hitbox_filter: <Group::Filter as SpatialQueryFilter>::HitboxParam<'a>,
    ) -> impl Iterator<Item = Self::HurtboxData> + 'a;

    /// Returns iterator over all the collisions that happened.
    /// f32 is distance in the direction of offset_dir. It is always less than offset_len.
    /// Dir2 is normal of the collision.
    fn cast<'a>(
        &'a mut self,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
        hitbox_filter: <Group::Filter as SpatialQueryFilter>::HitboxParam<'a>,
    ) -> impl Iterator<Item = (f32, Dir2, Self::HurtboxData)> + 'a;
}

