use bevy::prelude::*;

use crate::{
    domain::{GameState, Script},
    interpreter::process,
};

pub struct GameLoaderPlugin;
impl Plugin for GameLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_script)
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
            let mut commands = process(&script.content);
            game_state.commands.append(&mut commands);
        }
    }
}
