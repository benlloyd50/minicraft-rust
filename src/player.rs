use crate::{
    inventory::{Inventory, InventoryUpdate, ItemPickup},
    item::Item,
    SpriteAssets,
};

use super::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_rapier2d::prelude::*;

const Z_PLAYER: f32 = 40.;
const PLAYER_SPEED: f32 = 80.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_input.label(PSystems::Input))
                    .with_system(
                        move_player
                            .label(PSystems::Movement)
                            .before(PSystems::Input),
                    )
                    .with_system(direction_animation)
                    .with_system(
                        pickup_item
                            .label(Interact::Caller)
                            .after(PSystems::Movement),
                    ),
            )
            .register_inspectable::<PlayerState>();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum PSystems {
    Input,
    Movement,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum Interact {
    Caller,
    Reciever,
    _Ui,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component, Inspectable)]
enum PlayerState {
    Idle,
    Moving,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct InputCapture {
    movement: Vec2,
}

#[derive(Resource)]
pub struct PlayerEntity(pub Entity);

fn startup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let player_entity = commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: sprites.player_move.clone(),
                transform: Transform::from_xyz(10., 0., Z_PLAYER),
                ..Default::default()
            },
            Player,
            AnimationTimer(Timer::from_seconds(0.175, TimerMode::Repeating)),
            PlayerState::Idle,
            InputCapture {
                movement: Vec2::ZERO,
            },
            Direction::Down,
            Inventory::new(20),
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::round_cuboid(1.0, 1.0, 0.05),
            ActiveEvents::COLLISION_EVENTS,
            LockedAxes::ROTATION_LOCKED,
        ))
        .id();

    commands.insert_resource(PlayerEntity(player_entity));
}

fn direction_animation(
    time: Res<Time>,
    mut q: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Direction,
        &mut PlayerState,
    )>,
) {
    for (mut timer, mut sprite, dir, state) in &mut q {
        match *state {
            PlayerState::Idle => {
                return;
            }
            PlayerState::Moving => {}
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            match *dir {
                Direction::Up => {
                    sprite.index = 1;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Down => {
                    sprite.index = 0;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Right => {
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = false;
                }
                Direction::Left => {
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = true;
                }
            }
        }
    }
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<(&mut InputCapture, &mut Direction), With<Player>>,
) {
    let (mut input, mut dir) = q.single_mut();
    input.movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        input.movement.x -= 1.0;
        *dir = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input.movement.x += 1.0;
        *dir = Direction::Right;
    }
    if keyboard_input.pressed(KeyCode::W) {
        input.movement.y += 1.0;
        *dir = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input.movement.y -= 1.0;
        *dir = Direction::Down;
    }
}

// Velocity based movement
fn move_player(
    _time: Res<Time>,
    mut q: Query<(&mut Velocity, &mut PlayerState, &InputCapture), With<Player>>,
) {
    let (mut velocity, mut state, input_val) = q.single_mut();

    let move_delta = Vec2::new(input_val.movement.x, input_val.movement.y);

    if input_val.movement != Vec2::ZERO {
        *state = PlayerState::Moving;
        velocity.linvel = move_delta * PLAYER_SPEED;
    } else {
        *state = PlayerState::Idle;
        velocity.linvel = Vec2::ZERO;
    }
}

type InventoryQuery<'a> = (&'a Transform, Entity, &'a Inventory);
type ItemQuery<'a> = (&'a Transform, Entity, &'a Item);
//A collider system may be more advantageous
fn pickup_item(
    player_q: Query<InventoryQuery, (With<Player>, Without<Item>)>,
    items_q: Query<ItemQuery, (Without<Inventory>, Without<Player>)>,
    mut ev_itempickup: EventWriter<ItemPickup>,
    mut ev_inventory_update: EventWriter<InventoryUpdate>,
) {
    let (player, who, _inv) = player_q.single();
    for (transform, item, info) in items_q.iter() {
        let item_pos = Vec2::new(transform.translation.x, transform.translation.y);
        let player_pos = Vec2::new(player.translation.x, player.translation.y);

        let dist = item_pos.distance(player_pos);
        if dist < 8.0 {
            ev_itempickup.send(ItemPickup {
                item,
                what_item: info.name.to_string(),
                who,
            });
            ev_inventory_update.send(InventoryUpdate);
        }
    }
}
