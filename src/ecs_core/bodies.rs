use crate::core::collider::Collider;
use crate::ecs_core::LayerGroup;
use crate::utils::Bounded;
use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;
use std::marker::PhantomData;

fn register_hurtbox<Layer: LayerGroup>(app: &mut App) {
    app.add_systems(FixedPostUpdate, update_hurtbox_aabb::<Layer>);
}

#[derive(Reflect, Component, Copy, Clone, Default, PartialEq, Debug, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Hitbox<Layer: LayerGroup> {
    pub collider: Layer::Hitbox,
    pub layer: Layer,
    pub response: <Layer as LayerGroup>::Response,
    pub monitoring: bool,
}

impl<Layer: LayerGroup + Default> Default for Hitbox<Layer>
where
    <Layer as LayerGroup>::Hitbox: Bounded<Aabb2d> + Send + Sync + Default,
    <Layer as LayerGroup>::Response: Bounded<Aabb2d> + Send + Sync + Default,
{
    fn default() -> Self {
        Hitbox {
            collider: Default::default(),
            layer: Default::default(),
            response: Default::default(),
            monitoring: true,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct Hurtbox<Layer: LayerGroup> {
    pub collider: Layer::Hurtbox,
    pub layer: Layer,
    pub monitorable: bool,
}

#[derive(Component, Debug, Clone)]
pub(crate) struct HurtboxAabb<Layer: LayerGroup> {
    pub(crate) current_aabb: Aabb2d,
    pub(crate) last_aabb: Aabb2d,
    pub(crate) pd: PhantomData<Layer>,
}

fn update_hurtbox_aabb<Layer: LayerGroup>(
    mut changed_hurtboxes: Query<
        (&mut HurtboxAabb<Layer>, &Hurtbox<Layer>, &GlobalTransform),
        Or<(Changed<Hurtbox<Layer>>, Changed<GlobalTransform>)>,
    >,
) {
    for (mut aabb, hurtbox, transform) in changed_hurtboxes.iter_mut() {
        let mut collider = hurtbox.collider.clone();
        collider.set_position(collider.position() + transform.translation().xy());
        aabb.last_aabb = aabb.current_aabb;
        aabb.current_aabb = collider.bounding();
    }
}
