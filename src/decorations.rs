use crate::{constants::*, GameState, GlobalTextureAtlas};
use bevy::math::vec3;
use bevy::prelude::*;
use rand::Rng;

#[derive(Default)]
pub struct DecorationsPlugin;

impl Plugin for DecorationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), spawn_world_decoration);
    }
}

fn spawn_world_decoration(mut commands: Commands, atlas: Res<GlobalTextureAtlas>) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..=WORLD_W);
        let y = rng.gen_range(-WORLD_H..=WORLD_H);
        commands.spawn((SpriteSheetBundle {
            texture: atlas.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: atlas.layout.clone().unwrap(),
                index: rng.gen_range(12..=13),
            },
            transform: Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },));
    }
}
