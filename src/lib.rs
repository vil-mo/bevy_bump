pub mod core;
mod systems;

use bevy::prelude::*;

pub struct PhysicsAabbPlugin;

impl Plugin for PhysicsAabbPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(systems::RegisterSystems);
    }
}
