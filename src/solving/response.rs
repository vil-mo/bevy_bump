use bevy::prelude::*;
use std::ops::Deref;

const SPACE_GAP: f32 = 0.001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug, Clone)]
pub struct CollisionInformation {
    /// The point on the desired path (or on the path corrected by solver) at with collision was detected
    pub position: Vec2,
    /// Normal
    pub normal: Direction2d,
}
//
// /// Solver defines how actor will react to met colliders.
// /// Most of the time it will be to move actor along the path until it hits solid collider,
// /// then correct its path and solve again for the new path
// pub type CollisionResponse<Q: SolidCollidersAccess> = fn(
//     colliders: &mut Q,
//     actor: BoundingCircle,
//     desired_offset: Vec2,
// ) -> (Vec2, Vec<CollisionInformation>);
//
// /// Doesn't react to
// pub fn ignore<Q: SolidCollidersAccess>(
//     _colliders: &mut Q,
//     _actor: BoundingCircle,
//     desired_offset: Vec2,
// ) -> (Vec2, Vec<CollisionInformation>) {
//     (desired_offset, vec![])
// }
//
// /// Guaranteed to return either one item in `Vec<CollisionInformation>` or none
// pub fn touch<Q: SolidCollidersAccess>(
//     colliders: &mut Q,
//     actor: BoundingCircle,
//     desired_offset: Vec2,
// ) -> (Vec2, Vec<CollisionInformation>) {
//     let Ok(direction) = Direction2d::new(desired_offset) else {
//         (desired_offset, vec![])
//     };
//     let length  = desired_offset.length();
//
//     let mut cast = BoundingCircleCast::new(actor, Vec2::ZERO, direction, length);
//
//     for aabb in colliders.bounding_circle_cast(&cast) {
//         let cur_dist = cast.aabb_collision_at(aabb);
//
//         cur_dist
//             .into_iter()
//             .for_each(|x| cast.ray.max = cast.ray.max.min(x - SPACE_GAP));
//     }
//
//     let actual_pos = *direction.deref() * cast.ray.max;
//
//     (actual_pos, vec![])
// }
//
// pub fn slide<Q: SolidCollidersAccess>(
//     colliders: &mut Q,
//     actor: BoundingCircle,
//     desired_offset: Vec2,
// ) -> (Vec2, Vec<CollisionInformation>) {
//     let (mut actual_offset, mut vec_info) = touch(colliders.clone(), actor, desired_offset);
//     let mut opt_info = vec_info.first();
//
//     while let Some(info) = opt_info {
//         let along = Vec2::from_angle(90.0).rotate(info.normal.deref().clone());
//         let projected = desired_offset.project_onto_normalized(along);
//         let to_move = projected - along * actual_offset;
//
//         let mut vec_to_append;
//         (actual_offset, vec_to_append) = touch(colliders.clone(), actor, to_move);
//
//         vec_info.append(&mut vec_to_append);
//     }
//
//     (actual_offset, vec_info)
// }
