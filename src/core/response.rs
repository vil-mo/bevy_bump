use crate::core::broad_phase::BroadPhase;
use crate::core::collider::{ColliderInteraction, CollisionGroup};
use crate::core::layer::CollisionLayer;
use bevy::prelude::*;

/// Collisions are accurate up to the DELTA distance
const DELTA: f32 = 0.0001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    /// The point on the desired path (or on the path corrected by solver) at with collision was detected
    pub point: Vec2,
    /// Normal
    pub normal: Direction2d,
}

fn touch_point<'a, Group, L, C>(
    colliders: impl Iterator<Item = C>,
    actor: &Group::Actor,
    desired_offset: Vec2,
) -> (f32, Option<C>)
where
    Group: CollisionGroup,
    L: CollisionLayer,
    C: AsRef<Group::Target>,
{
    let mut min_dist = desired_offset.length();
    let mut res_item = None;

    for item in colliders {
        let collider = item.as_ref();
        let dist = actor.cast(collider, desired_offset);

        if let Some(dist) = dist {
            if min_dist > dist {
                min_dist = dist - DELTA;
                res_item = Some(item);
            }
        }
    }

    (min_dist, res_item)
}

/// Solver defines how actor will react to met colliders.
/// When actor meets collider it refers to `CollisionResponse`.
///
/// `desired_offset` is what movement was performed to meet this collider
///
/// `info` contains all relevant information about collider
///
/// returns new distance that actor should move from the point at with collision was detected
///
pub type CollisionResponse<
    'a,
    Group: CollisionGroup,
    L: CollisionLayer,
    BF: BroadPhase<'a, Group, L>,
> = fn(
    colliders: &BF,
    actor: &Group::Actor,
    desired_offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>);

pub fn ignore<'a, Group: CollisionGroup, L: CollisionLayer, BF: BroadPhase<'a, Group, L>>(
    _: &BF,
    _: &Group::Actor,
    desired_offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    (desired_offset, vec![])
}

pub fn touch(
    from: Vec2,
    desired_offset: Vec2,
    current_from: Vec2,
    current_offset: Vec2,
    info: CollisionInformation,
) -> Vec2 {
    Vec2::ZERO
}

pub fn slide(
    from: Vec2,
    desired_offset: Vec2,
    current_from: Vec2,
    current_offset: Vec2,
    info: CollisionInformation,
) -> Vec2 {
    // let along = Vec2::from_angle(90.0).rotate(info.normal.deref().clone());
    // let projected = desired_offset.project_onto_normalized(along);
    // let to_move = projected - along * actual_offset;
    //
    // let mut vec_to_append;
    // (actual_offset, vec_to_append) = touch(colliders.clone(), actor, to_move);
    //
    // vec_info.append(&mut vec_to_append);
    //
    //
    // (actual_offset, vec_info)
    todo!()
}
