use crate::{constants::*, GameState};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

#[derive(Default)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_movement_input, camera_follow_player)
                .run_if(in_state(GameState::InGame)),
        );
    }
}

#[derive(Component)]
pub struct Player;

fn handle_player_movement_input(
    mut player_query: Query<&mut Transform, With<Player>>,
    keyboard_input: ResMut<ButtonInput<KeyCode>>,
) {
    if player_query.is_empty() {
        return;
    }

    let mut transform = player_query.single_mut();
    let up = keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp);
    let down = keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown);
    let right =
        keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight);
    let left = keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft);

    let mut delta = vec2(0.0, 0.0);

    if up {
        delta.y += 1.0;
    }

    if down {
        delta.y -= 1.0;
    }

    if right {
        delta.x += 1.0;
    }

    if left {
        delta.x -= 1.0;
    }

    delta = if delta.x != 0.0 && delta.y != 0.0 {
        delta.normalize()
    } else {
        delta
    };

    transform.translation += vec3(delta.x, delta.y, 0.1) * PLAYER_SPEED;
}

fn camera_follow_player(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if camera_query.is_empty() || player_query.is_empty() {
        return;
    }

    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();
    let player_position = player_transform.translation.truncate();
    camera_transform.translation = camera_transform
        .translation
        .lerp(player_position.extend(camera_transform.translation.z), 0.1);
}
