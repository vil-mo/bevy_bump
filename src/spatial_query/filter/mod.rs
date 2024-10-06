use bevy::{ecs::{query::QueryFilter, system::SystemParam}, prelude::Entity};
use crate::ColliderGroup;

pub mod monitorable;
pub mod monitoring;
pub mod layer;

pub trait SpatialQueryFilter: Send + Sync + 'static {
    type HitboxFilterParam<'a>;
    type HurtboxFilterParam<'a>;

    fn filter(hitbox_data: Self::HitboxFilterParam<'_>, hurtbox_data: Self::HurtboxFilterParam<'_>)
        -> bool;
}

pub trait SystemSpatialQueryFilter<Group: ColliderGroup>: SpatialQueryFilter {
    type HitboxSystemParam: SystemParam;
    type HitboxQueryFilter: QueryFilter;

    type HurtboxSystemParam: SystemParam;
    type HurtboxQueryFilter: QueryFilter;

    fn hitbox_filter_param<'a>(
        hitbox: Entity,
        system_param: &'a mut <Self::HitboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> Self::HitboxFilterParam<'a>;
    fn hurtbox_filter_param<'a>(
        hurtbox: Entity,
        system_param: &'a mut <Self::HurtboxSystemParam as SystemParam>::Item<'_, '_>,
    ) -> Self::HurtboxFilterParam<'a>;
}

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
