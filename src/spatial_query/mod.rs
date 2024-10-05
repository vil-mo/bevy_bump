use super::collider::Collider;
use crate::collider::ColliderInteraction;
use bevy::{ecs::system::SystemParam, math::Dir2, prelude::Entity, utils::all_tuples};
use filter::SystemParamFilter;

pub mod filter;
pub mod response;
pub mod spatial_index;

/// BroadPhase returns iterators over
pub trait SpatialQuery: Copy {
    type HurtboxData;
    type Hurtbox: Send + Sync + 'static;
    type Filter: SystemParamFilter;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect<Hitbox: ColliderInteraction<Self::Hurtbox>>(
        self,
        hitbox: Collider<Hitbox>,
        filter_param: <Self::Filter as SystemParamFilter>::FilterParam,
    ) -> impl Iterator<Item = Self::HurtboxData>;

    /// Returns iterator over all the collisions that happened.
    /// f32 is distance in the direction of offset_dir. It is always less than offset_len.
    /// Dir2 is normal of the collision.
    fn cast<Hitbox: ColliderInteraction<Self::Hurtbox>>(
        self,
        hitbox: Collider<Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
        filter_param: <Self::Filter as SystemParamFilter>::FilterParam,
    ) -> impl Iterator<Item = (f32, Dir2, Self::HurtboxData)>;
}
