pub mod broad_phase;
pub mod collider;
pub mod response;

use collider::{Collider, ColliderInteraction};

/// Trait allows for easier to read generic code
pub trait ColliderGroup {
    /// Actor that is colliding
    type Hitbox: ColliderInteraction<Self::Hurtbox>;
    /// Bodies that generate collisions and usually stop actor's movement
    type Hurtbox: Collider;
}
