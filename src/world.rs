use bevy::prelude::*;
use noise::{
    Fbm, Perlin,
    utils::{NoiseMapBuilder, PlaneMapBuilder},
};

pub const WORLD_WIDTH: usize = 200;
pub const WORLD_HEIGHT: usize = 200;

#[derive(Resource, Clone, Copy)]
pub struct WorldMap {
    pub vals: [usize; WORLD_WIDTH * WORLD_HEIGHT],
}

impl WorldMap {
    pub fn get_val_at_coord(self, x: i32, y: i32) -> Option<usize> {
        let idx = x + (y * WORLD_WIDTH as i32);
        if (idx as usize + 1) > (WORLD_WIDTH * WORLD_HEIGHT) || idx < 0 {
            None
        } else {
            Some(self.vals[idx as usize])
        }
    }
    pub fn set_val_at_coord(&mut self, x: usize, y: usize, value: usize) {
        self.vals[x + (y * WORLD_WIDTH)] = value;
    }
}

pub const TILE_SIZE: f32 = 32.0;
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
    let mut world_map = WorldMap {
        vals: [0; WORLD_WIDTH * WORLD_HEIGHT],
    };
    for y in 0..WORLD_WIDTH {
        for x in 0..WORLD_HEIGHT {
            let val = (plane_map_builder.get_value(x, y) as f32) + 0.5;
            let idx = (val * 15.0).floor() as usize;
            world_map.set_val_at_coord(x, y, idx);
            let sprite = Sprite::from_atlas_image(
                gradient_map_texture.clone(),
                TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: idx,
                },
            );
            let transform = Transform::from_xyz(
                x as f32 * TILE_SIZE,
                y as f32 * TILE_SIZE,
                -10.0 + val, // should be back in the back
            );
            commands.spawn((sprite, transform));
        }
    }
    commands.insert_resource(world_map);
}
