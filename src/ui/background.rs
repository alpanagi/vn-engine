use bevy::prelude::*;

#[derive(Component)]
struct BackgroundDisplay;

pub struct BackgroundDisplayEvent {
    pub image_path: String,
}

pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BackgroundDisplayEvent>()
            .add_startup_system(setup)
            .add_system(background_display);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(BackgroundDisplay);
}

fn background_display(
    mut ev_background: EventReader<BackgroundDisplayEvent>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility), With<BackgroundDisplay>>,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_background.iter() {
        for (mut image_handle, mut visibility) in &mut query {
            if ev.image_path != "".to_string() {
                visibility.is_visible = true;
                *image_handle = asset_server.load(&format!("backgrounds/{}", &ev.image_path));
            } else {
                visibility.is_visible = false;
            }
        }
    }
}
