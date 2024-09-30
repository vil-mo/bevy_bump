use crate::{
    core::{
        collider::Collider,
        response::{Pass, RunningResponse},
    },
    ecs_core::{
        collision_check::CollisionCheck,
        components::{HitboxLayer, HitboxMonitoring, HitboxShape},
        CollisionDetectionSet, LayerGroup,
    },
    layer_types::collision_report_strategy::CollisionInformation,
};
use bevy::{ecs::system::StaticSystemParam, prelude::*};

use super::collision_report_strategy::CollisionReportStrategy;

pub trait ScannerGroup: LayerGroup {
    type ReportStrategy: CollisionReportStrategy<Self>;
}

pub(super) fn register_scanner_group<T: ScannerGroup>(app: &mut App) {
    T::ReportStrategy::register(app);

    app.add_systems(
        super::COLLISION_DETECTION_SCHEDULE,
        collide_scanner_group::<T>.in_set(CollisionDetectionSet::Colliding),
    );

    app.observe(add_scanner_last_position::<T>)
        .observe(remove_scanner_last_position::<T>);
}

#[derive(Component)]
struct ScannerHitboxLastPosition<Layer: ScannerGroup>(Vec2, std::marker::PhantomData<Layer>);

fn add_scanner_last_position<Layer: ScannerGroup>(
    trigger: Trigger<OnAdd, HitboxLayer<Layer>>,
    transform_helper: TransformHelper,
    mut commands: Commands,
) {
    let position = transform_helper
        .compute_global_transform(trigger.entity())
        .map(|global_transform|global_transform.translation().xy())
        .unwrap_or_else(|_| {
            warn!("Unable to compute global position of registered scanner of {}. Setting scanner position to (0, 0).", trigger.entity());
            Vec2::ZERO
        });

    commands
        .entity(trigger.entity())
        .insert(ScannerHitboxLastPosition::<Layer>(
            position,
            std::marker::PhantomData,
        ));
}

fn remove_scanner_last_position<Layer: ScannerGroup>(
    trigger: Trigger<OnRemove, HitboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .remove::<ScannerHitboxLastPosition<Layer>>();
}

fn collide_scanner_group<T: ScannerGroup>(
    collision_check: CollisionCheck<T>,
    mut hitboxes: Query<(
        Entity,
        &mut ScannerHitboxLastPosition<T>,
        &HitboxShape<T>,
        &HitboxLayer<T>,
        Option<&HitboxMonitoring<T>>,
    )>,
    transform_helper: TransformHelper,

    mut report_param: StaticSystemParam<<T::ReportStrategy as CollisionReportStrategy<T>>::Param>,
) {
    for (hitbox_entity, mut last_position, shape, layer, monitoring) in hitboxes.iter_mut() {
        let Ok(new_position) = transform_helper.compute_global_transform(hitbox_entity) else {
            warn!("Unable to compute global position of registered scanner of {hitbox_entity}. Skipping scanner update.");
            continue;
        };
        let new_position = new_position.translation().xy();
        let position_change = last_position.0 - new_position;

        use iter_n::iter2::*;
        let mut pass = Pass;

        let collisions = if monitoring.copied().unwrap_or_default().0 {
            let hitbox = Collider::new(&**shape, last_position.0);

            if let Ok((offset_dir, offset_len)) = Dir2::new_and_length(position_change) {
                collision_check
                    .check_movement(hitbox, offset_dir, offset_len, &layer.0, &mut pass)
                    .ignore_resulting_offset()
                    .map(|x| {
                        CollisionInformation::from_response::<T>(
                            hitbox_entity,
                            x,
                        )
                    })
                    .into_iter0()
            } else {
                collision_check
                    .check_intersection(hitbox, &layer.0)
                    .map(|hurtbox| CollisionInformation {
                        hitbox: hitbox_entity,
                        global_position: new_position,
                        hurtbox,
                        normal: None,
                    })
                    .into_iter1()
            }
            .into_iter0()
        } else {
            std::iter::empty::<CollisionInformation>().into_iter1()
        };

        T::ReportStrategy::report_collisions(collisions, &mut report_param);

        last_position.0 = new_position;
    }
}
