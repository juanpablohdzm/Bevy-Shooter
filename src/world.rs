use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy_pancam::PanCam;
use crate::{BulletRate, GameState, GlobalTextureAtlas, Gun, Player, SPRITE_SCALE_FACTOR};

#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world));
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam{
            grab_buttons: vec![],
            ..default()
        });
}

fn init_world(
    mut commands: Commands,
    atlas: Res<GlobalTextureAtlas>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture: atlas.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: atlas.layout.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: atlas.image.clone().unwrap(),
            atlas: TextureAtlas {
                layout: atlas.layout.clone().unwrap(),
                index: 9,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Gun,
        BulletRate(Stopwatch::new()),
    ));

    next_state.set(GameState::InGame);
}