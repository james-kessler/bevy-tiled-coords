//! Round-trip a tile position through world space with two roles.
//!
//! ```text
//! python3 tests/fixtures/generate_tile.py
//! cargo run --example geometry_roles
//! ```

use std::time::Duration;

use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowPlugin};
use bevy::winit::WinitPlugin;
use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{tile_drawable_y_offset, TileWorldRole, TiledMapGeometry};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: None,
                exit_condition: ExitCondition::DontExit,
                ..default()
            })
            .disable::<WinitPlugin>()
            .set(AssetPlugin {
                file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures").into(),
                ..default()
            }),
    )
    .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(16)))
    .add_plugins(TiledPlugin::default());

    let handle: Handle<TiledMapAsset> = app.world().resource::<AssetServer>().load("iso_map.tmx");

    for _ in 0..120 {
        app.update();
        if app
            .world()
            .resource::<Assets<TiledMapAsset>>()
            .get(&handle)
            .is_some()
        {
            break;
        }
    }

    let assets = app.world().resource::<Assets<TiledMapAsset>>();
    let asset = assets.get(&handle).expect("map asset");
    let anchor = TilemapAnchor::default();
    let grid = grid_size_from_map(&asset.map);
    let tile_size = TilemapTileSize {
        x: asset.map.tile_width as f32,
        y: asset.map.tile_height as f32,
    };
    let map_type = tilemap_type_from_map(&asset.map);
    let geometry = TiledMapGeometry::new(
        asset,
        &anchor,
        &grid,
        &tile_size,
        &map_type,
        &GlobalTransform::IDENTITY,
    );

    let tile = TilePos::new(3, 4);
    let grid_world = geometry.tile_to_world(tile, TileWorldRole::GridCenter);
    let draw_world = geometry.tile_to_world(tile, TileWorldRole::DrawableCenter);
    let offset = tile_drawable_y_offset(tile_size.y, asset.tilemap_size.y);

    println!("TilePos ({}, {})", tile.x, tile.y);
    println!("Grid center world: {}", grid_world.truncate());
    println!("Drawable center world: {}", draw_world.truncate());
    println!("Drawable y offset: {}", offset);

    let back = geometry
        .world_to_tile(grid_world.truncate(), TileWorldRole::GridCenter)
        .expect("round trip");
    println!("Round-trip tile: ({}, {})", back.x, back.y);
}
