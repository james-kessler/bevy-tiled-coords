//! Print the tile cell for a TMX object on an isometric map.
//!
//! Run from the crate root:
//!
//! ```text
//! cargo run --example object_grid_cell
//! ```

use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{iso_object_coords_to_tile, tiled_layer_to_bevy_tile};
use tiled::{LayerType, Loader};

fn main() {
    let fixture_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures");
    let map = Loader::new()
        .load_str(
            include_str!("../tests/fixtures/iso_map.tmx"),
            &[fixture_dir],
        )
        .expect("load iso_map.tmx");

    let object = find_object(&map);
    let (tx, ty) = iso_object_coords_to_tile(&object, &map);
    let map_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let tile = tiled_layer_to_bevy_tile(tx, ty, &map_size).expect("cell on map");

    println!("Object pixel position: ({}, {})", object.x, object.y);
    println!("Object grid cell (iso): ({}, {})", tx, ty);
    println!("Bevy TilePos: ({}, {})", tile.x, tile.y);
}

fn find_object(map: &tiled::Map) -> tiled::Object<'_> {
    fn walk<'a>(layers: impl Iterator<Item = tiled::Layer<'a>>) -> Option<tiled::Object<'a>> {
        for layer in layers {
            if let Some(group) = layer.as_object_layer() {
                if let Some(object) = group.objects().next() {
                    return Some(object);
                }
            }
            if let LayerType::Group(nested) = layer.layer_type() {
                if let Some(object) = walk(nested.layers()) {
                    return Some(object);
                }
            }
        }
        None
    }
    walk(map.layers()).expect("fixture includes one object")
}
