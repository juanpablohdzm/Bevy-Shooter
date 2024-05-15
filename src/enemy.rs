use crate::{
    GameState, GlobalTextureAtlas, Player, ENEMY_SPAWN_RATE, ENEMY_SPEED, MAX_ENEMY_AMOUNT,
    SPRITE_SCALE_FACTOR, WORLD_H, WORLD_W,
};
use bevy::app::App;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;
use std::char::MAX;
use std::time::Duration;

pub struct EnemyPlugin;

#[derive(Component)]
pub struct Enemy;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(in_state(GameState::InGame))
                .run_if(on_timer(Duration::from_secs_f32(ENEMY_SPAWN_RATE))),
        )
        .add_systems(
            Update,
            update_enemy_transform.run_if(in_state(GameState::InGame)),
        );
    }
}

fn spawn_enemies(
    mut commands: Commands,
    atlas: Res<GlobalTextureAtlas>,
    player_query: Query<&Transform, With<Player>>,
    enemies_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) {
    let num_enemies = enemies_query.iter().len();
    let enemy_spawn_count = (MAX_ENEMY_AMOUNT - num_enemies).min(10);

    if num_enemies >= MAX_ENEMY_AMOUNT || player_query.is_empty() {
        return;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..enemy_spawn_count {
        let x = rng.gen_range(-WORLD_W..=WORLD_W);
        let y = rng.gen_range(-WORLD_H..=WORLD_H);
        commands.spawn((
            SpriteSheetBundle {
                texture: atlas.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: atlas.layout.clone().unwrap(),
                    index: 0,
                },
                transform: Transform::from_translation(vec3(x, y, 0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Enemy,
        ));
    }
}

fn update_enemy_transform(
    player_query: Query<&Transform, With<Player>>,
    mut enemies_query: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    if player_query.is_empty() || enemies_query.is_empty() {
        return;
    }

    let player_position = player_query.single().translation.truncate();
    for mut transform in &mut enemies_query {
        let mut direction = player_position - transform.translation.truncate();
        direction = direction.normalize();

        transform.translation += direction.extend(0.0) * ENEMY_SPEED;
    }
}
