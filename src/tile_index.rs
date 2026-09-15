//! Tiled layer indices and object pixels to [`TilePos`].

use bevy_ecs_tiled::prelude::*;

use crate::geometry::TiledMapGeometry;
use crate::object_grid::ObjectGridCell;

/// Tile grid index from a TMX object's position on a diamond isometric map.
///
/// Tiled uses the object grid: divide `object.x` and `object.y` by map `tile_height`, then floor.
///
/// On a map with `tile_height` 16, an object at pixel (32, 32) maps to cell (2, 2).
pub fn iso_object_coords_to_tile(object: &tiled::Object, map: &tiled::Map) -> ObjectGridCell {
    let th = map.tile_height as f32;
    ObjectGridCell {
        x: (object.x / th).floor() as i32,
        y: (object.y / th).floor() as i32,
    }
}

/// Converts a Tiled layer cell index to Bevy [`TilePos`].
///
/// `tiled_x` and `tiled_y` are the indices stored in the layer data. The function flips `y` to
/// match the tilemap.
///
/// # Example
///
/// ```
/// use bevy_ecs_tiled::prelude::*;
/// use bevy_tiled_coords::tiled_layer_to_bevy_tile;
///
/// let map_size = TilemapSize { x: 10, y: 20 };
/// let tile = tiled_layer_to_bevy_tile(3, 5, &map_size).unwrap();
/// assert_eq!(tile.x, 3);
/// assert_eq!(tile.y, 14);
/// ```
pub fn tiled_layer_to_bevy_tile(
    tiled_x: i32,
    tiled_y: i32,
    map_size: &TilemapSize,
) -> Option<TilePos> {
    let bevy_y = map_size.y as i32 - 1 - tiled_y;
    TilePos::from_i32_pair(tiled_x, bevy_y, map_size)
}

/// [`TilePos`] for the grid cell that contains a TMX object.
///
/// Isometric maps use [`iso_object_coords_to_tile`]. Orthogonal maps divide by `tile_width` and
/// `tile_height`. The result always passes through [`tiled_layer_to_bevy_tile`].
pub fn bevy_tile_pos_from_object_grid(
    map_asset: &TiledMapAsset,
    object: &tiled::Object,
) -> Option<TilePos> {
    object_grid_cell_from_object(map_asset, object)
        .and_then(|cell| cell.to_tile_pos(&map_asset.tilemap_size))
}

/// Object grid cell before the Y-flip to [`TilePos`].
pub fn object_grid_cell_from_object(
    map_asset: &TiledMapAsset,
    object: &tiled::Object,
) -> Option<ObjectGridCell> {
    let cell = match tilemap_type_from_map(&map_asset.map) {
        TilemapType::Isometric(_) => iso_object_coords_to_tile(object, &map_asset.map),
        _ => ObjectGridCell {
            x: (object.x / map_asset.map.tile_width as f32).floor() as i32,
            y: (object.y / map_asset.map.tile_height as f32).floor() as i32,
        },
    };
    Some(cell)
}

impl TiledMapGeometry<'_> {
    /// [`TilePos`] on this tilemap for a TMX object.
    pub fn tile_pos_from_object(&self, object: &tiled::Object) -> Option<TilePos> {
        bevy_tile_pos_from_object_grid(self.map_asset, object)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn y_flip_matches_bevy_tile_storage() {
        let map_size = TilemapSize { x: 8, y: 8 };
        let tile = tiled_layer_to_bevy_tile(2, 2, &map_size).unwrap();
        assert_eq!(tile, TilePos::new(2, 5));
    }
}
