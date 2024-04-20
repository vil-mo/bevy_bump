pub mod body;
pub mod scanning;
pub mod solving;
mod systems;
//pub mod physics_world;

use bevy::prelude::*;

pub struct PhysicsAabbPlugin;

impl Plugin for PhysicsAabbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(systems::RegisterSystems);
    }
}
