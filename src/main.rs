mod domain;
mod game_loader;
mod script_loader;
mod script_parser;
mod text;

use bevy::prelude::*;
use domain::{GameState, Script};
use game_loader::GameLoaderPlugin;
use script_loader::ScriptLoader;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "VN Engine".to_string(),
            width: 800.,
            height: 600.,
            ..default()
        })
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .add_asset::<Script>()
        .init_asset_loader::<ScriptLoader>()
        .add_plugin(GameLoaderPlugin)
        .add_startup_system(setup)
        .run();
}
