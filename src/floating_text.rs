use bevy::prelude::*;
use bevy::sprite::Anchor;

#[derive(Event)]
pub struct NewText(pub String, pub f32, pub f32); // text, x, y

#[derive(Component)]
pub struct FloatingText(pub f32); // text, lifetime

const FLOATING_TEXT_FONT_SIZE: f32 = 10.0;
const FLOATING_TEXT_MOVE_SPEED: f32 = 5.0;
const FLOATING_TEXT_Z: f32 = 6.0;
const FLOATING_TEXT_LIFETIME_MS: f32 = 1500.0;

pub fn handle_new_text_event(
    mut ev_new_text: EventReader<NewText>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in ev_new_text.read() {
        // make a new text entity
        let font = asset_server.load("fonts/PressStart2P.ttf");
        let text_font = TextFont {
            font,
            font_size: FLOATING_TEXT_FONT_SIZE,
            ..default()
        };
        commands.spawn((
            Text2d::new(ev.0.to_string()),
            text_font,
            Anchor::BottomCenter,
            Transform::from_xyz(ev.1, ev.2, FLOATING_TEXT_Z),
            TextColor(Color::Srgba(Srgba::rgb(1.0, 1.0, 1.0))),
            FloatingText(FLOATING_TEXT_LIFETIME_MS),
        ));
    }
}

pub fn update_floating_text(
    mut floating_text_query: Query<(Entity, &mut Transform, &mut FloatingText, &mut TextColor)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    // loop through text, update transform pos, kill if lifetime is gone
    for (entity, mut transform, mut floating_text, mut text_color) in &mut floating_text_query {
        floating_text.0 += -(time.delta().as_millis() as f32);
        if floating_text.0 <= 0.0 {
            commands.get_entity(entity).unwrap().despawn();
        } else {
            // update pos of text (and opacity??)
            transform.translation +=
                Vec3::new(0.0, FLOATING_TEXT_MOVE_SPEED * time.delta_secs(), 0.0);
            text_color.0 = Color::srgba(1.0, 1.0, 1.0, floating_text.0 / FLOATING_TEXT_LIFETIME_MS);
        }
    }
}
