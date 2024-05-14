use crate::{CursorPosition, GameState, Player};
use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Default)]
pub struct GunPlugin;

impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_gun_transform.run_if(in_state(GameState::InGame)),
        );
    }
}

fn update_gun_transform(
    player_query: Query<&Transform, With<Player>>,
    mut gun_query: Query<&mut Transform, (With<Gun>, Without<Player>)>,
    cursor_position: Res<CursorPosition>,
) {
    if player_query.is_empty() || gun_query.is_empty() {
        return;
    }

    if cursor_position.0.is_none() {
        return;
    }

    let player_transform = player_query.single();
    let player_position = player_transform.translation;

    let mut gun_transform = gun_query.single_mut();
    let cursor_position = cursor_position.0.unwrap();

    let direction_from_player_to_mouse = (cursor_position - player_position.truncate()).normalize();
    let angle = direction_from_player_to_mouse
        .y
        .atan2(direction_from_player_to_mouse.x);

    gun_transform.rotation = Quat::from_rotation_z(angle);
    let pos_offset: f32 = 50.0;
    gun_transform.translation = vec3(
        player_position.x + pos_offset * angle.cos(),
        player_position.y + pos_offset * angle.sin(),
        0.0,
    );
}

#[derive(Component)]
pub struct Gun;
