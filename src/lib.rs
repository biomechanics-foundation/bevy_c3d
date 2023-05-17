mod c3d_loader;
pub use c3d_loader::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct C3dPlugin;

impl Plugin for C3dPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<C3dState>()
            .add_asset::<C3dAsset>()
            .init_asset_loader::<C3dLoader>()
            .add_event::<C3dLoadedEvent>()
            .add_system(c3d_loaded)
    ;}
}
