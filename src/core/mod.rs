use crate::core::collider::ColliderInteraction;

pub mod broad_phase;
pub mod collider;
pub mod response;

/// Trait allows for easier to read generic code
pub trait ColliderGroup: 'static {
    type CollisionData;

    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox> + 'static;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: 'static;
}
