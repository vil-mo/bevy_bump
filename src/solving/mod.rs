use bevy::prelude::*;
use std::marker::PhantomData;
use std::ops::Deref;
use bevy::utils::HashSet;

pub mod response;

pub trait Collider<ID: Eq = i32> {
    fn id(&self) -> ID;
}
pub trait ColliderCast<T: Collider<ID>, ID: Eq = i32>: Collider<ID> {
    fn collision_at(&self, other: &T, offset: Vec2) -> Option<f32>;
}

pub trait ColliderIntersect<T: Collider<ID>, ID: Eq = i32>: Collider<ID> {
    fn intersects(&self, other: &T) -> bool;
}

impl<T: Collider<ID> + ?Sized, ID: Eq> Collider<ID> for Box<T> {
    fn id(&self) -> ID {
        self.deref().id()
    }
}

impl<T: ColliderCast<V, ID> + ?Sized, V: Collider<ID>, ID: Eq> ColliderCast<V, ID> for Box<T> {
    fn collision_at(&self, other: &V, offset: Vec2) -> Option<f32> {
        self.deref().collision_at(other, offset)
    }
}

impl<T: ColliderIntersect<V, ID> + ?Sized, V: Collider<ID>, ID: Eq> ColliderIntersect<V, ID>
    for Box<T>
{
    fn intersects(&self, other: &V) -> bool {
        self.deref().intersects(other)
    }
}

pub trait CollidersConfig<ID: Eq = i32> {
    type SolidCollider: Collider<ID>;
    type ActorCollider: ColliderCast<Self::SolidCollider, ID>;
    type HitboxCollider: Collider<ID>;
    type HurtboxCollider: ColliderIntersect<Self::HitboxCollider, ID>;
}

/// SolidBodyQuery's provide data about colliders of solids
pub trait SolidColliderAccess<C: CollidersConfig<ID>, ID: Eq = i32> {
    fn actor_cast(
        &self,
        actor: &C::ActorCollider,
        offset: Vec2,
    ) -> impl Iterator<Item = C::SolidCollider>;
}

pub struct ColliderAccess<CF: CollidersConfig<ID>, SCA: SolidColliderAccess<CF, ID>, ID: Eq = i32> {
    access: SCA,
    ignored: HashSet<ID>,

    _pd: PhantomData<CF>,
}
