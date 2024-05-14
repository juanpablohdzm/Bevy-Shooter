use crate::{constants::*, GameState};
use bevy::prelude::*;
use bevy::time::Stopwatch;

#[derive(Default)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_bullets.run_if(in_state(GameState::InGame)));
    }
}

fn update_bullets(mut bullet_query: Query<(&mut Transform, &mut BulletDirection)>) {
    for (mut transform, bullet_direction) in &mut bullet_query {
        transform.translation += bullet_direction.0.normalize().extend(0.0) * BULLET_SPEED;
    }
}

#[derive(Component)]
pub struct Bullet;
#[derive(Component)]
pub struct BulletDirection(pub Vec2);
#[derive(Component)]
pub struct BulletRate(pub Stopwatch);
