use crate::body::responce::{slide, CollisionResponse, SolidCollidersAccess};
use bevy::math::bounding::BoundingCircle;
use bevy::prelude::*;
use enum_map::{EnumArray, EnumMap};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Component, Debug, Clone, Default)]
pub struct Velocity(pub Vec2);

impl Deref for Velocity {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Velocity {
    pub fn new(initial_velocity: Vec2) -> Self {
        Self(initial_velocity)
    }
}

#[derive(Component, Debug, Clone)]
pub struct ActorBody<L: EnumArray<CollisionResponse<Q>>, Q: SolidCollidersAccess, M = ()> {
    pub(crate) desired_offset: Vec2,
    pub(crate) collider: BoundingCircle,
    pub(crate) solvers: EnumMap<L, CollisionResponse<Q>>,

    _pd: PhantomData<M>,
}

impl<M, Q: SolidCollidersAccess> ActorBody<M, Q> {
    pub fn new(collider: impl Into<BoundingCircle>) -> Self {
        Self {
            desired_offset: Vec2::ZERO,
            collider: collider.into(),

            _pd: PhantomData,
        }
    }

    pub fn walk(&mut self, by: Vec2) {
        self.desired_offset += by;
    }
}

pub(crate) fn initialize_actor_body<M, Q: SolidCollidersAccess>(
    mut actors: Query<
        (
            Entity,
            &mut ActorBody<M, Q>,
            &GlobalTransform,
            Option<&Velocity>,
        ),
        Added<ActorBody<M, Q>>,
    >,
    mut commands: Commands,
) {
    for (entity, mut actor, transform, vel) in actors.iter_mut() {
        if vel.is_none() {
            commands.entity(entity).insert(Velocity::default());
        }
    }
}
