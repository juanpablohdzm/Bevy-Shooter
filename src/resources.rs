use crate::{constants::*, GameState};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

#[derive(Default)]
pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .insert_resource(ClearColor(Color::rgb_u8(
                BG_COLOR.0, BG_COLOR.1, BG_COLOR.2,
            )))
            .insert_resource(CursorPosition(None))
            .insert_resource(GlobalTextureAtlas::default())
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(
                Update,
                update_cursor_position.run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Resource, Default)]
pub struct GlobalTextureAtlas {
    pub layout: Option<Handle<TextureAtlasLayout>>,
    pub image: Option<Handle<Image>>,
}
#[derive(Resource)]
pub struct CursorPosition(pub Option<Vec2>);

fn load_assets(
    mut atlas: ResMut<GlobalTextureAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    atlas.image = Some(asset_server.load(SPRITE_SHEET_PATH));
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(TILE_WIDTH as f32, TILE_HEIGHT as f32),
        SPRITE_SHEET_W,
        SPRITE_SHEET_H,
        None,
        None,
    );
    atlas.layout = Some(texture_atlas_layout.add(layout));
    next_state.set(GameState::GameInit);
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
