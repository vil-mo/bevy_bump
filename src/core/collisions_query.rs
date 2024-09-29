use super::{collider::Collider, ColliderGroup};
use bevy::math::Dir2;

/// BroadPhase returns iterators over
pub trait CollisionsQuery<Group: ColliderGroup>: Copy {
    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect(self, hitbox: Collider<Group::Hitbox>)
        -> impl Iterator<Item = Group::HurtboxData>;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn cast(
        self,
        hitbox: Collider<Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl Iterator<Item = (f32, Dir2, Group::HurtboxData)>;
}
