use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{AppState, SpriteAssets};

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(add_to_inventory))
            .add_event::<ItemPickup>()
            .register_inspectable::<Inventory>()
            .register_inspectable::<Item>();
    }
}

const Z_ITEM: f32 = 35.;
const MAX_ITEM_STACK: i32 = 999;

#[derive(Component, Inspectable, Clone)]
pub struct Item {
    pub name: String,
    pub amt: i32,
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
        }
    }
}

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: i32,
    pub last_picked_up_item_id: u32, //This variable is a dumb fix for events sending twice but it works. How elegant I feel it is would be another topic but so is ECS.
}

impl Inventory {
    pub fn new(capacity: i32) -> Self {
        Self {
            items: vec![],
            capacity,
            last_picked_up_item_id: 0,
        }
    }
}

#[derive(Component)]
pub struct Stackable;

pub struct ItemPickup {
    pub item: Entity,
    pub what_item: String,
    pub who: Entity,
}

//Called when an entity pickups an item
fn add_to_inventory(
    mut ev_itempickup: EventReader<ItemPickup>,
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>, //Every inventory
    mut all_items: Query<(&Item, Option<&Stackable>)>, //Every item
) {
    for ev in ev_itempickup.iter() {
        let mut ev_inventory = match inventories.get_mut(ev.who) {
            Ok(inv) => {
                if inv.items.len() as i32 <= inv.capacity {
                    inv
                } else {
                    eprintln!("Inventory is full, cannot perform action");
                    return;
                }
            }
            Err(err) => {
                panic!("There is no inventory on that entity err msg: {}", err);
            }
        };

        match all_items.get_mut(ev.item) {
            //make sure the ground item still exists and an old event didnt already handle it
            Ok((ground_item, is_stackable)) => {
                if is_stackable.is_some() && ev_inventory.items.contains(ground_item) {
                    for mut item_in_inv in ev_inventory
                        .items
                        .iter_mut()
                        .find(|item| *item == ground_item)
                    {
                        if item_in_inv.amt >= MAX_ITEM_STACK {
                            continue;
                        }
                        item_in_inv.amt += ground_item.amt;
                    }
                } else {
                    //If the item doesn't exist in the inventory or it does but it is not stackable
                    ev_inventory.items.insert(0, ground_item.clone());
                }
                println!("entity id:{} despawned", ev.item.id());
                //when we go to drop/remove items from the inventory, that should reset the last picked up id to 0
                ev_inventory.last_picked_up_item_id = ev.item.id();
                commands.entity(ev.item).despawn();
            }
            Err(err) => eprintln!("{}, id:{}", err, ev.item.id()),
        };
    }
}

fn startup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    for i in 0..100 {
        let offset_x: f32 = i as f32;
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: sprites.items.clone(),
                sprite: TextureAtlasSprite {
                    index: 1,
                    color: Color::MAROON,
                    ..default()
                },
                transform: Transform::from_xyz(30. + offset_x * 2.0, 5., Z_ITEM),
                ..default()
            })
            .insert(Item {
                name: "Wood".to_string(),
                amt: 1,
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
            .insert(Item {
                name: "Pebble".to_string(),
                amt: 1,
            })
            .insert(Stackable);
    }
}
