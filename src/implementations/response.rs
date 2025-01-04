use crate::{collider::{Collider, ColliderInteraction}, spatial_query::SpatialQuery};
use bevy::math::{Dir2, Vec2};

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug)]
pub struct ResponseCollisionInformation<T: SpatialQuery> {
    /// The point on the desired path (or on the path corrected by solver) at wich collision was detected
    /// Should make sense for it to be [`Collider::position`] of actor that performed movement
    pub global_position: Vec2,
    /// Result of [`Collider::normal`] of body against which collision was detected
    pub normal: Dir2,
    pub data: T::HurtboxData,
}

impl<T: SpatialQuery> Clone for ResponseCollisionInformation<T>
where
    T::HurtboxData: Clone,
{
    fn clone(&self) -> Self {
        Self {
            global_position: self.global_position,
            normal: self.normal,
            data: self.data.clone(),
        }
    }
}

impl<T: SpatialQuery> Copy for ResponseCollisionInformation<T> where T::HurtboxData: Copy {}

impl<T: SpatialQuery> ResponseCollisionInformation<T> {
    fn from_cast(
        position: Vec2,
        direction: Dir2,
    ) -> impl FnMut((f32, Dir2, T::HurtboxData)) -> Self {
        move |(dist, normal, data)| Self {
            global_position: position + direction * dist,
            normal,
            data,
        }
    }
}

pub trait RunningResponse<T: SpatialQuery>: Sized {
    type AfterOutput: Iterator<Item = ResponseCollisionInformation<T>>;

    fn next(self) -> RunningResponseVariant<Self, T>;

    fn into_iter(self, buf: &mut Vec2) -> ResponseIterator<Self, T> {
        ResponseIterator {
            buf,
            current_iter: ResponseIteratorVariant::BeforeOutput(self),
        }
    }

    fn ignore_resulting_offset(self) -> IgnoreResultingOffsetIterator<Self, T> {
        IgnoreResultingOffsetIterator {
            curent_iter: ResponseIteratorVariant::BeforeOutput(self),
        }
    }

    fn until_resulting_offset(
        mut self,
        mut f: impl FnMut(ResponseCollisionInformation<T>),
    ) -> (Vec2, Self::AfterOutput) {
        use RunningResponseVariant::*;

        loop {
            match self.next() {
                Collision(item, next_iter) => {
                    self = next_iter;
                    f(item);
                }
                ResultingOffset(output, next_iter) => return (output, next_iter),
            }
        }
    }

    fn foreach(self, f: impl FnMut(ResponseCollisionInformation<T>)) -> Vec2 {
        let mut buf = Vec2::ZERO;
        self.into_iter(&mut buf).for_each(f);
        buf
    }
}

pub enum RunningResponseVariant<T: RunningResponse<Group>, Group: ColliderGroup> {
    Collision(ResponseCollisionInformation<Group>, T),
    ResultingOffset(Vec2, T::AfterOutput),
}

enum ResponseIteratorVariant<T: RunningResponse<Group>, Group: ColliderGroup> {
    BeforeOutput(T),
    AfterOutput(T::AfterOutput),
}

pub struct ResponseIterator<'a, T: RunningResponse<Group>, Group: ColliderGroup> {
    buf: &'a mut Vec2,
    current_iter: ResponseIteratorVariant<T, Group>,
}

impl<'a, T: RunningResponse<Group>, Group: ColliderGroup> Iterator
    for ResponseIterator<'a, T, Group>
{
    type Item = ResponseCollisionInformation<Group>;

    fn next(&mut self) -> Option<Self::Item> {
        use ResponseIteratorVariant::*;
        use RunningResponseVariant::*;

        replace_with::replace_with_or_abort_and_return(&mut self.current_iter, |current_iter| {
            match current_iter {
                BeforeOutput(iter) => match iter.next() {
                    Collision(item, next_iter) => (Some(item), BeforeOutput(next_iter)),
                    ResultingOffset(value, mut next_iter) => {
                        *self.buf = value;

                        (next_iter.next(), AfterOutput(next_iter))
                    }
                },

                AfterOutput(mut iter) => (iter.next(), AfterOutput(iter)),
            }
        })
    }
}

