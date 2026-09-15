//! Loads `iso_map.tmx` through Bevy's asset pipeline.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{bevy_tile_pos_from_object_grid, TileWorldRole, TiledMapGeometry};

mod common;
use common::first_object_in_map;

fn load_fixture_asset() -> (App, Handle<TiledMapAsset>) {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        file_path: concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures").into(),
        ..default()
    }))
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
    (app, handle)
}

#[test]
fn bevy_tile_pos_from_loaded_map() {
    let (app, handle) = load_fixture_asset();
    let assets = app.world().resource::<Assets<TiledMapAsset>>();
    let asset = assets.get(&handle).expect("map asset should load");
    let object = first_object_in_map(&asset.map);
    let tile = bevy_tile_pos_from_object_grid(asset, &object).unwrap();
    assert_eq!(tile, TilePos::new(2, 5));
}

#[test]
fn geometry_grid_round_trip_on_loaded_map() {
    let (app, handle) = load_fixture_asset();
    let assets = app.world().resource::<Assets<TiledMapAsset>>();
    let asset = assets.get(&handle).expect("map asset should load");
    let anchor = TilemapAnchor::default();
    let grid = grid_size_from_map(&asset.map);
    let tile_size = TilemapTileSize {
        x: asset.map.tile_width as f32,
        y: asset.map.tile_height as f32,
    };
    let map_type = tilemap_type_from_map(&asset.map);
    let global = GlobalTransform::IDENTITY;
    let geometry = TiledMapGeometry::new(asset, &anchor, &grid, &tile_size, &map_type, &global);
    let tile = TilePos::new(3, 4);
    let world = geometry.tile_to_world(tile, TileWorldRole::GridCenter);
    let back = geometry
        .world_to_tile(world.truncate(), TileWorldRole::GridCenter)
        .expect("tile under world point");
    assert_eq!(back, tile);
}
