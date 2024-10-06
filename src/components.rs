use super::ColliderGroup;
use bevy::prelude::{Component, Deref, DerefMut};
/// Shape of the hitbox. Stores [`ColliderGroup::Hitbox`](crate::core::ColliderGroup::Hitbox).
/// Every entity can have only one hitbox.
#[derive(Component, Deref, DerefMut)]
pub struct HitboxShape<Group: ColliderGroup>(pub Group::Hitbox);

#[derive(Component, Deref)]
pub struct HurtboxShape<Group: ColliderGroup>(pub Group::Hurtbox);
