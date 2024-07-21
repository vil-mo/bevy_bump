use crate::core::broad_phase::{BroadPhase, BroadPhaseIteratorOutput};
use crate::core::collider::Collider;
use crate::core::ColliderGroup;
use crate::ecs_core::components::{HurtboxLayer, HurtboxMonitorable, HurtboxShape};
use crate::ecs_core::spacial_index::{SpacialIndex, SpacialIndexRegistry};
use crate::ecs_core::LayerGroup;
use crate::utils::Bounded;
use bevy::ecs::system::SystemParam;
use bevy::math::bounding::{Aabb2d, BoundingVolume};
use bevy::prelude::*;

#[derive(SystemParam)]
pub struct CollisionCheck<'w, 's, Layer: LayerGroup> {
    world: Res<'w, SpacialIndex<Layer>>,
    hurtboxes: Query<
        'w,
        's,
        (
            &'static HurtboxShape<Layer>,
            &'static HurtboxLayer<Layer>,
            &'static SpacialIndexRegistry<Layer>,
            Option<&'static HurtboxMonitorable<Layer>>,
        ),
    >,
}

impl<'w, 's, Layer: LayerGroup> CollisionCheck<'w, 's, Layer> {
    pub fn check_intersection<'a>(
        &'a self,
        hitbox: Collider<'a, Layer::Hitbox>,
        layer: &'a Layer,
    ) -> impl Iterator<Item = Entity> + 'a {
        let broad_phase = WorldBroadPhase::new(self, layer);

        broad_phase
            // .intersect(hitbox)
            .iter_hurtboxes_on_aabb(hitbox.bounding())
            .filter_map(move |(hurtbox, entity)| {
                if hitbox.intersect(hurtbox) {
                    Some(entity)
                } else {
                    None
                }
            })
    }

    // pub fn check_movement(
    //     &self,
    //     hitbox: &Hitbox<Layer>,
    //     hitbox_position: Vec2,
    //     offset: Vec2,
    // ) -> (
    //     ResponseResult<Layer, WorldBroadPhase<Layer>>,
    //     Layer::Response,
    // ) {
    //     let mut collider = hitbox.collider.clone();
    //     collider.set_position(collider.position() + hitbox_position);
    //
    //     let broad_phase = WorldBroadPhase::new(self, hitbox.layer.clone());
    //
    //     let mut response = hitbox.response.clone();
    //     let result = response.respond(&broad_phase, &collider, offset);
    //     (result, response)
    // }
    // pub fn check_movement(
    //     &self,
    //     hitbox: Collider<Layer::Hitbox>,
    //     layer: &Layer,
    //     offset: Vec2,
    // ) -> impl Iterator<Item = Entity> {
    //     let broad_phase = WorldBroadPhase::new(self, layer);
    //
    //     broad_phase
    //         .intersect(hitbox)
    //         .filter_map(move |(hurtbox, entity)| {
    //             if hitbox.intersect(hurtbox) {
    //                 Some(entity)
    //             } else {
    //                 None
    //             }
    //         })
    // }
}

pub struct WorldBroadPhase<'a, 'w, 's, Layer: LayerGroup> {
    collision_check: &'a CollisionCheck<'w, 's, Layer>,
    layer: &'a Layer,
}

impl<'a, 'w, 's, Layer: LayerGroup> WorldBroadPhase<'a, 'w, 's, Layer> {
    fn new(collision_check: &'a CollisionCheck<'w, 's, Layer>, layer: &'a Layer) -> Self {
        Self {
            collision_check,
            layer,
        }
    }

    fn iter_hurtboxes_on_aabb(
        &self,
        aabb: Aabb2d,
    ) -> impl Iterator<Item = (Collider<'a, Layer::Hurtbox>, Entity)> {
        self.collision_check
            .world
            .iter_chunks_on_aabb(aabb)
            .map(|chunk| {
                chunk.iter().filter_map(|&entity| {
                    let (shape, layer, registry, monitorable) =
                        self.collision_check.hurtboxes.get(entity).ok()?;
                    let monitorable = *monitorable.cloned().unwrap_or_default();

                    if monitorable && self.layer.collides(layer) {
                        Some((Collider::new(&**shape, registry.current_position), entity))
                    } else {
                        None
                    }
                })
            })
            .flatten()
    }
}

impl<'a, 'w, 's, Layer: LayerGroup> BroadPhase<'a, Layer> for WorldBroadPhase<'a, 'w, 's, Layer> {
    fn intersect(
        &self,
        hitbox: Collider<Layer::Hitbox>,
    ) -> impl Iterator<Item = (Collider<'a, Layer::Hurtbox>, Entity)> {
        let aabb = hitbox.bounding();
        self.iter_hurtboxes_on_aabb(aabb)
    }

    fn cast(
        &self,
        hitbox: Collider<Layer::Hitbox>,
        offset: Vec2,
    ) -> impl Iterator<Item = BroadPhaseIteratorOutput<'a, Layer>> {
        let aabb1 = hitbox.bounding();
        let aabb2 = Aabb2d {
            min: aabb1.min + offset,
            max: aabb1.max + offset,
        };
        let aabb = aabb1.merge(&aabb2);

        self.iter_hurtboxes_on_aabb(aabb)
    }
}
