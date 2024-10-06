use std::marker::PhantomData;

use bevy::prelude::Component;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct RegisterHurtbox<T>(PhantomData<T>);

impl<T> RegisterHurtbox<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

fn register_hurtbox<T>(
    to_register: Query<
        Entity,
        (
            With<RegisterHurtbox<T>>,
            With<HurtboxShape<T>>,
            With<Transform>,
        ),
    >,
    mut commands: Commands,
) {
    for entity in to_register.iter() {
        commands
            .entity(entity)
            .add(|entity: Entity, world: &mut World| {
                // Until command is appled to the world, user can remove necessary components
                // So we need to check before inserting

                let mut entity_mut = world.entity_mut(entity);
                if !entity_mut.contains::<RegisterHurtbox<T>>() {
                    return;
                }
                entity_mut.remove::<RegisterHurtbox<T>>();

                if entity_mut.contains::<SpacialIndexRegistry<T>>() {
                    return;
                }

                if entity_mut.contains::<HurtboxLayer<T>>()
                    && entity_mut.contains::<HurtboxShape<T>>()
                    && entity_mut.contains::<Transform>()
                {
                    entity_mut.insert(SpacialIndexRegistry::<T>::not_valid());
                }
            });
    }
}
