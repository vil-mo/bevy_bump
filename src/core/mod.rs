use collider::ColliderInteraction;

pub mod collider;
pub mod collisions_query;
pub mod response;

/// Trait allows for easier to read generic code
pub trait ColliderGroup: 'static {
    type HurtboxData;

    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + 'static;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: 'static;
}
