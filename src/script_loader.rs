use bevy::{
    asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
};

use crate::{domain::GameState, interpreter::parse_text};

#[derive(TypeUuid)]
#[uuid = "243e3962-da47-4f5f-8e60-4eab0598dc6a"]
pub struct Script {
    pub content: String,
}

#[derive(Default)]
struct ScriptLoader;

impl AssetLoader for ScriptLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let contents = std::str::from_utf8(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(Script {
                content: contents.to_string(),
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["vn"]
    }
}
pub struct ScriptLoaderPlugin;
impl Plugin for ScriptLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<Script>()
            .init_asset_loader::<ScriptLoader>()
            .add_startup_system(load_script)
            .add_system(process_script);
    }
}

fn load_script(asset_server: Res<AssetServer>, mut game_state: ResMut<GameState>) {
    let script: Handle<Script> = asset_server.load("script.vn");
    game_state.unprocessed_scripts.push(script);
}

fn process_script(mut game_state: ResMut<GameState>, scripts: Res<Assets<Script>>) {
    if let Some(script_handle) = game_state.unprocessed_scripts.pop() {
        if let Some(script) = scripts.get(&script_handle) {
            let (game_data, mut commands) = parse_text(&script.content);
            game_state.data = game_data;
            game_state.commands.append(&mut commands);
        }
    }
}
