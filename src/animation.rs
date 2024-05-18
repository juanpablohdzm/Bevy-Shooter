use std::cmp::Ordering;
use crate::{CursorPosition, GameState, Player};
use bevy::prelude::*;

pub struct AnimationPlugin;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animation_timer_tick, animate_player).run_if(in_state(GameState::InGame)),
        );
    }
}
pub fn animation_timer_tick(
    time: Res<Time>,
    mut animation_timer_query: Query<&mut AnimationTimer>,
) {
    for mut timer in &mut animation_timer_query {
        //(*timer).0.tick(time.delta());
        timer.tick(time.delta());
    }
}
pub fn animate_player(
    cursor_position: Res<CursorPosition>,
    mut player_query: Query<(&mut TextureAtlas, &mut Sprite, &Transform, &AnimationTimer), With<Player>>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut texture_atlas, mut sprite,  transform,  animation_timer) = player_query.single_mut();
    if animation_timer.just_finished() {
        texture_atlas.index = (texture_atlas.index + 1) % 6
    }
    
    if let Some(cursor_position) = cursor_position.0 {
        sprite.flip_x = cursor_position.x.partial_cmp(&transform.translation.x) == Some(Ordering::Less);
    }
}
