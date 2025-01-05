use super::{
    components::SpatialIndexRegistry, spatial_index::SpatialIndex, SpatialIndexColliderGroup,
};
use crate::{
    bounded::Bounded,
    collider::Collider,
    components::HurtboxShape,
    spatial_query::{
        filter::{
            HitboxFilterSystemParam, HurtboxFilterSystemParam, SpatialQueryFilter,
            SystemSpatialQueryFilter,
        },
        SpatialQuery,
    },
};
use bevy::{
    ecs::{
        entity::EntityHashSet,
        system::{StaticSystemParam, SystemParam},
    },
    math::bounding::{Aabb2d, BoundingVolume},
    prelude::*,
};

#[derive(SystemParam)]
struct GenericSpatialIndexQuery<
    'w,
    's,
    Group: SpatialIndexColliderGroup,
    I: IterHurtboxesOnAabb,
> {
    index: Res<'w, SpatialIndex<Group>>,
    hurtboxes: Query<
        'w,
        's,
        (
            &'static HurtboxShape<Group>,
            &'static SpatialIndexRegistry<Group>,
        ),
    >,
    hitbox_filter: StaticSystemParam<'w, 's, HitboxFilterSystemParam<Group>>,
    hurtbox_filter: StaticSystemParam<'w, 's, HurtboxFilterSystemParam<Group>>,
    marker: std::marker::PhantomData<fn() -> I>,
}

trait IterHurtboxesOnAabb: Sized + Send + Sync + 'static {
    fn iter_hurtboxes_on_aabb<'w, 'a, Group: SpatialIndexColliderGroup>(
        index: &'a mut Res<'w, SpatialIndex<Group>>,
        aabb: Aabb2d,
    ) -> impl Iterator<Item = Entity>;
}

struct AllowDuplication;

impl IterHurtboxesOnAabb for AllowDuplication {
    fn iter_hurtboxes_on_aabb<'w, 'a, Group: SpatialIndexColliderGroup>(
        index: &'a mut Res<'w, SpatialIndex<Group>>,
        aabb: Aabb2d,
    ) -> impl Iterator<Item = Entity> {
        index
            .iter_chunks_on_aabb(aabb)
            .flat_map(|chunk| chunk.iter().copied())
    }
}

struct NoDuplication;

impl IterHurtboxesOnAabb for NoDuplication {
    fn iter_hurtboxes_on_aabb<'w, 'a, Group: SpatialIndexColliderGroup>(
        index: &'a mut Res<'w, SpatialIndex<Group>>,
        aabb: Aabb2d,
    ) -> impl Iterator<Item = Entity> {
        let mut deduplication_set = EntityHashSet::default();

        index
            .iter_chunks_on_aabb(aabb)
            .flat_map(|chunk| chunk.iter().copied())
            .filter(move |&entity| deduplication_set.insert(entity))
    }
}

pub type SpatialIndexQuery<'w, 's, Group: SpatialIndexColliderGroup> =
    GenericSpatialIndexQuery<'w, 's, Group, NoDuplication>;

pub type SpatialIndexQueryAllowDuplication<'w, 's, Group: SpatialIndexColliderGroup> =
    GenericSpatialIndexQuery<'w, 's, Group, AllowDuplication>;

impl<'w, 's, Group: SpatialIndexColliderGroup, I: IterHurtboxesOnAabb>
    GenericSpatialIndexQuery<'w, 's, Group, I>
{
    fn iter_hurtboxes_on_aabb<'a>(
        &'a mut self,
        aabb: Aabb2d,
        hitbox_param: <Group::Filter as SpatialQueryFilter>::HitboxParam<'a>,
    ) -> impl Iterator<Item = (Collider<'a, Group::Hurtbox>, Entity)> + use<'w, 's, 'a, I, Group>
    {
        let hurtbox_filter = &mut self.hurtbox_filter;

        I::iter_hurtboxes_on_aabb(&mut self.index, aabb)
            .filter_map(|entity| {
                let (shape, registry) = self.hurtboxes.get(entity).ok()?;
                Some((Collider::new(&**shape, registry.current_position()), entity))
            })
            .filter(move |(_, entity)| {
                let hurtbox_param = Group::Filter::hurtbox_filter_param(*entity, hurtbox_filter);
                Group::Filter::filter(hitbox_param, hurtbox_param)
            })
    }
}

impl<Group: SpatialIndexColliderGroup, I: IterHurtboxesOnAabb> SpatialQuery<Group>
    for GenericSpatialIndexQuery<'_, '_, Group, I>
{
    type HurtboxData = Entity;
    fn intersect<'a>(
        &'a mut self,
        hitbox: Collider<'a, <Group as crate::ColliderGroup>::Hitbox>,
        hitbox_param: <<Group as crate::ColliderGroup>::Filter as SpatialQueryFilter>::HitboxParam<
            'a,
        >,
    ) -> impl Iterator<Item = Self::HurtboxData> + 'a {
        let aabb = hitbox.bounding();

        self.iter_hurtboxes_on_aabb(aabb, hitbox_param).filter_map(
            move |(hurtbox, hurtbox_entity)| hitbox.intersect(hurtbox).then_some(hurtbox_entity),
        )
    }

    fn cast<'a>(
        &'a mut self,
        hitbox: Collider<'a, <Group as crate::ColliderGroup>::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
        hitbox_filter: <<Group as crate::ColliderGroup>::Filter as SpatialQueryFilter>::HitboxParam<
            'a,
        >,
    ) -> impl Iterator<Item = (f32, Dir2, Self::HurtboxData)> + 'a {
        let offset = offset_dir * offset_len;

        let aabb1 = hitbox.bounding();
        let aabb2 = Aabb2d {
            min: aabb1.min + offset,
            max: aabb1.max + offset,
        };
        let aabb = aabb1.merge(&aabb2);

        self.iter_hurtboxes_on_aabb(aabb, hitbox_filter)
            .filter_map(move |(other, data)| {
                hitbox
                    .cast(other, offset_dir, offset_len)
                    .map(|(dist, norm)| (dist, norm, data))
            })
    }
}
