use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{iso_object_coords_to_tile, ObjectGridCell, tiled_layer_to_bevy_tile};

mod common;
mod support;

use common::first_object_in_map;
use support::load_iso_fixture_map;

#[test]
fn tiled_layer_y_flip_into_bevy_tile() {
    let map_size = TilemapSize { x: 10, y: 20 };
    let tile = tiled_layer_to_bevy_tile(3, 5, &map_size).unwrap();
    assert_eq!(tile.x, 3);
    assert_eq!(tile.y, 14);
}

#[test]
fn tiled_layer_rejects_out_of_bounds() {
    let map_size = TilemapSize { x: 4, y: 4 };
    assert!(tiled_layer_to_bevy_tile(-1, 0, &map_size).is_none());
    assert!(tiled_layer_to_bevy_tile(0, 4, &map_size).is_none());
}

#[test]
fn iso_object_grid_from_fixture() {
    let map = load_iso_fixture_map();
    let object = first_object_in_map(&map);
    assert_eq!(
        iso_object_coords_to_tile(&object, &map),
        ObjectGridCell { x: 2, y: 2 }
    );
}

#[test]
fn object_grid_indices_align_with_layer_flip() {
    let map = load_iso_fixture_map();
    let object = first_object_in_map(&map);
    let cell = iso_object_coords_to_tile(&object, &map);
    let map_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let tile = cell.to_tile_pos(&map_size).unwrap();
    assert_eq!(tile, TilePos::new(2, 5));
}
