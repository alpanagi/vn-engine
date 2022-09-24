mod director;
mod domain;
mod interpreter;
mod script_loader;
mod ui;

use bevy::prelude::*;

use director::DirectorPlugin;
use domain::GameState;
use script_loader::ScriptLoaderPlugin;
use ui::BackgroundPlugin;
use ui::TextPlugin;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "VN Engine".to_string(),
            width: 800.,
            height: 600.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameState::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(ScriptLoaderPlugin)
        .add_plugin(DirectorPlugin)
        .add_plugin(TextPlugin)
        .add_plugin(BackgroundPlugin)
        .add_startup_system(setup)
        .run();
}
