pub mod body;
pub mod scanning;
mod systems;
//pub mod physics_world;

use crate::prelude::{PhysicsSet, PhysicsSetSteps};
use bevy::prelude::*;

pub struct PhysicsAabbPlugin;

impl Plugin for PhysicsAabbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(systems::RegisterSystems);

        app.add_systems(
            FixedPostUpdate,
            body::actor::initialize_actor_body
                .in_set(PhysicsSet)
                .before(PhysicsSetSteps::ApplyVelocity),
        );
    }
}

pub mod prelude {
    pub use crate::systems::{PhysicsSet, PhysicsSetSteps};
    pub use crate::PhysicsAabbPlugin;

    pub use crate::body::actor::{ActorBody, Velocity};
    pub use crate::body::solid_body::SolidBody;
}
