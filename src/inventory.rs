use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    item::Item,
    player::{Interact, PlayerID},
    AppState,
};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::InGame).with_system(
                add_to_inventory
                    .label(Interact::Reciever)
                    .after(Interact::Caller),
            ),
        )
        .add_event::<ItemPickup>()
        .add_event::<PlayerPickupSuccess>()
        .register_inspectable::<Inventory>();
    }
}

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: i32,
}

const MAX_ITEM_STACK: u32 = 999;

impl Inventory {
    pub fn new(capacity: i32) -> Self {
        Self {
            items: vec![],
            capacity,
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

pub struct PlayerPickupSuccess;

//Called when an entity pickups an item
fn add_to_inventory(
    mut ev_itempickup: EventReader<ItemPickup>,
    mut ev_success: EventWriter<PlayerPickupSuccess>,
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>, //Every inventory
    all_items: Query<(&Item, Option<&Stackable>)>, //Every item
    player_id: Res<PlayerID>,
) {
    for ev in ev_itempickup.iter() {
        let mut ev_inventory = match inventories.get_mut(ev.who) {
            Ok(inv) => {
                if (inv.items.len() as i32) < inv.capacity {
                    inv
                } else {
                    warn!("Inventory is full, cannot perform action");
                    return;
                }
            }
            Err(err) => {
                error!("There is no inventory on that entity err msg: {}", err);
                return;
            }
        };

        match all_items.get(ev.item) {
            //make sure the ground item still exists and an old event didnt already handle it
            Ok((ground_item, is_stackable)) => {
                if is_stackable.is_some() && ev_inventory.items.contains(ground_item) {
                    if let Some(mut item_in_inv) = ev_inventory
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
                info!("entity id:{} despawned", ev.item.id());
                if ev.who.id() == player_id.0 {
                    ev_success.send(PlayerPickupSuccess);
                }
                commands.entity(ev.item).despawn();
            }
            Err(err) => eprintln!("{}, id:{}", err, ev.item.id()),
        };
    }
}
