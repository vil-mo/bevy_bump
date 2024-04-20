// use std::marker::PhantomData;
// use glam::IVec2;
// use ahash::{HashSet, HashMap};
// use generational_arena::{Arena, Index};
// use crate::collider::Collider;
// 
// type Plane<T> = HashMap<IVec2, T>;
// 
// 
// #[derive(Debug, Clone, Default)]
// struct WorldCell {
//     solid_body: HashSet<Index>,
//     actor_body: HashSet<Index>,
// 
//     hitbox: HashSet<Index>,
//     hurtbox: HashSet<Index>,
// }
// 
// 
// #[derive(Debug, Default, Clone)]
// #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
// pub struct PhysicsWorld<Marker = ()> {
//     cell_size: u32,
// 
//     colliders: Arena<Collider>,
//     world: Plane<WorldCell>,
// 
// 
//     _pd: PhantomData<Marker>
// }
// 
// impl <Marker> Default for PhysicsWorld<Marker> {
//     fn default() -> Self {
//         Self {
//             cell_size: 64,
//             ..Default::default()
//         }
//     }
// }
// 
// impl <Marker> PhysicsWorld<Marker> {
//     fn
// }
// 
// //
// // #[derive(Debug, Clone)]
// // enum EntityBodyInCells {
// //     Solid(Box<[IVec2]>),
// //     Actor(Box<IVec2>),
// // }
// //
// // #[derive(Debug, Clone)]
// // struct EntityHitboxInCells(Box<[IVec2]>);
// //
// // #[derive(Debug, Clone)]
// // struct EntityHurtboxInCells(Box<[IVec2]>);
// //
// // #[derive(Debug, Clone, Default)]
// // struct EntityInCells {
// //     body: Option<EntityBodyInCells>,
// //     hitbox: Option<EntityHitboxInCells>,
// //     hurtbox: Option<EntityHurtboxInCells>,
// // }
// //
// // impl EntityInCells {
// //     fn is_empty(&self) -> bool {
// //         self.body.is_none() && self.hitbox.is_none() && self.hurtbox.is_none()
// //     }
// //
// //     fn remove_body_cell(&mut self) -> Option<EntityBodyInCells> {
// //         replace(&mut self.body, None)
// //     }
// //     fn remove_hitbox_cell(&mut self) -> Option<EntityHitboxInCells> {
// //         replace(&mut self.hitbox, None)
// //     }
// //     fn remove_hurtbox_cell(&mut self) -> Option<EntityHurtboxInCells> {
// //         replace(&mut self.hurtbox, None)
// //     }
// // }
// //
// // trait EntityCellHashMap<K: Hash + Eq + PartialEq> {
// //     fn remove_body_cell(&mut self, key: &K) -> Option<EntityBodyInCells>;
// //     fn remove_hitbox_cell(&mut self, key: &K) -> Option<EntityHitboxInCells>;
// //     fn remove_hurtbox_cell(&mut self, key: &K) -> Option<EntityHurtboxInCells>;
// // }
// //
// // impl <K: Hash + Eq + PartialEq> EntityCellHashMap<K> for HashMap<K, EntityInCells> {
// //     fn remove_body_cell(&mut self, key: &K) -> Option<EntityBodyInCells> {
// //         let cells_opt = self.get_mut(key);
// //
// //         match cells_opt {
// //             None => None,
// //             Some(cells) => {
// //                 let res = cells.remove_body_cell();
// //
// //                 if cells.is_empty() {
// //                     self.remove(key);
// //                 }
// //
// //                 res
// //             }
// //         }
// //     }
// //     fn remove_hitbox_cell(&mut self, key: &K) -> Option<EntityHitboxInCells> {
// //         let cells_opt = self.get_mut(key);
// //
// //         match cells_opt {
// //             None => None,
// //             Some(cells) => {
// //                 let res = cells.remove_hitbox_cell();
// //
// //                 if cells.is_empty() {
// //                     self.remove(key);
// //                 }
// //
// //                 res
// //             }
// //         }    }
// //     fn remove_hurtbox_cell(&mut self, key: &K) -> Option<EntityHurtboxInCells> {
// //         let cells_opt = self.get_mut(key);
// //
// //         match cells_opt {
// //             None => None,
// //             Some(cells) => {
// //                 let res = cells.remove_hurtbox_cell();
// //
// //                 if cells.is_empty() {
// //                     self.remove(key);
// //                 }
// //
// //                 res
// //             }
// //         }    }
// // }
// 
// 
// 
// //
// //
// // impl <Marker> PhysicsWorld<Marker> {
// //     pub fn new() -> Self {
// //         default()
// //     }
// //
// //     pub fn with_cell_size(cell_size: u32) -> Self {
// //         Self {
// //             cell_size,
// //             entity_cells: HashMap::new(),
// //             grid: HashMap::new(),
// //
// //             _pd: PhantomData,
// //         }
// //     }
// // }
// //
// //
// //
// // fn clear_removed_solid_bodies<Marker>(mut removed: RemovedComponents<SolidBody>, mut world_grid: ResMut<PhysicsWorld<Marker>>) {
// //     for entity in removed.read() {
// //         let entity_cell = world_grid.entity_cells.remove_body_cell(&entity);
// //
// //         if let Some(EntityBodyInCells::Solid(positions)) = entity_cell {
// //             for position in positions.into_iter() {
// //                 let Some(cell) = world_grid.grid.get_mut(position) else {continue};
// //                 let colliders = take(&mut cell.solid_colliders);
// //
// //                 cell.solid_colliders = colliders.into_iter().filter(|ci| ci.entity != entity).collect()
// //             }
// //         }
// //     }
// // }
