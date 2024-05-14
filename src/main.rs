use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::close_on_esc;
use bevy_pancam::{PanCam, PanCamPlugin};
use bevy_test::*;

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
        .add_plugins(WorldPlugin)
        .add_systems(Update, close_on_esc)
        .run()
}
