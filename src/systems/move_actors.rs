use crate::body::actor::ActorBody;
use crate::body::solid_body::SolidBody;
use bevy::math::bounding::AabbCast2d;
use bevy::prelude::*;
use std::ops::Deref;

pub(super) const SYSTEMS: fn(
    Query<(&mut ActorBody, &mut Transform), Without<SolidBody>>,
    Query<(&SolidBody, &GlobalTransform), Without<ActorBody>>,
) = move_actors;

fn project() {}

const SPACE_GAP: f32 = 0.001;

fn move_actors(
    mut actors: Query<(&mut ActorBody, &mut Transform), Without<SolidBody>>,
    solids: Query<(&SolidBody, &GlobalTransform), Without<ActorBody>>,
) {
    for (mut actor, mut actor_transform) in actors.iter_mut() {
        let Ok(direction) = Direction2d::new(actor.desired_offset) else {
            continue;
        };

        let mut cast = AabbCast2d::new(
            actor.collider.aabb,
            actor.global_pos,
            direction,
            actor.desired_offset.length(),
        );

        for (solid, solid_transform) in solids.iter() {
            let transform = solid_transform.translation().truncate();

            for collider in solid.colliders.iter() {
                let cur_dist = cast.aabb_collision_at(collider.global_aabb(&transform));

                cur_dist
                    .into_iter()
                    .for_each(|x| cast.ray.max = cast.ray.max.min(x - SPACE_GAP));
            }
        }

        let change = *direction.deref() * cast.ray.max;
        actor.global_pos += change;
        actor_transform.translation += change.extend(0.0);
        actor.desired_offset = Vec2::ZERO;
    }
}
