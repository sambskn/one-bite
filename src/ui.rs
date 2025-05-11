use std::time::Duration;

use bevy::prelude::*;

use crate::player::Player;
use crate::world::WorldContent;

#[derive(Component)]
pub struct Arrow;

#[derive(Component)]
pub struct GorbHolderUI;

#[derive(Component)]
pub struct Gorb;

#[derive(Component)]
pub struct SterbTime(pub Timer);

pub const SECONDS_IN_TIMER: f32 = 60.0 * 1.5;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let arrow_texture = asset_server.load("arrow.png");
    let gorb_texture = asset_server.load("gorb.png");
    let frame_texture = asset_server.load("box_frame.png");
    let slicer = TextureSlicer {
        border: BorderRect::all(5.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    commands.spawn((
        WorldContent,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            height: Val::Vh(25.0),
            width: Val::Vw(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
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
                        Node { ..default() },
                        Arrow,
                        children![(
                            ImageNode {
                                image: arrow_texture,
                                ..default()
                            },
                            Node {
                                width: Val::Px(32.0),
                                height: Val::Px(32.0),
                                ..default()
                            }
                        )]
                    ),
                    (
                        Node {
                            margin: UiRect {
                                left: Val::Px(10.0),
                                right: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(0.0)
                            },
                            ..default()
                        },
                        Text::new("ortfinder"),
                        TextFont {
                            font: asset_server.load("castlevainia3nes.ttf"),
                            font_size: 12.0,
                            ..default()
                        },
                    )
                ],
            ),
            (
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
                                left: Val::Px(0.0),
                                right: Val::Px(0.0),
                                top: Val::Px(0.0),
                                bottom: Val::Px(10.0)
                            },
                            ..default()
                        },
                        Text::new("gorbs chamber"),
                        TextFont {
                            font: asset_server.load("castlevainia3nes.ttf"),
                            font_size: 12.0,
                            ..default()
                        },
                    ),
                    (
                        Node {
                            display: Display::Flex,
                            justify_content: JustifyContent::Start,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        GorbHolderUI,
                        children![(
                            Gorb,
                            ImageNode {
                                image: gorb_texture,
                                ..default()
                            },
                            Node {
                                max_width: Val::Px(32.0),
                                max_height: Val::Px(32.0),
                                margin: UiRect::all(Val::Px(2.0)),
                                ..default()
                            }
                        )]
                    )
                ],
            ),
            (
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
                        Text::new("dead time"),
                        TextFont {
                            font: asset_server.load("castlevainia3nes.ttf"),
                            font_size: 12.0,
                            ..default()
                        },
                    ),
                    (
                        Node { ..default() },
                        Text::new("00:30:00"),
                        TextFont {
                            font: asset_server.load("castlevainia3nes.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        SterbTime(Timer::new(
                            Duration::new(SECONDS_IN_TIMER as u64, 0),
                            TimerMode::Once
                        ))
                    )
                ],
            ),
        ],
    ));
}

pub fn update_gorb_count(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Player>,
    gorb_holder_query: Query<Entity, With<GorbHolderUI>>,
    gorb_query: Query<Entity, With<Gorb>>,
) {
    let mut ui_gorb_count = 0;
    for _gorb in &gorb_query {
        ui_gorb_count += 1;
    }
    for &player in player_query {
        let gorb_count = player.gorb_count;
        if gorb_count != ui_gorb_count {
            let gorb_texture = asset_server.load("gorb.png");
            for gorb_entity in &gorb_query {
                commands.entity(gorb_entity).despawn();
            }
            for gorb_holder_entity in &gorb_holder_query {
                for _i in 0..gorb_count {
                    commands.entity(gorb_holder_entity).with_child((
                        Gorb,
                        ImageNode {
                            image: gorb_texture.clone(),
                            ..default()
                        },
                        Node {
                            max_width: Val::Px(32.0),
                            max_height: Val::Px(32.0),
                            margin: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}
