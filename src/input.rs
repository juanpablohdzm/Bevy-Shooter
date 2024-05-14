use crate::{
    Bullet, BulletDirection, BulletRate, GameState, GlobalTextureAtlas, Gun, BULLET_SPAWN_INTERVAL,
    SPRITE_SCALE_FACTOR,
};
use bevy::math::vec3;
use bevy::prelude::*;

#[derive(Default)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            handle_player_fire_input.run_if(in_state(GameState::InGame)),
        );
    }
}

fn handle_player_fire_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut BulletRate), With<Gun>>,
    atlas: Res<GlobalTextureAtlas>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if gun_query.is_empty() {
        return;
    }

    let (gun_transform, mut gun_stopwatch) = gun_query.single_mut();
    gun_stopwatch.0.tick(time.delta());

    if !mouse_button_input.pressed(MouseButton::Left) {
        return;
    }

    let gun_position = gun_transform.translation.truncate();
    let gun_direction = gun_transform.rotation * vec3(1.0, 0.0, 0.0);

    if gun_stopwatch.0.elapsed_secs() >= BULLET_SPAWN_INTERVAL {
        commands.spawn((
            SpriteSheetBundle {
                texture: atlas.image.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: atlas.layout.clone().unwrap(),
                    index: 8,
                },
                transform: Transform::from_translation(gun_position.extend(0.0))
                    .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
                ..default()
            },
            Bullet,
            BulletDirection(gun_direction.truncate()),
        ));
        gun_stopwatch.0.reset();
    }
}
