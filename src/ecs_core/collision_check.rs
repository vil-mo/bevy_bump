use super::{
    components::{HurtboxLayer, HurtboxMonitorable, HurtboxShape},
    spacial_index::{SpacialIndex, SpacialIndexRegistry},
    LayerGroup,
};
use crate::core::{collider::Collider, collisions_query::CollisionsQuery, response::{CollisionResponse, RunningResponse}, ColliderGroup};
use crate::utils::Bounded;
use bevy::ecs::{entity::EntityHashSet, system::SystemParam};
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
    pub fn collisions_on_layer<'a>(&'a self, layer: &'a Layer) -> CollisionsOnLayer<'a, Layer> {
        CollisionsOnLayer {
            inner: self.collisions_on_layer_allow_duplication(layer),
        }
    }

    pub fn collisions_on_layer_allow_duplication<'a>(
        &'a self,
        layer: &'a Layer,
    ) -> CollisionsOnLayerAllowDuplication<'a, Layer> {
        CollisionsOnLayerAllowDuplication::new(self, layer)
    }

    pub fn check_intersection<'a>(
        &'a self,
        hitbox: Collider<'a, Layer::Hitbox>,
        layer: &'a Layer,
    ) -> impl Iterator<Item = Entity> + 'a {
        let collisions = self.collisions_on_layer(layer);
        collisions.intersect(hitbox)
    }

    pub fn check_movement<'a>(
        &'a self,
        hitbox: Collider<'a, Layer::Hitbox>,
        offset: Vec2,
        layer: &'a Layer,
        response: &'a mut impl CollisionResponse,
    ) -> impl RunningResponse<Layer> + 'a {
        let collisions = self.collisions_on_layer(layer);
        response.respond(collisions, hitbox, offset)
    }
}

pub struct CollisionsOnLayerAllowDuplication<'a, Layer: LayerGroup> {
    collision_check: &'a CollisionCheck<'a, 'a, Layer>,
    layer: &'a Layer,
}

impl<Layer: LayerGroup> Clone for CollisionsOnLayerAllowDuplication<'_, Layer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Layer: LayerGroup> Copy for CollisionsOnLayerAllowDuplication<'_, Layer> {}

impl<'a, Layer: LayerGroup> CollisionsOnLayerAllowDuplication<'a, Layer> {
    fn new(collision_check: &'a CollisionCheck<'a, 'a, Layer>, layer: &'a Layer) -> Self {
        Self {
            collision_check,
            layer,
        }
    }

    fn iter_hurtboxes_on_aabb(
        self,
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

impl<Layer: LayerGroup> CollisionsQuery<Layer> for CollisionsOnLayerAllowDuplication<'_, Layer> {
    fn intersect(
        self,
        hitbox: Collider<<Layer as ColliderGroup>::Hitbox>,
    ) -> impl Iterator<Item = Entity> {
        let aabb = hitbox.bounding();
        self.iter_hurtboxes_on_aabb(aabb)
            .filter_map(move |(other, data)| {
                if hitbox.intersect(other) {
                    Some(data)
                } else {
                    None
                }
            })
    }

    fn cast(
        self,
        hitbox: Collider<<Layer as ColliderGroup>::Hitbox>,
        offset: Vec2,
    ) -> impl Iterator<Item = (f32, Dir2, Entity)> {
        let aabb1 = hitbox.bounding();
        let aabb2 = Aabb2d {
            min: aabb1.min + offset,
            max: aabb1.max + offset,
        };
        let aabb = aabb1.merge(&aabb2);

        self.iter_hurtboxes_on_aabb(aabb)
            .filter_map(move |(other, data)| {
                if let Some((dist, norm)) = hitbox.cast(other, offset) {
                    Some((dist, norm, data))
                } else {
                    None
                }
            })
    }
}

pub struct CollisionsOnLayer<'a, Layer: LayerGroup> {
    inner: CollisionsOnLayerAllowDuplication<'a, Layer>,
}

impl<Layer: LayerGroup> Clone for CollisionsOnLayer<'_, Layer> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Layer: LayerGroup> Copy for CollisionsOnLayer<'_, Layer> {}

impl<Layer: LayerGroup> CollisionsQuery<Layer> for CollisionsOnLayer<'_, Layer> {
    fn intersect(self, hitbox: Collider<Layer::Hitbox>) -> impl Iterator<Item = Entity> {
        let mut deduplication_set = EntityHashSet::default();

        let aabb = hitbox.bounding();
        self.inner
            .iter_hurtboxes_on_aabb(aabb)
            .filter(move |(_, entity)| deduplication_set.insert(*entity))
            .filter_map(move |(other, data)| {
                if hitbox.intersect(other) {
                    Some(data)
                } else {
                    None
                }
            })
    }

    fn cast(
        self,
        hitbox: Collider<Layer::Hitbox>,
        offset: Vec2,
    ) -> impl Iterator<Item = (f32, Dir2, Entity)> {
        let mut deduplication_set = EntityHashSet::default();

        let aabb1 = hitbox.bounding();
        let aabb2 = Aabb2d {
            min: aabb1.min + offset,
            max: aabb1.max + offset,
        };
        let aabb = aabb1.merge(&aabb2);

        self.inner
            .iter_hurtboxes_on_aabb(aabb)
            .filter(move |(_, entity)| deduplication_set.insert(*entity))
            .filter_map(move |(other, data)| {
                if let Some((dist, norm)) = hitbox.cast(other, offset) {
                    Some((dist, norm, data))
                } else {
                    None
                }
            })
    }
}
