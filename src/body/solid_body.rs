// use bevy::prelude::*;
// use bevy::render::primitives::Aabb;
// 
// #[derive(Component, Debug, Clone)]
// pub struct SolidBody {
//     pub(crate) colliders: Box<[Aabb]>,
// }
// 
// impl SolidBody {
//     pub fn new(colliders: impl IntoIterator<Item = impl Into<Aabb>>) -> Self {
//         Self {
//             colliders: colliders
//                 .into_iter()
//                 .map(Into::<Aabb>::into)
//                 .collect::<Vec<_>>()
//                 .into(),
//         }
//     }
// }
