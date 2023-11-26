mod c3d_loader;
pub use c3d_loader::*;

use bevy_app::{App, Plugin, Update};
use bevy_asset::AssetApp;

pub mod prelude {
    pub use crate::c3d_loader::*;
    pub use c3dio::prelude::*;
}

pub use prelude::*;

#[derive(Default)]
pub struct C3dPlugin;

impl Plugin for C3dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<C3dState>()
            .register_asset_loader(C3dLoader)
            .init_asset::<C3dAsset>()
            .add_event::<C3dLoadedEvent>()
            .add_systems(Update, c3d_loaded);
    }
}
