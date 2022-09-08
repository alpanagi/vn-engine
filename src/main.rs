mod director;
mod domain;
mod game_loader;
mod input;
mod script_loader;
mod script_parser;
mod text;

use bevy::prelude::*;
use director::DirectorPlugin;
use domain::{GameState, Script};
use game_loader::GameLoaderPlugin;
use input::InputPlugin;
use script_loader::ScriptLoader;
use text::TextPlugin;

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
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .add_asset::<Script>()
        .init_asset_loader::<ScriptLoader>()
        .add_plugin(GameLoaderPlugin)
        .add_plugin(DirectorPlugin)
        .add_plugin(TextPlugin)
        .add_plugin(InputPlugin)
        .add_startup_system(setup)
        .run();
}
