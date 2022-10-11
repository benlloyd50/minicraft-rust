use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{assetload::SoundAssets, inventory::PlayerPickupSuccess, AppState};
pub struct GameSoundPlugin;

impl Plugin for GameSoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::InGame).with_system(p_item_pickup_sfx));
    }
}

pub fn p_item_pickup_sfx(
    mut ev_itempickup: EventReader<PlayerPickupSuccess>,
    noises: Res<SoundAssets>,
    audio: Res<Audio>,
) {
    for _ in ev_itempickup.iter() {
        audio.play(noises.item_pickup.clone()).with_volume(0.05);
    }
}
