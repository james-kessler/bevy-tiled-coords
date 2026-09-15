use bevy::prelude::*;
use bevy_tiled_coords::{
    accumulated_tiled_layer_offset, mixed_height_layer_y_adjustment, y_sort_z_offset,
};

mod support;

use support::load_iso_fixture_map;

#[test]
fn accumulated_offset_includes_group() {
    let map = load_iso_fixture_map();
    let object_layer_id = 2u32;
    let offset = accumulated_tiled_layer_offset(&map, object_layer_id).unwrap();
    assert_eq!(offset, Vec2::new(0.0, 16.0));
}

#[test]
fn mixed_height_adjustment_zero_without_layer_y() {
    assert_eq!(mixed_height_layer_y_adjustment(0.0, 16.0, 32.0), 0.0);
}

#[test]
fn mixed_height_adjustment_short_tile_on_offset_layer() {
    assert_eq!(mixed_height_layer_y_adjustment(32.0, 16.0, 32.0), -8.0);
}

#[test]
fn y_sort_offset_matches_tilemap_formula() {
    let map_size_y = 8u32;
    let tile_height = 16.0;
    let world_y = 64.0;
    let z = y_sort_z_offset(world_y, map_size_y, tile_height);
    let expected = 1.0 - (world_y / (map_size_y as f32 * tile_height));
    assert!((z - expected).abs() < 1e-5);
}
