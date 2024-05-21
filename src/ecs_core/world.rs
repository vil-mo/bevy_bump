use crate::core::broad_phase::BroadPhase;
use crate::ecs_core::ECSCollisionGroup;
use crate::utils::Bounded;
use bevy::math::bounding::{Aabb2d, BoundingVolume};
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::cmp::max;

#[derive(Resource)]
pub struct PhysicsWorld<Group: ECSCollisionGroup> {
    chunks: HashMap<IVec2, Chunk<Group>>,
    chunk_size: IVec2,
}

#[derive(Default)]
pub struct Chunk<Group: ECSCollisionGroup> {
    buf: Vec<Group::Target>,
}

impl<Group: ECSCollisionGroup> PhysicsWorld<Group> {
    pub fn chunk_to_world(&self, chunk: IVec2) -> Vec2 {
        (chunk * self.chunk_size).as_vec2()
    }

    pub fn world_to_chunk(&self, world: Vec2) -> IVec2 {
        IVec2 {
            x: (world.x / self.chunk_size.x as f32).floor() as i32,
            y: (world.x / self.chunk_size.x as f32).floor() as i32,
        }
    }

    pub fn chunks_on_segment(&self, min: Vec2, max: Vec2) -> impl Iterator<Item = &Chunk<Group>> {
        // TODO: optimize algorithm, currently it returns all the chunks on aabb of segment
        let min_chunk = self.world_to_chunk(min);
        let max_chunk = self.world_to_chunk(max);

        (min_chunk.x..=max_chunk.x)
            .into_iter()
            .map(move |x| {
                (min_chunk.y..=max_chunk.y)
                    .into_iter()
                    .filter_map(move |y| self.chunks.get(&IVec2::new(x, y)))
            })
            .flatten()
    }

    pub fn chunks_on_projected_aabb(
        &self,
        aabb: &Aabb2d,
        offset: Vec2,
    ) -> impl Iterator<Item = &Chunk<Group>> {
        // TODO: optimize algorithm, currently it returns all the chunks on aabb of movement

        let offset_aabb = Aabb2d {
            min: aabb.min + offset,
            max: aabb.max + offset,
        };

        let merged_aabb = aabb.merge(&offset_aabb);

        self.chunks_on_segment(merged_aabb.min, merged_aabb.max)
    }
}

impl<Group: ECSCollisionGroup> BroadPhase<Group> for PhysicsWorld<Group> {
    fn intersect(&self, actor: &Group::Actor) -> impl Iterator<Item = &Group::Target> {
        let actor_aabb = actor.bounding();
        todo!();
        vec![].into_iter()
    }

    fn cast(&self, actor: &Group::Actor, offset: Vec2) -> impl Iterator<Item = &Group::Target> {
        todo!();
        vec![].into_iter()
    }
}
