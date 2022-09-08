use bevy::prelude::*;

pub struct TextEvent {
    pub text: String,
}

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TextEvent>()
            .add_startup_system(setup)
            .add_system(text);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        TextBundle::from_section(
            "",
            TextStyle {
                font: asset_server.load("fonts/dejavu/DejaVuSans.ttf"),
                font_size: 22.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(120.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    );
}

fn text(mut ev_text: EventReader<TextEvent>, mut query: Query<&mut Text>) {
    for text_event in ev_text.iter() {
        for mut text_display in &mut query {
            text_display.sections[0].value = text_event.text.clone();
        }
    }
}
