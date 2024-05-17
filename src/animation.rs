use bevy::prelude::*;
use crate::{GameState, Player};

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate_player).run_if(in_state(GameState::InGame)));
    }
}
pub fn animation_timer_tick(
    time: Res<Time>,
    mut animation_timer_query : Query<&mut AnimationTimer>
){
    for mut timer in &mut animation_timer_query {
        //(*timer).0.tick(time.delta()); ==
        timer.tick(time.delta());
    }
}
pub fn animate_player(
   mut player_query : Query<(&mut TextureAtlas, &mut AnimationTimer), With<Player>>
){
    if player_query.is_empty() {
        return;
    }
    let (mut texture_atlas, mut animation_timer) = player_query.single_mut();
    texture_atlas.index = (texture_atlas.index + 1) % 8
}
