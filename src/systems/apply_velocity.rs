// use crate::body::actor::{ActorBody, Velocity};
// use crate::body::solid_body::SolidBody;
// use bevy::prelude::*;
//
// pub(super) const SYSTEMS: fn(Query<(&mut ActorBody, &Velocity), Without<SolidBody>>, Res<Time>) =
//     apply_velocity;
//
// fn apply_velocity(
//     mut actors: Query<(&mut ActorBody, &Velocity), Without<SolidBody>>,
//     time: Res<Time>,
// ) {
//     for (mut actor, vel) in actors.iter_mut() {
//         actor.desired_offset += vel.0 * time.delta_seconds();
//     }
// }
