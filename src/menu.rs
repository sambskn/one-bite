use bevy::prelude::*;

#[derive(Component)]
pub struct MenuStuff;

pub fn menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let frame_texture = asset_server.load("box_frame.png");
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    commands.spawn((Camera2d, MenuStuff));

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            height: Val::Vh(100.0),
            width: Val::Vw(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        MenuStuff,
        children![(
            Node {
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(16.0)),
                margin: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            ImageNode {
                image: frame_texture.clone(),
                image_mode: NodeImageMode::Sliced(slicer.clone()),
                ..default()
            },
            children![
                (
                    Node {
                        margin: UiRect {
                            left: Val::Px(00.0),
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            bottom: Val::Px(10.0)
                        },
                        ..default()
                    },
                    Text::new("gorb transporter"),
                    TextFont {
                        font: asset_server.load("castlevainia3nes.ttf"),
                        font_size: 24.0,
                        ..default()
                    },
                ),
                (
                    Node { ..default() },
                    Text::new("button sum starten"),
                    TextFont {
                        font: asset_server.load("castlevainia3nes.ttf"),
                        font_size: 12.0,
                        ..default()
                    },
                )
            ],
        ),],
    ));
}

pub fn clear_menu_content(
    mut commands: Commands,
    menu_stuff_query: Query<Entity, With<MenuStuff>>,
) {
    for menu_stuff_entity in &menu_stuff_query {
        commands.entity(menu_stuff_entity).despawn();
    }
}
