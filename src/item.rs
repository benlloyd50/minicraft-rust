use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{inventory::Stackable, AppState, SpriteAssets};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup))
            .register_inspectable::<Item>();
    }
}

const Z_ITEM: f32 = 35.;

#[derive(Component, Inspectable, Clone)]
pub struct Item {
    pub name: String,
    pub amt: u32,
    pub sprite_index: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl Eq for Item {}

impl Default for Item {
    fn default() -> Self {
        Item {
            name: "Empty".to_string(),
            amt: 0,
            sprite_index: 0,
        }
    }
}

fn startup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    for i in 0..100 {
        let offset_x: f32 = i as f32;
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.items.clone(),
                sprite: TextureAtlasSprite {
                    index: 4,
                    color: Color::MAROON,
                    ..default()
                },
                transform: Transform::from_xyz(30. + offset_x * 2.0, 5., Z_ITEM),
                ..default()
            })
            .insert(Item {
                name: format!("Wood, {}", i % 10),
                amt: 1,
                sprite_index: 4,
            })
            .insert(Stackable);
    }

    for i in (0..100).step_by(10) {
        let offset_x: f32 = i as f32;
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.items.clone(),
                sprite: TextureAtlasSprite {
                    index: 2,
                    color: Color::GRAY,
                    ..default()
                },
                transform: Transform::from_xyz(30. + offset_x * 2.0, 20., Z_ITEM),
                ..default()
            })
            .insert(Item::pebble())
            .insert(Stackable);
    }
}

impl Item {
    fn pebble() -> Item {
        Item {
            name: "pebble".to_string(),
            amt: 1,
            sprite_index: 2,
        }
    }

    fn wood() -> Item {
        Item {
            name: "Wood".to_string(),
            amt: 1,
            sprite_index: 4,
        }
    }
}
