use crate::core::broad_phase::BroadPhase;
use crate::core::collider::{Collider, ColliderInteraction, CollisionGroup};
use bevy::prelude::*;

/// Collisions are accurate up to the DELTA distance
const DELTA: f32 = 0.0001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    /// The point on the desired path (or on the path corrected by solver) at with collision was detected
    /// Should make sense for it to be [`Collider::position`] of actor that performed movement 
    pub point: Vec2,
    /// Result of [`Collider::normal`] of body against which collision was detected
    pub normal: Direction2d,
}

/// Solver defines how actor will react to met colliders.
/// When actor meets collider it refers to `CollisionResponse`.
///
/// `colliders` is a broad phase
/// `actor` is actor that performs the collision
/// 
/// `offset` is offset that `actor` desires to move this call
/// 
/// Returns actual offset that actor should move from the point at with collision was detected 
/// and information about all the collisions that happened
pub type CollisionResponse<Group: CollisionGroup, BF: BroadPhase<Group>> =
    fn(
        colliders: &BF,
        actor: &<Group as CollisionGroup>::Actor,
        offset: Vec2,
    ) -> (Vec2, Vec<CollisionInformation>);

pub fn ignore<Group: CollisionGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Actor,
    offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    let position = actor.position();
    let normal = offset.normalize();

    (
        offset,
        colliders
            .cast(actor, offset)
            .filter_map(|other| {
                actor.cast(other, offset).map(|dist| CollisionInformation {
                    point: position + normal * dist,
                    normal: other.normal(position),
                })
            })
            .collect(),
    )
}

fn touch_point<'a, Group: CollisionGroup>(
    colliders: impl Iterator<Item = &'a Group::Target>,
    actor: &Group::Actor,
    desired_offset: Vec2,
) -> Option<(f32, &'a Group::Target)> {
    let mut res = None;

    for collider in colliders {
        let dist = actor.cast(collider, desired_offset);

        if let Some(dist) = dist {
            if let Some((old_dist, _)) = res {
                if old_dist > dist {
                    res = Some((dist - DELTA, collider));
                }
            } else {
                res = Some((dist - DELTA, collider));
            }
        }
    }

    res
}

pub fn touch_light<Group: CollisionGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Actor,
    offset: Vec2,
) -> (Vec2, Option<CollisionInformation>) {
    match touch_point::<Group>(colliders.cast(actor, offset), actor, offset) {
        None => (offset, None),
        Some((dist, other)) => {
            let position = actor.position();
            let point = position + offset.normalize() * dist;

            (
                point,
                Some(CollisionInformation {
                    point,
                    normal: other.normal(position),
                }),
            )
        }
    }
}

pub fn touch<Group: CollisionGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Actor,
    offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    let (offset, collider) = touch_light(colliders, actor, offset);

    (offset, collider.into_iter().collect())
}

pub fn slide<Group: CollisionGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Actor,
    offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    let mut res_vec = Vec::new();

    let mut last_offset = offset;
    let (mut new_offset, mut opt_info) = touch_light(colliders, actor, offset);

    let mut actor = actor.clone();
    
    while let Some(info) = opt_info {
        actor.set_position(info.point);
        
        let along = info.normal.perp();
        let diff_offset = last_offset - new_offset;
        
        last_offset = diff_offset.project_onto_normalized(along);
        (new_offset, opt_info) = touch_light(colliders, &actor, last_offset);
        
        res_vec.push(info);
    }

    (actor.position() + new_offset, res_vec)
}

pub fn bounce<Group: CollisionGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Actor,
    offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    let mut res_vec = Vec::new();

    let mut last_offset = offset;
    let (mut new_offset, mut opt_info) = touch_light(colliders, actor, offset);

    let mut actor = actor.clone();

    while let Some(info) = opt_info {
        actor.set_position(info.point);

        let along = info.normal.perp();
        let diff_offset = last_offset - new_offset;

        last_offset = diff_offset - 2.0 * diff_offset.reject_from_normalized(along);
        (new_offset, opt_info) = touch_light(colliders, &actor, last_offset);

        res_vec.push(info);
    }

    (actor.position() + new_offset, res_vec)
}