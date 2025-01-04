use crate::collider::{Collider, ColliderInteraction};
use bevy::math::Dir2;
use filter::SpatialQueryFilter;

pub mod filter;
pub mod spatial_index;

pub trait SpatialQuery {
    type HurtboxData;
    type Hurtbox: Send + Sync + 'static;
    type Filter: SpatialQueryFilter;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect<Hitbox: ColliderInteraction<Self::Hurtbox>>(
        &mut self,
        hitbox: Collider<Hitbox>,
        filter_param: <Self::Filter as SpatialQueryFilter>::HitboxFilterParam<'_>,
    ) -> impl Iterator<Item = Self::HurtboxData>;

    /// Returns iterator over all the collisions that happened.
    /// f32 is distance in the direction of offset_dir. It is always less than offset_len.
    /// Dir2 is normal of the collision.
    fn cast<Hitbox: ColliderInteraction<Self::Hurtbox>>(
        &mut self,
        hitbox: Collider<Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
        filter_param: <Self::Filter as SpatialQueryFilter>::HurtboxFilterParam<'_>,
    ) -> impl Iterator<Item = (f32, Dir2, Self::HurtboxData)>;
}
