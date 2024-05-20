use super::collider::CollisionGroup;
use bevy::prelude::*;

/// BroadPhase returns iterators over 
pub trait BroadPhase<Group: CollisionGroup> {
    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn cast(&self, actor: &Group::Actor, offset: Vec2) -> impl Iterator<Item = &Group::Target>;

    /// Should return only colliders that are potentially colliding with actor,
    /// and only thing that could prevent collision is stored in collider itself (usually it`s only position)
    fn intersect(&self, actor: &Group::Actor) -> impl Iterator<Item = &Group::Target>;
}
