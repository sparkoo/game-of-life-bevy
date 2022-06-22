use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

#[derive(Debug, TypeUuid)]
#[uuid = "f68a9949-6fad-4a2f-9317-9bae18767c85"]
pub struct CoordsAsset {
    coords: Vec<(i32, i32)>,
}

pub struct Formations(pub Vec<Handle<CoordsAsset>>);

#[derive(Default)]
pub struct CoordsAssetLoader;

impl AssetLoader for CoordsAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::asset::BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            //            println!("bytes {:?}", bytes);
            let custom_asset = CoordsAsset {
                coords: vec![(1, 1), (2, 2)],
            };
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

pub struct LoaderPlugin;

impl Plugin for LoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<CoordsAsset>()
            .init_asset_loader::<CoordsAssetLoader>()
            .add_startup_system_to_stage(StartupStage::PreStartup, load_formations);
    }
}

fn load_formations(mut commands: Commands, asset_server: Res<AssetServer>) {
    match asset_server.load_folder("formations") {
        Ok(assets) => {
            let mut formations = Vec::new();
            for a in assets.iter() {
                let b = a.clone().typed::<CoordsAsset>();
                println!("coords {:?}", b);
                formations.push(b);
            }
            commands.insert_resource(formations);
        }

        Err(err) => eprintln!("failed loading assets {err}"),
    }
}
