use bevy::prelude::*;
use noise::{
    Fbm, Perlin,
    utils::{NoiseMapBuilder, PlaneMapBuilder},
};
use rand::Rng;

pub const WORLD_WIDTH: usize = 100;
pub const WORLD_HEIGHT: usize = 100;

#[derive(Component)]
pub struct WorldContent;

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

#[derive(Component)]
pub struct Target {
    pub position: Vec2,
}

#[derive(Component)]
pub struct GorbHole {
    pub position: Vec2,
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
    // load target texture
    // and determine random location within world bounds
    let mut rng = rand::rng();
    let target_texture = asset_server.load("haus.png");
    let target = Target {
        position: Vec2::new(
            rng.random_range(0..WORLD_WIDTH) as f32,
            rng.random_range(0..WORLD_HEIGHT) as f32,
        ),
    };
    commands.spawn((
        WorldContent,
        Sprite::from_image(target_texture),
        Transform::from_xyz(
            target.position.x * TILE_SIZE,
            target.position.y * TILE_SIZE,
            4.0,
        ),
        target,
    ));
    // spawn gorb home
    let gorb_hole_texture = asset_server.load("gorb_hole.png");
    let gorm_hole = GorbHole {
        position: Vec2::new((WORLD_WIDTH / 2) as f32, (WORLD_HEIGHT / 2) as f32),
    };
    commands.spawn((
        WorldContent,
        Sprite::from_image(gorb_hole_texture),
        Transform::from_xyz(
            gorm_hole.position.x * TILE_SIZE,
            gorm_hole.position.y * TILE_SIZE,
            4.0,
        ),
        gorm_hole,
    ));

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
            commands.spawn((sprite, transform, WorldContent));
        }
    }
    commands.insert_resource(world_map);
}

pub fn clear_world(mut commands: Commands, world_content_query: Query<Entity, With<WorldContent>>) {
    for world_entity in &world_content_query {
        commands.entity(world_entity).despawn();
    }
}
