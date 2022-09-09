use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundDisplay;

pub struct BackgroundEvent {
    pub image: String,
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackgroundEvent>()
            .add_startup_system(setup)
            .add_system(background_display);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle::default())
        .insert(BackgroundDisplay);
}

fn background_display(
    mut ev_background: EventReader<BackgroundEvent>,
    mut query: Query<&mut Handle<Image>, With<BackgroundDisplay>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_background.iter() {
        for mut image_handle in &mut query {
            *image_handle = asset_server.load(&format!("backgrounds/{}", &ev.image));
        }
    }
}
