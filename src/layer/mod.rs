pub trait LayerGroup:
    ColliderGroup<
        HurtboxData = Entity,
        Hitbox: Bounded<Aabb2d>,
        Hurtbox: Bounded<Aabb2d>,
    >
{
    type Layer: CollisionLayer + Send + Sync + 'static;
}
