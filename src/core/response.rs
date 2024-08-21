use super::{collisions_query::CollisionsQuery, collider::Collider, ColliderGroup};
use bevy::prelude::*;

/// Collisions are accurate up to the DELTA distance
const DELTA: f32 = 0.0001;

/// Contains information about one of collisions that was processed with [`CollisionResponse`].
#[derive(Debug, Clone, Copy)]
pub struct CollisionInformation<Group: ColliderGroup> {
    /// The point on the desired path (or on the path corrected by solver) at with collision was detected
    /// Should make sense for it to be [`Collider::position`] of actor that performed movement
    pub global_position: Vec2,
    /// Result of [`Collider::normal`] of body against which collision was detected
    pub normal: Dir2,
    pub data: Group::HurtboxData,
}

pub enum RunningResponseVariant<T: RunningResponse<Group>, Group: ColliderGroup> {
    Collision(CollisionInformation<Group>, T),
    ResultingOffset(Vec2, T::AfterOutput),
}

pub trait RunningResponse<Group: ColliderGroup>: Sized {
    type AfterOutput: Iterator<Item = CollisionInformation<Group>>;

    fn next(self) -> RunningResponseVariant<Self, Group>;
}

enum CollisionsIteratorVariant<T: RunningResponse<Group>, Group: ColliderGroup> {
    BeforeOutput(T),
    AfterOutput(T::AfterOutput),
}

pub struct CollisionsIterator<'a, T: RunningResponse<Group>, Group: ColliderGroup> {
    buf: &'a mut Vec2,
    current_iter: CollisionsIteratorVariant<T, Group>,
}

impl<'a, T: RunningResponse<Group>, Group: ColliderGroup> Iterator
    for CollisionsIterator<'a, T, Group>
{
    type Item = CollisionInformation<Group>;

    fn next(&mut self) -> Option<Self::Item> {
        use CollisionsIteratorVariant::*;
        use RunningResponseVariant::*;

        replace_with::replace_with_or_abort_and_return(&mut self.current_iter, |current_iter| {
            match current_iter {
                AfterOutput(mut iter) => (iter.next(), AfterOutput(iter)),

                BeforeOutput(iter) => match iter.next() {
                    Collision(item, next_iter) => (Some(item), BeforeOutput(next_iter)),
                    ResultingOffset(value, mut next_iter) => {
                        *self.buf = value;

                        (next_iter.next(), AfterOutput(next_iter))
                    }
                },
            }
        })
    }
}

pub trait RunningResponseImpl<Group: ColliderGroup>: RunningResponse<Group> {
    fn into_iter(self, item: &mut Vec2) -> CollisionsIterator<Self, Group>;
    fn until_resulting_offset(
        self,
        f: impl FnMut(CollisionInformation<Group>),
    ) -> (Vec2, Self::AfterOutput);
}

impl<T: RunningResponse<Group>, Group: ColliderGroup> RunningResponseImpl<Group> for T {
    fn into_iter(self, buf: &mut Vec2) -> CollisionsIterator<Self, Group> {
        CollisionsIterator {
            buf,
            current_iter: CollisionsIteratorVariant::BeforeOutput(self),
        }
    }

    fn until_resulting_offset(
        mut self,
        mut f: impl FnMut(CollisionInformation<Group>),
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
}

#[inline(always)]
fn empty<Group: ColliderGroup>() -> std::iter::Empty<CollisionInformation<Group>> {
    std::iter::empty()
}

struct ImmediateResultingOffset<
    Next: Iterator<Item = CollisionInformation<Group>>,
    Group: ColliderGroup,
> {
    offset: Vec2,
    next: Next,
}

impl<Next: Iterator<Item = CollisionInformation<Group>>, Group: ColliderGroup>
    ImmediateResultingOffset<Next, Group>
{
    fn new(offset: Vec2, next: Next) -> Self {
        ImmediateResultingOffset { offset, next }
    }
}

impl<'a, Next: Iterator<Item = CollisionInformation<Group>>, Group: ColliderGroup>
    RunningResponse<Group> for ImmediateResultingOffset<Next, Group>
{
    type AfterOutput = Next;

    fn next(self) -> RunningResponseVariant<Self, Group> {
        RunningResponseVariant::ResultingOffset(self.offset, self.next)
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
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group>>(
        &'a mut self,
        colliders: &'a Collisions,
        actor: Collider<'a, Group::Hitbox>,
        offset: Vec2,
    ) -> impl RunningResponse<Group> + 'a;
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ignore;

impl CollisionResponse for Ignore {
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group>>(
        &'a mut self,
        _colliders: &'a Collisions,
        _actor: Collider<'a, Group::Hitbox>,
        offset: Vec2,
    ) -> impl RunningResponse<Group> + 'a {
        ImmediateResultingOffset::new(offset, empty())
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pass;

impl CollisionResponse for Pass {
    fn respond<'a, Group: ColliderGroup, Collisions: CollisionsQuery<Group>>(
        &'a mut self,
        colliders: &'a Collisions,
        actor: Collider<'a, Group::Hitbox>,
        offset: Vec2,
    ) -> impl RunningResponse<Group> + 'a {
        let position = actor.position;
        let normal = offset.normalize();

        ImmediateResultingOffset::new(
            offset,
            colliders
                .cast(actor, offset)
                .map(move |(dist, norm, data)| CollisionInformation {
                    global_position: position + normal * dist,
                    normal: norm,
                    data,
                }),
        )
    }
}

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
