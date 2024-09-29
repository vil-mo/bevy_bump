mod scanner;

use bevy::prelude::*;

pub use scanner::*;

pub trait BumpAppExtension {
    fn add_scanner_group<T: ScannerGroup>(&mut self) -> &mut Self;
}

impl BumpAppExtension for App {
    fn add_scanner_group<T: ScannerGroup>(&mut self) -> &mut Self {
        register_scanner_group::<T>(self);
        self
    }
}
