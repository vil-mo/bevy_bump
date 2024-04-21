use crate::solving::layer::CollisionLayer;
use bevy::prelude::*;
use std::hash::Hash;
use std::ops::Deref;

pub trait Collider {}

pub trait ColliderCast<T>: Collider
where
    T: Collider,
{
    fn cast(&self, other: &T, offset: Vec2) -> Option<f32>;
}

pub trait ColliderIntersect<T>: Collider
where
    T: Collider,
{
    fn intersect(&self, other: &T) -> bool;
}

impl<T> Collider for Box<T> where T: Collider + ?Sized {}

impl<T, V> ColliderCast<V> for Box<T>
where
    T: ColliderCast<V> + ?Sized,
    V: Collider,
{
    fn cast(&self, other: &V, offset: Vec2) -> Option<f32> {
        self.deref().cast(other, offset)
    }
}

impl<T, V> ColliderIntersect<V> for Box<T>
where
    T: ColliderIntersect<V> + ?Sized,
    V: Collider,
{
    fn intersect(&self, other: &V) -> bool {
        self.deref().intersect(other)
    }
}

pub trait CollidersConfig: 'static {
    type SolidCollider: Collider;
    type ActorCollider: ColliderCast<Self::SolidCollider>;
    type HitboxCollider: Collider;
    type HurtboxCollider: ColliderIntersect<Self::HitboxCollider>;
}
