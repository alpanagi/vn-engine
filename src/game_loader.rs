use bevy::prelude::*;

use crate::{
    domain::{GameState, Script},
    script_parser::parse,
};

pub struct GameLoaderPlugin;
impl Plugin for GameLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_script)
            .add_system(script_parser);
    }
}

fn load_script(asset_server: Res<AssetServer>, mut game_state: ResMut<GameState>) {
    let script: Handle<Script> = asset_server.load("script.vn");
    game_state.unparsed_scripts.push(script);
}

fn script_parser(mut game_state: ResMut<GameState>, scripts: Res<Assets<Script>>) {
    if let Some(script_handle) = game_state.unparsed_scripts.pop() {
        if let Some(script) = scripts.get(&script_handle) {
            let mut commands = parse(&script.content);
            game_state.commands.append(&mut commands);
        }
    }
}
