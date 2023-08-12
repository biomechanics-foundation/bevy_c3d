use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypeUuid, TypePath},
    utils::BoxedFuture,
};
use c3dio::C3d;

#[derive(Default)]
pub struct C3dLoader;

impl AssetLoader for C3dLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
        ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move { Ok(load_c3d(bytes, load_context).await?) })
    }

    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["c3d"];
        EXTENSIONS
    }
}

async fn load_c3d<'a, 'b>(
    bytes: &'a [u8],
    load_context: &'a mut LoadContext<'b>,
    ) -> Result<(), bevy::asset::Error> {
    let c3d = C3d::from_bytes(bytes);
    let c3d = match c3d {
        Ok(c3d) => c3d,
        Err(_) => {
            return Err(bevy::asset::Error::msg(
                    "Failed to parse C3D file".to_string(),
                    ));
        }
    };
    let c3d_asset = C3dAsset { c3d };
    load_context.set_default_asset(LoadedAsset::new(c3d_asset));
    Ok(())
}

#[derive(Resource, Default, Debug)]
pub struct C3dState {
    pub handle: Handle<C3dAsset>,
    pub loaded: bool,
}


#[derive(Debug, TypeUuid, TypePath)]
#[type_path = "bevy_c3d::c3d_loader::C3dAsset"]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct C3dAsset {
    pub c3d: C3d,
}

#[derive(Debug, Event)]
pub struct C3dLoadedEvent;

pub fn c3d_loaded(
    mut ev_loaded: EventWriter<C3dLoadedEvent>,
    mut c3d_state: ResMut<C3dState>,
    custom_assets: ResMut<Assets<C3dAsset>>,
) {
    let custom_asset = custom_assets.get(&c3d_state.handle);
    if c3d_state.loaded || custom_asset.is_none() {
        return;
    }
    ev_loaded.send(C3dLoadedEvent);
    c3d_state.loaded = true;
}
