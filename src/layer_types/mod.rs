pub mod collision_report_strategy;
mod scanner;
mod velocity;

use bevy::prelude::*;

pub use scanner::*;

/// Implements ScheduleLabel
const COLLISION_DETECTION_SCHEDULE: Update = Update;

pub trait BumpAppExtension {
    fn add_scanner_group<T: ScannerGroup>(&mut self) -> &mut Self;
}

impl BumpAppExtension for App {
    fn add_scanner_group<T: ScannerGroup>(&mut self) -> &mut Self {
        register_scanner_group::<T>(self);
        self
    }
}