pub struct IgnoreResultingOffsetIterator<T: RunningResponse<Group>, Group: ColliderGroup> {
    curent_iter: ResponseIteratorVariant<T, Group>,
}

impl<T: RunningResponse<Group>, Group: ColliderGroup> Iterator
    for IgnoreResultingOffsetIterator<T, Group>
{
    type Item = ResponseCollisionInformation<Group>;

    fn next(&mut self) -> Option<Self::Item> {
        use ResponseIteratorVariant::*;
        use RunningResponseVariant::*;

        replace_with::replace_with_or_abort_and_return(&mut self.curent_iter, |current_iter| {
            match current_iter {
                BeforeOutput(iter) => match iter.next() {
                    Collision(item, next_iter) => (Some(item), BeforeOutput(next_iter)),
                    ResultingOffset(_, mut next_iter) => (next_iter.next(), AfterOutput(next_iter)),
                },

                AfterOutput(mut iter) => (iter.next(), AfterOutput(iter)),
            }
        })
    }
}

#[inline(always)]
fn empty<Group: ColliderGroup>() -> std::iter::Empty<ResponseCollisionInformation<Group>> {
    std::iter::empty()
}

pub struct ImmediateResultingOffset<
    Collisions: Iterator<Item = ResponseCollisionInformation<Group>>,
    Group: ColliderGroup,
> {
    offset: Vec2,
    collisions: Collisions,
}

impl<Collisions: Iterator<Item = ResponseCollisionInformation<Group>>, Group: ColliderGroup>
    ImmediateResultingOffset<Collisions, Group>
{
    fn new(offset: Vec2, collisions: Collisions) -> Self {
        ImmediateResultingOffset { offset, collisions }
    }
}

impl<Collisions: Iterator<Item = ResponseCollisionInformation<Group>>, Group: ColliderGroup>
    RunningResponse<Group> for ImmediateResultingOffset<Collisions, Group>
{
    type AfterOutput = Collisions;

    fn next(self) -> RunningResponseVariant<Self, Group> {
        RunningResponseVariant::ResultingOffset(self.offset, self.collisions)
    }
}

pub struct LazyResponse<
    F: FnOnce() -> (Vec2, Collisions),
    Group: ColliderGroup,
    Collisions: Iterator<Item = ResponseCollisionInformation<Group>>,
>(pub F);

impl<
        F: FnOnce() -> (Vec2, Collisions),
        Group: ColliderGroup,
        Collisions: Iterator<Item = ResponseCollisionInformation<Group>>,
    > RunningResponse<Group> for LazyResponse<F, Group, Collisions>
{
    type AfterOutput = Collisions;

    fn next(self) -> RunningResponseVariant<Self, Group> {
        let (offset, collisions) = self.0();
        RunningResponseVariant::ResultingOffset(offset, collisions)
    }
}

