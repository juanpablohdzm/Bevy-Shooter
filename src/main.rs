use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::close_on_esc;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_test::*;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
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

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: true,
                        focused: true,
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PanCamPlugin)
        .add_plugins(ResourcesPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(GunPlugin)
        .add_plugins(BulletPlugin)
        .add_plugins(DecorationsPlugin)
        .add_plugins(InputPlugin)
        .add_systems(OnEnter(GameState::GameInit), (setup_camera, init_world))
        .add_systems(Update, close_on_esc)
        .run()
}
