use bevy::math::{Dir2, Vec2};

use super::{collider::Collider, collisions_query::CollisionsQuery, ColliderGroup};

/// Collisions are accurate up to the DELTA distance
const DELTA: f32 = 0.0001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug)]
pub struct ResponseCollisionInformation<Group: ColliderGroup> {
    /// The point on the desired path (or on the path corrected by solver) at wich collision was detected
    /// Should make sense for it to be [`Collider::position`] of actor that performed movement
    pub global_position: Vec2,
    /// Result of [`Collider::normal`] of body against which collision was detected
    pub normal: Dir2,
    pub data: Group::HurtboxData,
}

impl<Group: ColliderGroup> Clone for ResponseCollisionInformation<Group>
where
    Group::HurtboxData: Clone,
{
    fn clone(&self) -> Self {
        Self {
            global_position: self.global_position,
            normal: self.normal,
            data: self.data.clone(),
        }
    }
}

impl<Group: ColliderGroup> Copy for ResponseCollisionInformation<Group> where
    Group::HurtboxData: Copy
{
}

pub enum RunningResponseVariant<T: RunningResponse<Group>, Group: ColliderGroup> {
    Collision(ResponseCollisionInformation<Group>, T),
    ResultingOffset(Vec2, T::AfterOutput),
}

pub trait RunningResponse<Group: ColliderGroup>: Sized {
    type AfterOutput: Iterator<Item = ResponseCollisionInformation<Group>>;

    fn next(self) -> RunningResponseVariant<Self, Group>;

    fn into_iter(self, buf: &mut Vec2) -> ResponseIterator<Self, Group> {
        ResponseIterator {
            buf,
            current_iter: ResponseIteratorVariant::BeforeOutput(self),
        }
    }

    fn ignore_resulting_offset(self) -> IgnoreResultingOffsetIterator<Self, Group> {
        IgnoreResultingOffsetIterator {
            curent_iter: ResponseIteratorVariant::BeforeOutput(self),
        }
    }

    fn until_resulting_offset(
        mut self,
        mut f: impl FnMut(ResponseCollisionInformation<Group>),
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

    fn foreach(self, f: impl FnMut(ResponseCollisionInformation<Group>)) -> Vec2 {
        let mut buf = Vec2::ZERO;
        self.into_iter(&mut buf).for_each(f);
        buf
    }
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

struct ImmediateResultingOffset<
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

impl<
        'a,
        Collisions: Iterator<Item = ResponseCollisionInformation<Group>>,
        Group: ColliderGroup,
    > RunningResponse<Group> for ImmediateResultingOffset<Collisions, Group>
{
    type AfterOutput = Collisions;

    fn next(self) -> RunningResponseVariant<Self, Group> {
        RunningResponseVariant::ResultingOffset(self.offset, self.collisions)
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
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ignore;

impl CollisionResponse for Ignore {
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group> + 'a>(
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
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group> + 'a>(
        &'a mut self,
        collisions: Collisions,
        hitbox: Collider<'a, Group::Hitbox>,
        offset_dir: Dir2,
        offset_len: f32,
    ) -> impl RunningResponse<Group> + 'a {
        let position = hitbox.position;

        ImmediateResultingOffset::new(
            offset_dir * offset_len,
            collisions
                .cast(hitbox, offset_dir, offset_len)
                .map(move |(dist, norm, data)| ResponseCollisionInformation {
                    global_position: position + offset_dir * dist,
                    normal: norm,
                    data,
                }),
        )
    }
}

// TODO: Implement responses

// fn touch_point<Group: ColliderGroup, BF: BroadPhase<Group>>(
//     colliders: &BF,
//     actor: &Group::Hitbox,
//     offset: Vec2,
// ) -> (Vec2, Option<(CollisionInformation, Group::ColliderData)>) {
//     let mut res = None;
//     let offset_normal = offset.normalize();
//     let position = actor.position();
//
//     for (collider, data) in colliders.cast(actor, offset) {
//         let dist = actor.cast(collider, offset);
//
//         if let Some((dist, normal)) = dist {
//             if let Some((old_dist, _, _)) = res {
//                 if old_dist > dist {
//                     res = Some((dist, normal, data));
//                 }
//             } else {
//                 res = Some((dist, normal, data));
//             }
//         }
//     }
//
//     if let Some((distance, normal, data)) = res {
//         let actual_offset = offset_normal * (distance - DELTA);
//         (
//             actual_offset,
//             Some((
//                 CollisionInformation {
//                     point: actual_offset + position,
//                     normal,
//                 },
//                 data,
//             )),
//         )
//     } else {
//         (offset, None)
//     }
// }
//
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Touch;
// impl CollisionResponse for Touch {
//     fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
//         &mut self,
//         colliders: &BF,
//         actor: &Group::Hitbox,
//         offset: Vec2,
//     ) -> ResponseResult<Group> {
//         let (offset, collider) = touch_point(colliders, actor, offset);
//
//         ResponseResult::new(offset, collider.into_iter().collect())
//     }
// }
//
// fn trajectory_change_on_touch<Group: ColliderGroup, BF: BroadPhase<Group>>(
//     colliders: &BF,
//     actor: &Group::Hitbox,
//     offset: Vec2,
//     // (movement that was supposed to be made, but stopped, normal of the collision) -> new movement from position where were stopped
//     mut new_trajectory: impl FnMut(Vec2, Dir2) -> Vec2,
// ) -> ResponseResult<Group, BF> {
//     let mut res_vec = Vec::new();
//
//     let mut last_offset = offset;
//     let (mut new_offset, mut opt_info) = touch_point(colliders, actor, offset);
//
//     let mut actor = actor.clone();
//
//     while let Some((info, data)) = opt_info {
//         actor.set_position(info.point);
//
//         let diff_offset = last_offset - new_offset;
//
//         last_offset = new_trajectory(diff_offset, info.normal);
//         (new_offset, opt_info) = touch_point(colliders, &actor, last_offset);
//
//         res_vec.push((info, data));
//     }
//
//     (actor.position() + new_offset, res_vec)
// }
//
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Slide;
//
// impl CollisionResponse for Slide {
//     fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
//         &mut self,
//         colliders: &BF,
//         actor: &Group::Hitbox,
//         offset: Vec2,
//     ) -> ResponseResult<Group, BF> {
//         trajectory_change_on_touch(colliders, actor, offset, |left_movement, normal| {
//             left_movement.project_onto_normalized(normal.perp())
//         })
//     }
// }
//
// #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct Bounce;
//
// impl CollisionResponse for Bounce {
//     fn respond<Group: ColliderGroup, BF: BroadPhase<Group>>(
//         &mut self,
//         colliders: &BF,
//         actor: &Group::Hitbox,
//         offset: Vec2,
//     ) -> ResponseResult<Group, BF> {
//         trajectory_change_on_touch(colliders, actor, offset, |left_movement, normal| {
//             left_movement - 2.0 * left_movement.project_onto_normalized(*normal)
//         })
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub struct LimitedBounce(pub u32);
//
// impl LimitedBounce {
//     #[inline]
//     pub fn new(bounces: u32) -> Self {
//         Self(bounces)
//     }
// }
