//! # C3D file loader for Bevy
//!
//! This crate provides a plugin for loading C3D files in the Bevy game engine.
//! It uses the [c3dio](https://crates.io/crates/c3dio) crate to parse the C3D files.
//!
//! ## Usage
//! Add the `bevy_c3d` crate to your `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! bevy = "0.12"
//! bevy_c3d = "0.12"
//! ```
//! Then add the `C3dPlugin` to your Bevy app:
//! ```rust
//! use bevy::prelude::*;
//! use bevy_c3d::prelude::*;
//!
//! fn main() {
//!    App::build()
//!    .add_plugins(DefaultPlugins)
//!    .add_plugin(C3dPlugin)
//!    .add_startup_system(setup)
//!    .run();
//! }
//!
//! ```
//!
//! Refer to the basic example for a complete example of how to use the plugin.
//!
//! You can run the example with the following command:
//! ```sh
//! cargo run --example basic
//! ```
//!
mod c3d_loader;

use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;

pub mod prelude {
    pub use crate::c3d_loader::*;
    pub use c3dio::prelude::*;
}

pub use prelude::*;

/// Plugin for loading C3D files
#[derive(Default)]
pub struct C3dPlugin;

/// Required components for loading C3D files
impl Plugin for C3dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<C3dState>()
            .register_asset_loader(C3dLoader)
            .init_asset::<C3dAsset>()
            .add_event::<C3dLoadedEvent>()
            .add_systems(Update, c3d_loaded);
    }
}
