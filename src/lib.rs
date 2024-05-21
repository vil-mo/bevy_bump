pub mod core;
pub mod ecs_core;
mod systems;
pub mod utils;

use bevy::prelude::*;

pub mod prelude {}

pub struct PhysicsAabbPlugin;

impl Plugin for PhysicsAabbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(systems::RegisterSystems);
    }
}
