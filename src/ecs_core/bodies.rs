use crate::core::response::{slide, CollisionResponse};
use crate::ecs_core::world::PhysicsWorld;
use crate::ecs_core::ECSCollisionGroup;
use bevy::prelude::*;

#[derive(Component)]
pub struct Actor<Group: ECSCollisionGroup> {
    pub collider: Group::Actor,
    pub layer: Group::Layer,
    pub response: CollisionResponse<Group, PhysicsWorld<Group>>,
    pub monitoring: bool,
}

impl<Group: ECSCollisionGroup> Default for Actor<Group>
where
    Group::Actor: Default,
    Group::Layer: Default,
{
    fn default() -> Actor<Group> {
        Actor {
            collider: Default::default(),
            layer: Default::default(),
            response: slide,
            monitoring: true,
        }
    }
}

impl<Group: ECSCollisionGroup> Actor<Group> {
}

#[derive(Component, Copy, Clone, Default, PartialEq, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec2);
