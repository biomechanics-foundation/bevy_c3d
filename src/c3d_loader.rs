use bevy_asset::{
    io::Reader, Asset, AssetLoader, Assets, AsyncReadExt, BoxedFuture, Handle, LoadContext,
};
use bevy_ecs::prelude::{Event, EventWriter, ResMut, Resource};
use bevy_reflect::TypePath;
use c3dio::{C3d, C3dParseError};

/// Loader for C3D files
#[derive(Default)]
pub struct C3dLoader;

impl AssetLoader for C3dLoader {
    type Asset = C3dAsset;
    type Settings = ();
    type Error = C3dParseError;

    /// The most convenient way to load C3D files is to read the entire file into memory
    /// and then parse it.
    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<C3dAsset, C3dParseError>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            let res = reader.read_to_end(&mut bytes).await;
            if let Err(err) = res {
                return Err(C3dParseError::ReadError(err));
            }
            load_c3d(bytes.as_slice(), load_context).await
        })
    }

    /// C3D files have the extension "c3d"
    /// TODO: Ensure files that end in ".C3D" are also accepted
    fn extensions(&self) -> &[&str] {
        static EXTENSIONS: &[&str] = &["c3d"];
        EXTENSIONS
    }
}

async fn load_c3d<'a, 'b>(
    bytes: &'a [u8],
    _load_context: &'a mut LoadContext<'b>,
) -> Result<C3dAsset, C3dParseError> {
    let c3d = C3d::from_bytes(bytes);
    let c3d = match c3d {
        Ok(c3d) => c3d,
        Err(err) => {
            return Err(err);
        }
    };
    Ok(C3dAsset { c3d })
}

/// State for loading C3D files
/// Includes the path to the file, the handle to the asset, and whether the file has been loaded
#[derive(Resource, Default, Debug)]
pub struct C3dState {
    pub path: String,
    pub handle: Handle<C3dAsset>,
    pub loaded: bool,
}

/// Asset for C3D files
/// This is a wrapper around the C3D struct
/// It is required because Bevy's system needs a type path to work with
#[derive(Debug, TypePath, Asset)]
#[type_path = "bevy_c3d::c3d_loader::C3dAsset"]
pub struct C3dAsset {
    pub c3d: C3d,
}

/// Event that is sent when a C3D file is loaded
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
