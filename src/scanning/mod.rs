use crate::collider::Collider;
use bevy::prelude::*;

pub trait Alignment: Eq {}

#[derive(Component, Debug, Clone)]
pub struct Hitbox<A: Alignment> {
    colliders: Box<[Collider]>,
    alignment: A,
}

#[derive(Debug, Clone)]
pub struct Hurtbox<A: Alignment> {
    colliders: Box<[Collider]>,
    alignment: A,
}
