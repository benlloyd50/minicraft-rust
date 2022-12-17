use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;
use bracket_noise::prelude::*;
use rand::Rng;

use crate::{AppState, SpriteAssets};

const Z_FLOOR: f32 = 0.;
pub const TILE_SIZE: f32 = 16.; // coords are mulitplied by

pub struct MapGenerationPlugin;

#[derive(Component)]
pub struct UnwalkableTile;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::GameLoad).with_system(tm_startup));
    }
}

fn tm_startup(mut commands: Commands, tiles: Res<SpriteAssets>) {
    let tilemap_size = TilemapSize { x: 500, y: 500 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let mut rng = rand::thread_rng();

    let seed = rng.gen::<u64>();

    // Perlin noise map, settings are tweakable
    let mut noise = FastNoise::seeded(seed);
    noise.set_noise_type(NoiseType::SimplexFractal);
    noise.set_fractal_type(FractalType::FBM);
    noise.set_fractal_octaves(6);
    noise.set_fractal_gain(0.2);
    noise.set_fractal_lacunarity(2.0);
    noise.set_frequency(1.5);

    // Spawn the elements of the tilemap.
    for x in 0..tilemap_size.x as u32 {
        for y in 0..tilemap_size.y as u32 {
            let tile_entity = commands.spawn_empty().id();

            let tile_pos = TilePos { x, y };

            let mut perlin_value = noise.get_noise((x as f32) / 160.0, (y as f32) / 100.0);
            perlin_value = (perlin_value + 1.0) * 0.5;

            let tile_index: u32;
            if perlin_value > 0.75 {
                tile_index = 4; // Water
                commands.entity(tile_entity).insert(UnwalkableTile);
            } else if perlin_value > 0.7 {
                tile_index = 2; // Sand
            } else if perlin_value > 0.2 {
                tile_index = 0; // Grass
            } else {
                tile_index = 1; // Stone
                commands.entity(tile_entity).insert(UnwalkableTile);
            }
            // println!("PVal: {} TileIndex: {}", perlin_value, tile_index);

            commands.entity(tile_entity).insert(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(tile_index),
                ..Default::default()
            });
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    for y in 0..10 {
        let rect_width = 5.;
        let rect_height = 4.;
        commands.spawn((
            TransformBundle::from(Transform::from_xyz(
                3. * TILE_SIZE,
                y as f32 * TILE_SIZE,
                Z_FLOOR,
            )),
            Collider::cuboid(TILE_SIZE / 2. * rect_width, TILE_SIZE / 2. * rect_height),
        ));
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tiles.tiles1.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, Z_FLOOR),
        ..Default::default()
    });
}
