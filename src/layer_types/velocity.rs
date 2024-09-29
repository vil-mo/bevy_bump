use bevy::prelude::*;

use crate::{
    core::response::CollisionResponse,
    ecs_core::{components::HitboxLayer, CollisionDetectionSet, LayerGroup},
};

use super::collision_report_strategy::CollisionReportStrategy;

pub trait VelocityGroup: LayerGroup {
    type ReportStrategy: CollisionReportStrategy;
    type Response: CollisionResponse;
}

pub(super) fn register_velocity_group<T: VelocityGroup>(app: &mut App) {
    T::ReportStrategy::register::<T>(app);

    app.add_systems(
        super::COLLISION_DETECTION_SCHEDULE,
        collide_velocity_group::<T>.in_set(CollisionDetectionSet::Colliding),
    );

    app.observe(add_velocity::<T>);
}

#[derive(Reflect, Component, Copy, Clone, Default, PartialEq, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

fn add_velocity<Layer: VelocityGroup>(
    trigger: Trigger<OnAdd, HitboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .add(|entity: Entity, world: &mut World| {
            let mut entity_mut = world.entity_mut(entity);
            if !entity_mut.contains::<Velocity>() {
                entity_mut.insert(Velocity::default());
            }
        });
}

fn collide_velocity_group<T: VelocityGroup>() {
    todo!()
}

