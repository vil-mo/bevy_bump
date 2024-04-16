mod apply_velocity;
mod move_actors;

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct PhysicsSet;
#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PhysicsSetSteps {
    ApplyVelocity,
    MoveActors,
    CheckHits,
}
pub(crate) struct RegisterSystems;

impl Plugin for RegisterSystems {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            FixedPostUpdate,
            (
                PhysicsSetSteps::ApplyVelocity.in_set(PhysicsSet),
                PhysicsSetSteps::MoveActors
                    .in_set(PhysicsSet)
                    .after(PhysicsSetSteps::ApplyVelocity),
                PhysicsSetSteps::CheckHits
                    .in_set(PhysicsSet)
                    .after(PhysicsSetSteps::MoveActors),
            ),
        );

        app.add_systems(
            FixedPostUpdate,
            apply_velocity::SYSTEMS.in_set(PhysicsSetSteps::ApplyVelocity),
        );
        app.add_systems(
            FixedPostUpdate,
            move_actors::SYSTEMS.in_set(PhysicsSetSteps::MoveActors),
        );
    }
}
