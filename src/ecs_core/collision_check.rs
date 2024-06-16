use crate::core::broad_phase::BroadPhase;
use crate::core::collider::Collider;
use crate::ecs_core::bodies::{Hitbox, Hurtbox, Velocity};
use crate::ecs_core::world::CollisionWorld;
use crate::ecs_core::LayerGroup;
use crate::utils::Bounded;
use bevy::ecs::system::SystemParam;
use bevy::math::bounding::{Aabb2d, BoundingVolume};
use bevy::prelude::*;
use crate::core::response::{CollisionResponse, ResponseResult};

#[derive(SystemParam)]
pub struct CollisionCheck<'w, 's, Layer: LayerGroup> {
    world: Res<'w, CollisionWorld<Layer>>,
    hurtboxes: Query<'w, 's, &'static Hurtbox<Layer>>,
}

impl<'w, 's, Layer: LayerGroup> CollisionCheck<'w, 's, Layer> {

    pub fn check_intersection(&self, hitbox: &Hitbox<Layer>, hitbox_position: Vec2, offset: Vec2) -> ResponseResult {
        let mut collider = hitbox.collider.clone();
        collider.set_position(collider.position() + hitbox_position);

        let broad_phase = WorldBroadPhase::new(self, hitbox.layer.clone());

        
    }

    pub fn check_intersection_modify(&self, hitbox: &mut Hitbox<Layer>, hitbox_position: Vec2, offset: Vec2) -> ResponseResult {
        let local_position = hitbox.collider.position();
        hitbox.collider.set_position(local_position + hitbox_position);

        let broad_phase = WorldBroadPhase::new(self, hitbox.layer.clone());

        let result = hitbox.response.respond(&broad_phase, &hitbox.collider, offset);
        hitbox.collider.set_position(local_position);

        result
    }
    
    pub fn check_movement(&self, hitbox: &Hitbox<Layer>, hitbox_position: Vec2, offset: Vec2) -> (ResponseResult, Layer::Response) {
        let mut collider = hitbox.collider.clone();
        collider.set_position(collider.position() + hitbox_position);

        let broad_phase = WorldBroadPhase::new(self, hitbox.layer.clone());
        
        let mut response = hitbox.response.clone();
        let result = response.respond(&broad_phase, &collider, offset);
        (result, response)
    }

    pub fn check_movement_modify(&self, hitbox: &mut Hitbox<Layer>, hitbox_position: Vec2, offset: Vec2) -> ResponseResult {
        let local_position = hitbox.collider.position();
        hitbox.collider.set_position(local_position + hitbox_position);

        let broad_phase = WorldBroadPhase::new(self, hitbox.layer.clone());

        let result = hitbox.response.respond(&broad_phase, &hitbox.collider, offset);
        hitbox.collider.set_position(local_position);

        result
    }
}

struct WorldBroadPhase<'a, 'w, 's, Layer: LayerGroup> {
    collision_check: &'a CollisionCheck<'w, 's, Layer>,
    layer: Layer,
}

impl<'a, 'w, 's, Layer: LayerGroup> WorldBroadPhase<'a, 'w, 's, Layer> {
    fn new(collision_check: &'a CollisionCheck<'w, 's, Layer>, layer: Layer) -> Self {
        Self {
            collision_check,
            layer,
        }
    }

    fn iter_hurtboxes_on_aabb(&self, aabb: Aabb2d) -> impl Iterator<Item = &Layer::Hurtbox> {
        let min = self.collision_check.world.global_to_chunk(aabb.min);
        let max = self.collision_check.world.global_to_chunk(aabb.max);

        self.collision_check
            .world
            .chunks
            .iter_rect(min.x, min.y, max.x, max.y)
            .map(|(_, chunk)| {
                chunk.entries.iter().filter_map(|entity| {
                    let hurtbox = self.collision_check.hurtboxes.get(*entity).ok()?;
                    if hurtbox.monitorable && hurtbox.layer.collides(&self.layer) {
                        Some(&hurtbox.collider)
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }
}

impl<'a, 'w, 's, Layer: LayerGroup> BroadPhase<Layer> for WorldBroadPhase<'a, 'w, 's, Layer> {
    fn intersect(&self, hitbox: &Layer::Hitbox) -> impl Iterator<Item = &Layer::Hurtbox> {
        let aabb = hitbox.bounding();
        self.iter_hurtboxes_on_aabb(aabb)
    }
    fn cast(&self, hitbox: &Layer::Hitbox, offset: Vec2) -> impl Iterator<Item = &Layer::Hurtbox> {
        let aabb1 = hitbox.bounding();
        let aabb2 = Aabb2d {
            min: aabb1.min + offset,
            max: aabb1.max + offset,
        };
        let aabb = aabb1.merge(&aabb2);

        self.iter_hurtboxes_on_aabb(aabb)
    }
}

fn main_world_move<Layer: LayerGroup>(
    collision_check: CollisionCheck<Layer>,
    mut hitboxes: Query<(&Hitbox<Layer>, &Velocity, &GlobalTransform, &mut Transform)>,
    time: Res<Time<Virtual>>,
) {
    for (hitbox, velocity, global_transform, _transform) in hitboxes.iter_mut() {
        let position = global_transform.translation().xy();
        let offset = **velocity * time.delta_seconds();

        collision_check.check_movement(hitbox, position, offset);
    }
}

fn world_check_collisions() {}
