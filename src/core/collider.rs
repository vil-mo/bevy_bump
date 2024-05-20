use std::ops::{Deref, DerefMut};
use bevy::prelude::*;

/// A shape against which collisions are checked
/// Should be able to store global position of collider
pub trait Collider: Clone {
    /// Needed access for correct normal calculations
    fn position(&self) -> Vec2;
    
    /// After calling this method, `self.position` should return `to`
    fn set_position(&mut self, to: Vec2);

    /// Normal with the line, which ends are at `from` and `self.position()`
    fn normal(&self, from: Vec2) -> Direction2d;
}

/// Trait allows `Self` to perform collisions with `T`
pub trait ColliderInteraction<T: Collider>: Collider {
    /// Returns true if `self` intersects with `other`
    fn intersect(&self, other: &T) -> bool;
    
    /// If `self` were to move along the `offset`, 
    /// returns distance at which `self.position` will be intersecting `other`  
    fn cast(&self, other: &T, offset: Vec2) -> Option<f32>;
}

impl<T: Collider + ?Sized> Collider for Box<T> {
    fn position(&self) -> Vec2 {
        self.deref().position()
    }
    fn set_position(&mut self, to: Vec2) {
        self.deref_mut().set_position(to)
    }
    fn normal(&self, from: Vec2) -> Direction2d {
        self.deref().normal(from)
    }
}

/// Trait allows for easier to read generic code 
pub trait CollisionGroup: 'static {
    /// Actor that is colliding
    type Actor: ColliderInteraction<Self::Target>;
    /// Bodies that generate collisions and usually stop actor's movement 
    type Target: Collider;
}
