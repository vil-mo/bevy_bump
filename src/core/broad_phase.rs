use super::ColliderGroup;
use crate::core::collider::Collider;
use bevy::prelude::*;

pub type BroadPhaseIteratorOutput<'a, Group> = (Collider<'a, <Group as ColliderGroup>::Hurtbox>, <Group as ColliderGroup>::CollisionData);

/// BroadPhase returns iterators over
pub trait BroadPhase<'o, Group: ColliderGroup> {
    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect(
        &self,
        hitbox: Collider<Group::Hitbox>,
    ) -> impl Iterator<Item = BroadPhaseIteratorOutput<'o, Group>>;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn cast(
        &self,
        hitbox: Collider<Group::Hitbox>,
        offset: Vec2,
    ) -> impl Iterator<Item = BroadPhaseIteratorOutput<'o, Group>>;
}