/// Solver defines how actor will react to met colliders.
/// When actor meets collider it refers to `CollisionResponse`.
///
/// `colliders` is a broad phase
/// `actor` is actor that performs the collision
///
/// `offset` is offset that `actor` desires to move this call
///
/// Returns actual offset that actor should move from the point at with collision was detected
/// and information about all the collisions that happened
pub trait CollisionResponse {
    fn respond<'a, Hitbox: ColliderInteraction<SQ::Hurtbox>, SQ: SpatialQuery + 'a>(
        &'a mut self,
        collisions: SQ,
        hitbox: Collider<'a, Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<SQ> + 'a;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ignore;

impl CollisionResponse for Ignore {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        _collisions: Collisions,
        _hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        ImmediateResultingOffset::new(offset_dir * offset_len, empty())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pass;

impl CollisionResponse for Pass {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        ImmediateResultingOffset::new(
            offset_dir * offset_len,
            collisions.cast(hitbox, offset_dir, offset_len).map(
                ResponseCollisionInformation::from_cast(hitbox.position, offset_dir),
            ),
        )
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Touch;

impl CollisionResponse for Touch {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        let mut min_distance = offset_len;
        let mut resulting_collision = None;

        LazyResponse(move || {
            for collision in collisions.cast(hitbox, offset_dir, offset_len) {
                if min_distance >= collision.0 {
                    min_distance = collision.0;
                    resulting_collision = Some(ResponseCollisionInformation::from_cast(
                        hitbox.position,
                        offset_dir,
                    )(collision));
                }
            }

            (offset_dir * min_distance, resulting_collision.into_iter())
        })
    }
}

pub fn trajectory_change_on_touch<
    'a,
    F: FnMut(Vec2, Dir2) -> Vec2,
    Group: ColliderGroup,
    Collisions: SpatialQuery<Group>,
>(
    collisions: Collisions,
    mut hitbox: Collider<'a, Group::Hitbox>,
    offset_dir: Dir2,
    offset_len: f32,
    mut trajectory_change: F,
) -> (
    Vec2,
    std::vec::IntoIter<ResponseCollisionInformation<Group>>,
) {
    // Vector with all collisions
    let mut res_vec = Vec::new();

    // We want to move that distance
    let mut desired_offset = offset_dir * offset_len;

    // We actually moved that distance
    let mut actual_offset = Vec2::ZERO;
    // Moving that distance, checking if we collide
    let mut opt_collision_information = Touch
        .respond(collisions, hitbox, offset_dir, offset_len)
        .into_iter(&mut actual_offset)
        .next();

    // While we collide
    while let Some(collision_information) = opt_collision_information {
        let normal = collision_information.normal;
        // Register the fact we collided
        res_vec.push(collision_information);

        // Move hitbox that distance
        hitbox.position += actual_offset;

        // Trajectory change takes difference between desired and actual offset, and normal of the collision
        let diff_offset = desired_offset - actual_offset;
        desired_offset = (trajectory_change)(diff_offset, normal);

        // If desired offset is zero, we are done
        let Ok((desired_dir, desired_len)) = Dir2::new_and_length(desired_offset) else {
            break;
        };

        // If not zero, check if colliding agin, with once again setting actual offset
        opt_collision_information = Touch
            .respond(collisions, hitbox, desired_dir, desired_len)
            .into_iter(&mut actual_offset)
            .next();
    }

    (actual_offset, res_vec.into_iter())
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Slide;

impl CollisionResponse for Slide {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        LazyResponse(move || {
            trajectory_change_on_touch(
                collisions,
                hitbox,
                offset_dir,
                offset_len,
                |left_movement, normal| left_movement.project_onto_normalized(normal.perp()),
            )
        })
    }
}

fn bounce(left_movement: Vec2, normal: Dir2) -> Vec2 {
    left_movement - 2.0 * left_movement.project_onto_normalized(*normal)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounce;

impl CollisionResponse for Bounce {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        LazyResponse(move || {
            trajectory_change_on_touch(collisions, hitbox, offset_dir, offset_len, bounce)
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimitedBounce<NextResponse: CollisionResponse> {
    pub bounces: u32,
    pub next_response: NextResponse,
}

impl<NextResponse: CollisionResponse> LimitedBounce<NextResponse> {
    #[inline]
    pub fn new(bounces: u32, next_response: NextResponse) -> Self {
        Self {
            bounces,
            next_response,
        }
    }
}

impl CollisionResponse for LimitedBounce<Ignore> {
    fn respond<'a, Group: ColliderGroup, Collisions: SpatialQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        LazyResponse(move || {
            let mut outer_left_movement = Vec2::ZERO;
            let (offset, collisions) = trajectory_change_on_touch(
                collisions,
                hitbox,
                offset_dir,
                offset_len,
                |left_movement, normal| {
                    if self.bounces == 0 {
                        outer_left_movement = left_movement;
                        return Vec2::ZERO;
                    }
                    self.bounces -= 1;
                    bounce(left_movement, normal)
                },
            );

            (offset + outer_left_movement, collisions)
        })
    }
}
