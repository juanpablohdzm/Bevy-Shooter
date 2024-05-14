use bevy::math::{vec2, vec3};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::{close_on_esc, PrimaryWindow};
use bevy_pancam::{PanCam, PanCamPlugin};
use rand::Rng;

// Window
const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 700.0;

//Sprites
const SPRITE_SHEET_PATH: &str = "assets.png";
const SPRITE_SCALE_FACTOR: f32 = 3.0;
const TILE_WIDTH: usize = 16;
const TILE_HEIGHT: usize = 16;
const SPRITE_SHEET_W: usize = 4;
const SPRITE_SHEET_H: usize = 4;

//World
const NUM_WORLD_DECORATIONS: usize = 100;
const WORLD_W: f32 = 3000.0;
const WORLD_H: f32 = 250.0;

//Colors
const BG_COLOR: (u8, u8, u8) = (197, 204, 184);

//Player
const PLAYER_SPEED: f32 = 2.0;

//Gun
const BULLET_SPAWN_INTERVAL: f32 = 0.05;
const BULLET_SPEED: f32 = 10.0;

//Game behavior
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    GameInit,
    InGame,
}

//Resources
#[derive(Resource)]
struct GlobalTextureAtlasHandle(Option<Handle<TextureAtlasLayout>>);

#[derive(Resource)]
struct GlobalSpriteSheetHandle(Option<Handle<Image>>);

#[derive(Resource)]
struct CursorPosition(Option<Vec2>);

//Components
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gun;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct BulletDirection(Vec2);

#[derive(Component)]
struct BulletRate(Stopwatch);

// Systems
fn load_assets(
    mut texture_atlas: ResMut<GlobalTextureAtlasHandle>,
    mut image_handle: ResMut<GlobalSpriteSheetHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    image_handle.0 = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    texture_atlas.0 = Some(texture_atlas_layout.add(layout));
    next_state.set(GameState::GameInit);
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(PanCam::default());
}

fn init_world(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },
        Player,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
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

fn spawn_world_decoration(
    mut commands: Commands,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..NUM_WORLD_DECORATIONS {
        let x = rng.gen_range(-WORLD_W..=WORLD_W);
        let y = rng.gen_range(-WORLD_H..=WORLD_H);
        commands.spawn((SpriteSheetBundle {
            texture: image_handle.0.clone().unwrap(),
            atlas: TextureAtlas {
                layout: texture_atlas.0.clone().unwrap(),
                index: rng.gen_range(12..=13),
            },
            transform: Transform::from_translation(vec3(x, y, 0.0))
                .with_scale(Vec3::splat(SPRITE_SCALE_FACTOR)),
            ..default()
        },));
    }
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
    let player_position = player_transform.translation;
    camera_transform.translation = camera_transform.translation.lerp(player_position, 0.1);
}

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

fn update_cursor_position(
    mut cursor_position: ResMut<CursorPosition>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    if window_query.is_empty() || camera_query.is_empty() {
        cursor_position.0 = None;
    }

    let (camera, camera_transform) = camera_query.single();

    let window = window_query.single();

    cursor_position.0 = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate());
}

fn handle_player_fire_input(
    mut commands: Commands,
    time: Res<Time>,
    mut gun_query: Query<(&Transform, &mut BulletRate), With<Gun>>,
    texture_atlas: Res<GlobalTextureAtlasHandle>,
    image_handle: Res<GlobalSpriteSheetHandle>,
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
                texture: image_handle.0.clone().unwrap(),
                atlas: TextureAtlas {
                    layout: texture_atlas.0.clone().unwrap(),
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

fn update_bullets(mut bullet_query: Query<(&mut Transform, &mut BulletDirection)>) {
    for (mut transform, bullet_direction) in &mut bullet_query {
        transform.translation += bullet_direction.0.normalize().extend(0.0) * BULLET_SPEED;
    }
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
        .add_plugins(PanCamPlugin::default())
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb_u8(
            BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
        )))
        .insert_resource(CursorPosition(None))
        .insert_resource(GlobalTextureAtlasHandle(None))
        .insert_resource(GlobalSpriteSheetHandle(None))
        .add_systems(OnEnter(GameState::Loading), load_assets)
        .add_systems(
            OnEnter(GameState::GameInit),
            (setup_camera, (spawn_world_decoration, init_world).chain()),
        )
        .add_systems(
            Update,
            (
                update_cursor_position,
                handle_player_movement_input,
                update_gun_transform,
                handle_player_fire_input,
                update_bullets,
                camera_follow_player,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Update, close_on_esc)
        .run()
}
