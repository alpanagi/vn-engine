use bevy::prelude::*;

#[derive(Component)]
pub struct Speaker;

#[derive(Component)]
pub struct Dialog;

pub struct TextDisplayEvent {
    pub speaker: Option<String>,
    pub text: String,
}

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextDisplayEvent>()
            .add_startup_system(setup)
            .add_system(speaker_text)
            .add_system(dialog_text);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/dejavu/DejaVuSans.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(130.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Speaker);

    commands
        .spawn_bundle(
            TextBundle::from_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/dejavu/DejaVuSans.ttf"),
                    font_size: 22.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                align_self: AlignSelf::FlexStart,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(100.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(Dialog);
}

fn speaker_text(
    mut ev_text_display: EventReader<TextDisplayEvent>,
    mut query: Query<&mut Text, With<Speaker>>,
) {
    for text_event in ev_text_display.iter() {
        for mut text_display in &mut query {
            if let Some(speaker) = text_event.speaker.clone() {
                text_display.sections[0].value = speaker.clone();
            } else {
                text_display.sections[0].value = "".to_string();
            }
        }
    }
}

fn dialog_text(
    mut ev_text_display: EventReader<TextDisplayEvent>,
    mut query: Query<&mut Text, With<Dialog>>,
) {
    for text_event in ev_text_display.iter() {
        for mut text_display in &mut query {
            text_display.sections[0].value = text_event.text.clone();
        }
    }
}
