use bevy::prelude::*;
use bevy_tiled_coords::{
    accumulated_tiled_layer_offset, mixed_height_layer_y_adjustment, y_sort_z_offset, IsoDepthSort,
    YSortDepth,
};

mod support;

use support::load_iso_fixture_map;

#[test]
fn accumulated_offset_includes_group() {
    let map = load_iso_fixture_map();
    let object_layer_id = 2u32;
    let offset = accumulated_tiled_layer_offset(&map, object_layer_id).unwrap();
    assert_eq!(offset.as_vec2(), Vec2::new(0.0, 16.0));
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

#[test]
fn y_sort_depth_matches_formula() {
    let depth = YSortDepth {
        layer_z: 1.0,
        map_size_y: 8,
        tile_height: 16.0,
        bias: 0.1,
    };
    let z = depth.z_at_world_y(48.0);
    assert!((z - (1.0 + y_sort_z_offset(48.0, 8, 16.0) + 0.1)).abs() < 1e-5);
}

#[test]
fn iso_depth_sort_uses_default_tile_height() {
    let sort = IsoDepthSort::entity(2.0);
    let z = sort.z_at_world_y(32.0, 10, 16.0);
    assert!((z - (2.0 + y_sort_z_offset(32.0, 10, 16.0) + IsoDepthSort::ENTITY_BIAS)).abs() < 1e-5);
}
