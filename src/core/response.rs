use super::{
    broad_phase::BroadPhase,
    collider::{Collider, ColliderInteraction},
    ColliderGroup,
};
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

#[derive(Debug, Clone)]
pub struct ResponseResult {
    pub actual_position: Vec2,
    pub collisions: Vec<CollisionInformation>,
}

impl ResponseResult {
    #[inline]
    pub fn new(actual_position: Vec2, collisions: Vec<CollisionInformation>) -> Self {
        Self {
            actual_position,
            collisions,
        }
    }
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
pub trait CollisionResponse: Clone {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        colliders: &BF,
        actor: &<Group as ColliderGroup>::Hitbox,
        offset: Vec2,
    ) -> ResponseResult;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ignore;

impl CollisionResponse for Ignore {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        _: &BF,
        _: &Group::Hitbox,
        offset: Vec2,
    ) -> ResponseResult {
        ResponseResult::new(offset, vec![])
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pass;
impl CollisionResponse for Pass {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        colliders: &BF,
        actor: &Group::Hitbox,
        offset: Vec2,
    ) -> ResponseResult {
        let position = actor.position();
        let normal = offset.normalize();

        ResponseResult {
            actual_position: offset,
            collisions: colliders
                .cast(actor, offset)
                .filter_map(|other| {
                    actor
                        .cast(other, offset)
                        .map(|(dist, norm)| CollisionInformation {
                            point: position + normal * dist,
                            normal: norm,
                        })
                })
                .collect(),
        }
    }
}

fn touch_point<'a, T: Collider + 'a, A: ColliderInteraction<T>>(
    colliders: impl Iterator<Item = &'a T>,
    actor: &A,
    desired_offset: Vec2,
) -> (Vec2, Option<CollisionInformation>) {
    let mut res = None;
    let offset_normal = desired_offset.normalize();
    let position = actor.position();

    for collider in colliders {
        let dist = actor.cast(collider, desired_offset);

        if let Some((dist, normal)) = dist {
            if let Some((old_dist, _)) = res {
                if old_dist > dist {
                    res = Some((dist, normal));
                }
            } else {
                res = Some((dist, normal));
            }
        }
    }

    if let Some(res) = res {
        let actual_offset = offset_normal * (res.0 - DELTA);
        (
            actual_offset,
            Some(CollisionInformation {
                point: actual_offset + position,
                normal: res.1,
            }),
        )
    } else {
        (desired_offset, None)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Touch;
impl CollisionResponse for Touch {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        colliders: &BF,
        actor: &Group::Hitbox,
        offset: Vec2,
    ) -> ResponseResult {
        let (offset, collider) = touch_point(colliders.cast(actor, offset), actor, offset);

        ResponseResult::new(offset, collider.into_iter().collect())
    }
}

fn trajectory_change_on_touch<Group: ColliderGroup, BF: BroadPhase<Group>>(
    colliders: &BF,
    actor: &Group::Hitbox,
    offset: Vec2,
    // (movement that was supposed to be made, but stopped, normal of the collision) -> new movement from position where were stopped
    mut new_trajectory: impl FnMut(Vec2, Direction2d) -> Vec2,
) -> ResponseResult {
    let mut res_vec = Vec::new();

    let mut last_offset = offset;
    let (mut new_offset, mut opt_info) = touch_point(colliders.cast(actor, offset), actor, offset);

    let mut actor = actor.clone();

    while let Some(info) = opt_info {
        actor.set_position(info.point);

        let diff_offset = last_offset - new_offset;

        last_offset = new_trajectory(diff_offset, info.normal);
        (new_offset, opt_info) = touch_point(colliders.cast(&actor, offset), &actor, last_offset);

        res_vec.push(info);
    }

    ResponseResult::new(actor.position() + new_offset, res_vec)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slide;

impl CollisionResponse for Slide {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        colliders: &BF,
        actor: &Group::Hitbox,
        offset: Vec2,
    ) -> ResponseResult {
        trajectory_change_on_touch(colliders, actor, offset, |left_movement, normal| {
            left_movement.project_onto_normalized(normal.perp())
        })
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounce;

impl CollisionResponse for Bounce {
    fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
        &mut self,
        colliders: &BF,
        actor: &Group::Hitbox,
        offset: Vec2,
    ) -> ResponseResult {
        trajectory_change_on_touch(colliders, actor, offset, |left_movement, normal| {
            left_movement - 2.0 * left_movement.project_onto_normalized(*normal)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimitedBounce(pub u32);

impl LimitedBounce {
    #[inline]
    pub fn new(bounces: u32) -> Self {
        Self(bounces)
    }
}
