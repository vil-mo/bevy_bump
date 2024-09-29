use bevy::{
    ecs::system::SystemParam,
    math::{Dir2, Vec2},
    prelude::Entity,
};

use crate::{
    core::{response::ResponseCollisionInformation, ColliderGroup},
    ecs_core::LayerGroup,
};

#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    pub entity: Entity,
    pub global_position: Vec2,
    pub normal: Option<Dir2>,
}

impl<Group: ColliderGroup> From<ResponseCollisionInformation<Group>> for CollisionInformation
where
    Group::HurtboxData: Into<Entity>,
{
    fn from(response: ResponseCollisionInformation<Group>) -> Self {
        Self {
            entity: response.data.into(),
            global_position: response.global_position,
            normal: Some(response.normal),
        }
    }
}

// TODO: Document about system state
pub trait CollisionReportStrategy {
    type Param<Layer: LayerGroup>: SystemParam;

    fn report_collisions<Layer: LayerGroup>(
        hitbox: Entity,
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param<Layer> as SystemParam>::Item<'_, '_>,
    );
}
