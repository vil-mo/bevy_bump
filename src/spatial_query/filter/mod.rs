use bevy::{ecs::system::SystemParam, prelude::Entity, utils::all_tuples};

pub mod monitorable;

pub trait Filter {
    type FilterParam;

    fn filter(param: Self::FilterParam) -> bool;
}

pub trait SystemFilter: Filter {
    type SystemParam: SystemParam;
    fn get_filter_param(entity: Entity, system_param: &mut Self::SystemParam) -> Self::FilterParam;

    fn system_filter(entity: Entity, system_param: &mut Self::SystemParam) -> bool {
        Self::filter(Self::get_filter_param(entity, system_param))
    }
}

macro_rules! impl_filter {
    ($(($t:ident, $p:ident)),*) => {
        impl<$($t: Filter),*> Filter for ($($t,)*) {
            type FilterParam = ($($t::FilterParam,)*);

            fn filter(param: Self::FilterParam) -> bool {
                let ($($p,)*) = param;
                true $(&& <$t>::filter($p))*
            }
        }

        impl<$($t: SystemFilter),*> SystemFilter for ($($t,)*) {
            type SystemParam = ($($t::SystemParam,)*);

            fn get_filter_param(entity: Entity, system_param: &mut Self::SystemParam) -> Self::FilterParam {
                let ($($p,)*) = system_param;
                ($(<$t>::get_filter_param(entity, $p),)*)
            }
        }
    };
}

all_tuples!(impl_filter, 0, 8, T, p);
