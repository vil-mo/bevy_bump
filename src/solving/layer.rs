use crate::solving::response::CollisionResponse;
use enum_map::EnumArray;

pub trait CollisionLayer: EnumArray<CollisionResponse> {}
