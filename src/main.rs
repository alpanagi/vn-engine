use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
