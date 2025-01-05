use crate::ColliderGroup;
use bevy::{ecs::system::{ReadOnlySystemParam, SystemParamItem}, prelude::Entity};

pub mod layer;
pub mod monitorable;
pub mod monitoring;

pub trait SpatialQueryFilter: Send + Sync + 'static {
    type HitboxParam<'a>: Copy;
    type HurtboxParam<'a>: Copy;

    fn filter(
        hitbox_data: Self::HitboxParam<'_>,
        hurtbox_data: Self::HurtboxParam<'_>,
    ) -> bool;
}

pub trait SystemSpatialQueryFilter<Group>: SpatialQueryFilter {
    type HitboxSystemParam: ReadOnlySystemParam;
    type HurtboxSystemParam: ReadOnlySystemParam;

    fn hitbox_filter_param<'a>(
        hitbox: Entity,
        system_param: &'a mut SystemParamItem<Self::HitboxSystemParam>
    ) -> Self::HitboxParam<'a>;
    fn hurtbox_filter_param<'a>(
        hurtbox: Entity,
        system_param: &'a mut SystemParamItem<Self::HurtboxSystemParam>,
    ) -> Self::HurtboxParam<'a>;
}

pub type HitboxFilterSystemParam<Group> =
    <<Group as ColliderGroup>::Filter as SystemSpatialQueryFilter<Group>>::HitboxSystemParam;
pub type HurtboxFilterSystemParam<Group> =
    <<Group as ColliderGroup>::Filter as SystemSpatialQueryFilter<Group>>::HurtboxSystemParam;

// TODO
// macro_rules! impl_filter {
//     ($(($t:ident, $p:ident)),*) => {
//         impl<$($t: Filter),*> Filter for ($($t,)*) {
//             type FilterParam = ($($t::FilterParam,)*);

//             fn filter(param: Self::FilterParam) -> bool {
//                 let ($($p,)*) = param;
//                 true $(&& <$t>::filter($p))*
//             }
//         }

//         impl<$($t: SystemFilter),*> SystemFilter for ($($t,)*) {
//             type SystemParam = ($($t::SystemParam,)*);

//             fn get_filter_param(entity: Entity, system_param: &mut Self::SystemParam) -> Self::FilterParam {
//                 let ($($p,)*) = system_param;
//                 ($(<$t>::get_filter_param(entity, $p),)*)
//             }
//         }
//     };
// }

// all_tuples!(impl_filter, 0, 8, T, p);
