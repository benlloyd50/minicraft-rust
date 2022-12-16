use bevy::{prelude::*, ui::widget::ImageMode};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

use crate::{
    item::Item,
    player::{Interact, Player, PlayerEntity},
    AppState, FontAssets, SpriteAssets,
};

const MAX_ITEM_STACK: u32 = 999;
const Z_UI: f32 = 80.;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::GameLoad).with_system(inventory_ui_startup),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(
                    add_to_inventory
                        .label(Interact::Reciever)
                        .after(Interact::Caller),
                )
                .with_system(ui_inventory_update)
                .with_system(toggle_ui_menu),
        )
        .add_event::<ItemPickup>()
        .add_event::<PlayerPickupSuccess>()
        .add_event::<InventoryUpdate>()
        .register_inspectable::<Inventory>();
    }
}

#[derive(Component, Default, Inspectable)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: i32,
}

#[derive(Component)]
pub struct Stackable;

#[derive(Component)]
pub struct PlayerMenu;

#[derive(Component)]
pub struct InventoryUINode;

#[derive(Component)]
pub struct InventorySlot(i32);

impl Inventory {
    pub fn new(capacity: i32) -> Self {
        Self {
            items: vec![],
            capacity,
        }
    }
}

fn toggle_ui_menu(
    keeb_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Visibility, &mut Transform), With<InventoryUINode>>,
    mut ev_invopen: EventWriter<InventoryUpdate>,
) {
    let mut menu = query.single_mut();

    if keeb_input.just_pressed(KeyCode::X) {
        menu.0.is_visible = !menu.0.is_visible;
    }

    if menu.0.is_visible {
        ev_invopen.send(InventoryUpdate);
    }
}

//Events
pub struct ItemPickup {
    pub item: Entity,
    pub what_item: String,
    pub who: Entity,
}

pub struct PlayerPickupSuccess;

pub struct InventoryUpdate;

//Called when an entity pickups an item (only works for player)
fn add_to_inventory(
    mut ev_itempickup: EventReader<ItemPickup>,
    mut ev_success: EventWriter<PlayerPickupSuccess>,
    mut commands: Commands,
    mut inventories: Query<&mut Inventory>, //Every inventory
    all_items: Query<(&Item, Option<&Stackable>)>, //Every item
    player_e: Res<PlayerEntity>,
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
                info!("entity id:{} despawned", ev.item.index());
                if ev.who == player_e.0 {
                    ev_success.send(PlayerPickupSuccess);
                }
                commands.entity(ev.item).despawn();
            }
            Err(err) => eprintln!("{}, id:{}", err, ev.item.index()),
        };
    }
}

fn inventory_ui_startup(
    mut commands: Commands,
    font: Res<FontAssets>,
    elements: Res<SpriteAssets>,
) {
    let text_style = TextStyle {
        font: font.monogram.clone(),
        font_size: 24.0,
        color: Color::BLACK,
    };
    
    let inv_bg_style = Style {
        align_self: AlignSelf::Center,
        position_type: PositionType::Absolute,
        position: UiRect {
            top: Val::Percent(5.),
            left: Val::Percent(15.),
            ..default()
        },
        size: Size::new(Val::Px(400.), Val::Px(700.)),
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                transform: Transform::from_xyz(0., 0., Z_UI),
                visibility: Visibility { is_visible: false },   // inventory is hidden on startup
                ..default()
            },
            InventoryUINode,
        ))
        .with_children(|parent| {
            // the window which objects for the inventory ui will sit on
            parent
                .spawn(ImageBundle {
                    image: UiImage(elements.menu.clone()),
                    style: inv_bg_style.clone(),
                    image_mode: ImageMode::KeepAspect,
                    ..default()
                })
                .with_children(|inv_parent| {
                    // multiple text objects for items
                    for i in 0..20 {
                        let offset: f32 = i as f32 * 20.0;
                        inv_parent
                            .spawn(
                                TextBundle::from_section(String::new(), text_style.clone())
                                    .with_style(Style {
                                        align_self: AlignSelf::Center,
                                        position_type: PositionType::Absolute,
                                        position: UiRect {
                                            top: Val::Px(12. + offset),
                                            left: Val::Px(44.),
                                            ..default()
                                        },
                                        ..default()
                                    }),
                            )
                            .insert(InventorySlot(i));
                        // .with_children(|inv_slot_parent| {
                        //     inv_slot_parent.spawn_bundle(AtlasImageBundle {
                        //         style: Style {
                        //             align_self: AlignSelf::Center,
                        //             position_type: PositionType::Absolute,
                        //             position: UiRect {
                        //                 top: Val::Px(12. + offset),
                        //                 left: Val::Px(0.),
                        //                 ..default()
                        //             },
                        //             ..default()
                        //         },
                        //         atlas_image: UiAtlasImage {
                        //             atlas: elements.items.clone(),
                        //             index: 19,
                        //         },

                        //         ..Default::default()
                        //     });
                        // });
                    }
                });
        });
}

fn ui_inventory_update(
    mut ev_invopen: EventReader<InventoryUpdate>,
    mut q_ui_slots: Query<(&mut Text, &InventorySlot)>,
    q_inv: Query<(&mut Inventory, &Player)>, //Every inventory
    player_e: Res<PlayerEntity>,
) {
    for _ in ev_invopen.iter() {
        let (player_inv, _player) = match q_inv.get(player_e.0) {
            Ok((inv, p)) => (inv, p),
            Err(_) => panic!("Could not fetch the player's inventory!!!"),
        };

        for (mut text, slot_idx) in q_ui_slots.iter_mut() {
            let item = player_inv.items.get(slot_idx.0 as usize);
            if item != None {
                let i = item.unwrap().clone();
                text.sections[0].value = format!("{: <20}AMT:{:>3}", i.name, i.amt);
            } else {
                text.sections[0].value = String::from("------");
            }
        }
    }
}
