use crate::{
    core::{
        collider::{Collider, ColliderInteraction},
        response::Pass,
    },
    ecs_core::{
        collision_check::CollisionCheck,
        components::{HitboxLayer, HitboxMonitoring, HitboxShape},
        layer::CollisionLayer,
        LayerGroup,
    },
    utils::Bounded,
};
use bevy::{math::bounding::Aabb2d, prelude::*};

pub trait ScannerGroup: CollisionLayer + Send + Sync + 'static {
    type Hitbox: ColliderInteraction<Self::Hurtbox> + Bounded<Aabb2d> + Send + Sync + 'static;
    type Hurtbox: Bounded<Aabb2d> + Send + Sync + 'static;
}

impl<T: ScannerGroup> LayerGroup for T {
    type Hitbox = T::Hitbox;
    type Hurtbox = T::Hurtbox;
}

pub(super) fn register_scanner_group<T: ScannerGroup>(app: &mut App) {}

#[derive(Component)]
pub struct ScannerHitboxLastPosition<Layer: ScannerGroup>(Vec2, std::marker::PhantomData<Layer>);

fn collide_scanner_group<T: ScannerGroup>(
    collision_check: CollisionCheck<T>,
    mut query: Query<(
        Entity,
        &mut ScannerHitboxLastPosition<T>,
        &HitboxShape<T>,
        &HitboxLayer<T>,
        Option<&HitboxMonitoring<T>>,
    )>,
    transform_helper: TransformHelper,
) {
    for (entity, mut last_position, shape, layer, monitoring) in query.iter_mut() {
        let Ok(new_position) = transform_helper.compute_global_transform(entity) else {
            warn!("Unable to compute global position of registered scanner of {entity}. Skipping scanner update.");
            continue;
        };
        let new_position = new_position.translation().xy();
        let position_change = last_position.0 - new_position;

        if monitoring.copied().unwrap_or_default().0 {
            let hitbox = Collider::new(&**shape, last_position.0);
            collision_check.check_movement(hitbox, position_change, &layer.0, &mut Pass);
        }

        last_position.0 = new_position;
    }
}

//TODO: do collision report strategy