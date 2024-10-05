
/// Wether or not the hitbox should be disabled.
/// If `false`, the collisions with the hurtbox will be ignored.
#[derive(Component, Deref)]
pub struct HitboxMonitoring<Group: ColliderGroup>(#[deref] pub bool, PhantomData<Group>);

impl<Group: ColliderGroup> Default for HitboxMonitoring<Group> {
    #[inline]
    fn default() -> Self {
        Self::new(true)
    }
}

impl<Group: ColliderGroup> HitboxMonitoring<Group> {
    #[inline]
    pub fn new(monitoring: bool) -> Self {
        Self(monitoring, PhantomData)
    }
}

impl<Group: ColliderGroup> Clone for HitboxMonitoring<Group> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<Group: ColliderGroup> Copy for HitboxMonitoring<Group> {}
