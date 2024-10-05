use bevy::{ecs::system::SystemParam, prelude::*, utils::all_tuples};

use crate::{
    core::{response::ResponseCollisionInformation, ColliderGroup},
    ecs_core::{components::{HitboxLayer, HurtboxLayer}, LayerGroup},
};

#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation {
    pub hitbox: Entity,
    pub hurtbox: Entity,
    pub global_position: Vec2,
    pub normal: Option<Dir2>,
}

impl CollisionInformation {
    pub fn from_response<Group: ColliderGroup>(
        hitbox: Entity,
        response: ResponseCollisionInformation<Group>,
    ) -> Self
    where
        Group::HurtboxData: Into<Entity>,
    {
        Self {
            hitbox,
            hurtbox: response.data.into(),
            global_position: response.global_position,
            normal: Some(response.normal),
        }
    }
}

// TODO: Document about system state
pub trait CollisionReportStrategy<Layer: LayerGroup> {
    type Param: SystemParam;

    fn register(app: &mut App);

    fn report_collisions(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    );
}

macro_rules! impl_collision_report_strategy {
    ($(($t:ident, $p:ident)),*) => {
        impl<Layer: LayerGroup, $($t: CollisionReportStrategy<Layer>),*> CollisionReportStrategy<Layer> for ($($t,)*)
        {
            type Param = ParamSet<'static, 'static, ($($t::Param,)*)>;

            fn register(app: &mut App) {
                $($t::register(app);)*
            }

            fn report_collisions(
                collisions: impl Iterator<Item = CollisionInformation>,
                param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
            ) {
                for collision in collisions {
                    $($t::report_collisions(std::iter::once(collision), &mut param.$p());)*
                }
            }
        }
    };
}

all_tuples!(impl_collision_report_strategy, 1, 8, T, p);

#[derive(Event)]
pub struct Collided(pub CollisionInformation);

pub struct SendCollisionEvent;

impl<Layer: LayerGroup> CollisionReportStrategy<Layer> for SendCollisionEvent {
    type Param = EventWriter<'static, Collided>;

    fn register(app: &mut App) {
        app.add_event::<Collided>();
    }

    fn report_collisions(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            param.send(Collided(collision));
        }
    }
}

pub struct ObserveCollision;

#[derive(Event)]
pub struct HitboxCollided(pub CollisionInformation);
#[derive(Event)]
pub struct HurtboxCollided(pub CollisionInformation);

impl<Layer: LayerGroup> CollisionReportStrategy<Layer> for ObserveCollision {
    type Param = Commands<'static, 'static>;

    fn register(_app: &mut App) {}

    fn report_collisions(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            param.trigger(Collided(collision));
            param.trigger_targets(Collided(collision), [collision.hitbox, collision.hurtbox]);
            param.trigger_targets(HitboxCollided(collision), collision.hitbox);
            param.trigger_targets(HurtboxCollided(collision), collision.hurtbox);
        }
    }
}

#[derive(Component)]
pub struct HitboxCollisions<Layer: LayerGroup>(
    pub Vec<CollisionInformation>,
    std::marker::PhantomData<Layer>,
);

impl<Layer: LayerGroup> Default for HitboxCollisions<Layer> {
    fn default() -> Self {
        Self(Vec::new(), std::marker::PhantomData)
    }
}

fn add_hitbox_collisions<Layer: LayerGroup>(
    trigger: Trigger<OnAdd, HitboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .insert(HitboxCollisions::<Layer>::default());
}

fn remove_hitbox_collisions<Layer: LayerGroup>(
    trigger: Trigger<OnRemove, HitboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .remove::<HitboxCollisions<Layer>>();
}

pub struct ExtendHitboxComponent;

impl<Layer: LayerGroup> CollisionReportStrategy<Layer> for ExtendHitboxComponent {
    type Param = Query<'static, 'static, &'static mut HitboxCollisions<Layer>>;

    fn register(app: &mut App) {
        app.observe(add_hitbox_collisions::<Layer>);
        app.observe(remove_hitbox_collisions::<Layer>);
    }

    fn report_collisions(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            let Ok(mut component) = param.get_mut(collision.hitbox) else {
                return;
            };

            component.0.push(collision);
        }
    }
}

#[derive(Component)]
pub struct HurtboxCollisions<Layer: LayerGroup>(
    pub Vec<CollisionInformation>,
    std::marker::PhantomData<Layer>,
);

impl<Layer: LayerGroup> Default for HurtboxCollisions<Layer> {
    fn default() -> Self {
        Self(Vec::new(), std::marker::PhantomData)
    }
}

fn add_hurtbox_collisions<Layer: LayerGroup>(
    trigger: Trigger<OnAdd, HurtboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .insert(HurtboxCollisions::<Layer>::default());
}

fn remove_hurtbox_collisions<Layer: LayerGroup>(
    trigger: Trigger<OnRemove, HurtboxLayer<Layer>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.entity())
        .remove::<HurtboxCollisions<Layer>>();
}

pub struct ExtendHurtboxComponent;

impl<Layer: LayerGroup> CollisionReportStrategy<Layer> for ExtendHurtboxComponent {
    type Param = Query<'static, 'static, &'static mut HurtboxCollisions<Layer>>;

    fn register(app: &mut App) {
        app.observe(add_hurtbox_collisions::<Layer>);
        app.observe(remove_hurtbox_collisions::<Layer>);
    }

    fn report_collisions(
        collisions: impl Iterator<Item = CollisionInformation>,
        param: &mut <Self::Param as SystemParam>::Item<'_, '_>,
    ) {
        for collision in collisions {
            let Ok(mut component) = param.get_mut(collision.hurtbox) else {
                return;
            };

            component.0.push(collision);
        }
    }
}
