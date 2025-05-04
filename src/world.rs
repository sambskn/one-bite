use bevy::prelude::*;
use noise::{
    Fbm, Perlin,
    utils::{NoiseMapBuilder, PlaneMapBuilder},
};

const WORLD_WIDTH: usize = 100;
const WORLD_HEIGHT: usize = 100;
pub const TILE_SIZE: f32 = 32.0;
const WOLRD_OFFSET: f32 = 50.0 * TILE_SIZE;
pub fn generate_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // generate noise
    let fbm = Fbm::<Perlin>::default();
    let plane_map_builder = PlaneMapBuilder::new(&fbm)
        .set_size(WORLD_WIDTH, WORLD_HEIGHT)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build();
    // load texture
    let gradient_map_texture = asset_server.load("gradient_1bit_map.png");
    let layout = TextureAtlasLayout::from_grid(UVec2 { x: 32, y: 32 }, 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for x in 0..WORLD_WIDTH {
        for y in 0..WORLD_HEIGHT {
            let val = plane_map_builder.get_value(x, y) as f32 + 0.5;
            let index = (val * 16.0).floor() as usize;
            let sprite = Sprite::from_atlas_image(
                gradient_map_texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone_weak(),
                    index,
                },
            );
            let mut transform = Transform::from_xyz(
                (x as f32 * TILE_SIZE) - WOLRD_OFFSET,
                (y as f32 * TILE_SIZE) - WOLRD_OFFSET,
                -10.0 + val, // should be back in the back
            );
            let semi_rand = (val * 10000.0 - (val * 10000.0).floor()).abs();
            match semi_rand {
                0.0..=0.25 => transform.rotate_local_y((90.0 as f32).to_radians()),
                0.25..=0.5 => transform.rotate_local_y((180.0 as f32).to_radians()),
                0.5..=0.75 => transform.rotate_local_y((270.0 as f32).to_radians()),
                _ => {}
            }
            commands.spawn((sprite, transform));
        }
    }
}
