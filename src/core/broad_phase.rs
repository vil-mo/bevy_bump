use super::ColliderGroup;
use bevy::prelude::*;

/// BroadPhase returns iterators over
pub trait BroadPhase<Group: ColliderGroup> {
    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect(&self, hitbox: &Group::Hitbox) -> impl Iterator<Item = &Group::Hurtbox>;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn cast(&self, hitbox: &Group::Hitbox, offset: Vec2) -> impl Iterator<Item = &Group::Hurtbox>;
}
