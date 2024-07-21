use bevy::app::App;
use bevy::prelude::Commands;

pub mod core;
pub mod ecs_core;
pub mod registry;
pub mod utils;

pub mod prelude {}

trait Sealed {}
impl Sealed for App {}
impl<'w, 's> Sealed for Commands<'w, 's> {}

pub trait BumpAppExtension: Sealed {}
impl BumpAppExtension for App {}
pub trait BumpCommandsExtension: Sealed {}
impl<'w, 's> BumpCommandsExtension for Commands<'w, 's> {}
