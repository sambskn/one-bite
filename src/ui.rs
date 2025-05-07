use bevy::prelude::*;

#[derive(Component)]
pub struct Arrow;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let arrow_texture = asset_server.load("arrow.png");
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            max_height: Val::Px(64.0),
            max_width: Val::Px(256.0),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
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
                        max_width: Val::Px(32.0),
                        max_height: Val::Px(32.0),
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
                Text::new("ordfinder")
            )
        ],
    ));
}
