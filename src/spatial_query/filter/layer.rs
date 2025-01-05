use super::{SpatialQueryFilter, SystemSpatialQueryFilter};
use crate::{ColliderGroup, CollisionImplementation};
use bevy::{ecs::system::SystemParam, prelude::*};
#[cfg(feature = "enumset_layer")]
use enumset::{EnumSet, EnumSetType};

pub trait LayeredImplementation<Group: ColliderGroup<Implementation = Self>>:
    CollisionImplementation<Group>
{
    type Layer: CollisionLayer + Send + Sync + 'static;
}

pub trait LayeredColliderGroup: ColliderGroup<Implementation: LayeredImplementation<Self>> {
    type Layer: CollisionLayer + Send + Sync + 'static;
}

impl<Group: ColliderGroup<Implementation: LayeredImplementation<Group>>> LayeredColliderGroup
    for Group
{
    type Layer = <Group::Implementation as LayeredImplementation<Group>>::Layer;
}

pub struct Layer<L: CollisionLayer + Send + Sync + 'static>(pub L);

impl<L: CollisionLayer + Send + Sync + 'static> SpatialQueryFilter for Layer<L> {
    type HitboxParam<'a> = &'a L;
    type HurtboxParam<'a> = &'a L;

    fn filter(
        hitbox_data: Self::HitboxParam<'_>,
        hurtbox_data: Self::HurtboxParam<'_>,
    ) -> bool {
        !hitbox_data.collides(hurtbox_data)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct HitboxLayer<Group: LayeredColliderGroup>(pub Group::Layer);
#[derive(Component, Deref, DerefMut)]
pub struct HurtboxLayer<Group: LayeredColliderGroup>(pub Group::Layer);

impl<Group: LayeredColliderGroup<Layer = L>, L: CollisionLayer + Send + Sync + 'static>
    SystemSpatialQueryFilter<Group> for Layer<L>
{
    type HitboxSystemParam = Query<'static, 'static, &'static HitboxLayer<Group>>;
    type HurtboxSystemParam = Query<'static, 'static, &'static HurtboxLayer<Group>>;

    fn hitbox_filter_param<'a>(
        hitbox: Entity,
        system_param: &'a mut <Self::HitboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> Self::HitboxParam<'a> {
        system_param.get(hitbox).expect(
            "Implementation must guaratee that `hitbox_filter_param` only called for entities that pass filter `SystemSpatialQueryFilter::HitboxQueryFilter`"
        )
    }

    fn hurtbox_filter_param<'a>(
        hurtbox: Entity,
        system_param: &'a mut <Self::HurtboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> Self::HurtboxParam<'a> {
        system_param.get(hurtbox).expect(
            "Implementation must guaratee that `hurtbox_filter_param` only called for entities that pass filter `SystemSpatialQueryFilter::HurtboxQueryFilter`"
        )
    }
}

pub trait CollisionLayer {
    fn collides(&self, other: &Self) -> bool;
}

#[cfg(feature = "enumset_layer")]
impl<T: EnumSetType> CollisionLayer for EnumSet<T> {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        !self.is_disjoint(*other)
    }
}

impl CollisionLayer for () {
    #[inline(always)]
    fn collides(&self, _: &Self) -> bool {
        true
    }
}

impl CollisionLayer for u8 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u16 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u32 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u64 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for u128 {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}

impl CollisionLayer for usize {
    #[inline(always)]
    fn collides(&self, other: &Self) -> bool {
        self & other != 0
    }
}
