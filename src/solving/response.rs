use std::ops::Deref;
use crate::solving::collider::{ColliderCast, CollidersConfig};
use crate::solving::collider_query::ColliderQueryCast;
use crate::solving::layer::CollisionLayer;
use bevy::prelude::*;
use enum_map::EnumMap;

const SPACE_GAP: f32 = 0.0001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    /// The point on the desired path (or on the path corrected by solver) at with collision was detected
    pub position: Vec2,
    /// Normal
    pub normal: Direction2d,
}

fn get_touch_point<'a, CF: CollidersConfig, L: CollisionLayer>(
    colliders: impl Iterator<Item = &'a CF::SolidCollider>,
    actor: &CF::ActorCollider,
    desired_offset: Vec2,
) -> (f32, Option<&'a CF::SolidCollider>) {
    let mut min_dist = desired_offset.length();
    let mut res_collider = None;

    for collider in colliders {
        let dist = actor.cast(collider, desired_offset);

        if let Some(dist) = dist {
            if min_dist > dist {
                min_dist = dist - SPACE_GAP;
                res_collider = Some(collider);
            }
        }
    }

    (min_dist, res_collider)
}

fn solve<'a, CF: CollidersConfig, L: CollisionLayer>(
    colliders: &impl ColliderQueryCast<'a, CF::SolidCollider, CF::ActorCollider, L>,
    actor: &CF::ActorCollider,
    actor_layers: &EnumMap<L, CollisionResponse>,
    desired_offset: Vec2,
) -> (Vec2, Vec<CollisionInformation>) {
    let mut collision_information = vec![];

    loop {
        if desired_offset.abs_diff_eq(Vec2::ZERO, SPACE_GAP * 2.0) {
            return (desired_offset, collision_information);
        }

        let touch_point = get_touch_point::<CF, L>(
            colliders.cast(actor, desired_offset).filter_map(|(c, l)| {
                if actor_layers[l] == ignore {
                    None
                } else {
                    Some(c)
                }
            }),
            actor,
            desired_offset,
        );
        
        
    }
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
pub type CollisionResponse = fn(desired_offset: Vec2, info: CollisionInformation) -> Vec2;

pub fn ignore(desired_offset: Vec2, info: CollisionInformation) -> Vec2 {
    desired_offset - info.position
}

pub fn touch(_: Vec2, _: CollisionInformation) -> Vec2 {
    Vec2::ZERO
}


pub fn slide(desired_offset: Vec2, info: CollisionInformation) -> Vec2{
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
