use bevy::{ecs::system::SystemParam, prelude::*, utils::all_tuples};

use crate::{
    core::{response::ResponseCollisionInformation, ColliderGroup},
    ecs_core::LayerGroup,
};

#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    pub hitbox: Entity,
    pub hurtbox: Entity,
    pub global_position: Vec2,
    pub normal: Option<Dir2>,
}

impl CollisionInformation {
    pub fn from_response<Group: ColliderGroup>(
        hitbox: Entity,
        response: ResponseCollisionInformation<Group>,
    ) -> Self
    where
        Group::HurtboxData: Into<Entity>,
    {
        Self {
            hitbox,
            hurtbox: response.data.into(),
            global_position: response.global_position,
            normal: Some(response.normal),
        }
    }
}

// TODO: Document about system state
pub trait CollisionReportStrategy {
    type Param<Layer: LayerGroup>: SystemParam;

    fn register<Layer: LayerGroup>(app: &mut App);

    fn report_collisions<Layer: LayerGroup>(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param<Layer> as SystemParam>::Item<'_, '_>,
    );
}

macro_rules! impl_collision_report_strategy {
    ($(($t:ident, $p:ident)),*) => {
        impl<$($t: CollisionReportStrategy),*> CollisionReportStrategy for ($($t,)*)
        {
            type Param<Layer: LayerGroup> = ParamSet<'static, 'static, ($($t::Param<Layer>,)*)>;

            fn register<Layer: LayerGroup>(app: &mut App) {
                $($t::register::<Layer>(app);)*
            }

            fn report_collisions<Layer: LayerGroup>(
                collisions: impl Iterator<Item = CollisionInformation>,
                param: &mut <Self::Param<Layer> as SystemParam>::Item<'_, '_>,
            ) {
                for collision in collisions {
                    $($t::report_collisions::<Layer>(std::iter::once(collision), &mut param.$p());)*
                }
            }
        }
    };
}

all_tuples!(impl_collision_report_strategy, 1, 8, T, p);

#[derive(Event)]
pub struct Collided(pub CollisionInformation);

pub struct SendCollisionEvent;

impl CollisionReportStrategy for SendCollisionEvent {
    type Param<Layer: LayerGroup> = EventWriter<'static, Collided>;

    fn register<Layer: LayerGroup>(app: &mut App) {
        app.add_event::<Collided>();
    }

    fn report_collisions<Layer: LayerGroup>(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param<Layer> as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            param.send(Collided(collision));
        }
    }
}

pub struct ObseveCollision;

impl CollisionReportStrategy for ObseveCollision {
    type Param<Layer: LayerGroup> = Commands<'static, 'static>;

    fn register<Layer: LayerGroup>(app: &mut App) {}

    fn report_collisions<Layer: LayerGroup>(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param<Layer> as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            param.trigger(Collided(collision));
            param.trigger_targets(Collided(collision), [collision.hitbox, collision.hurtbox]);
        }
    }
}

pub struct Collisions {
    
}

pub struct AddToCollisionsResource;
