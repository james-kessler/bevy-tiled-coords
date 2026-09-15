//! Tiled object grid indices before the layer Y-flip to [`TilePos`].

use bevy_ecs_tiled::prelude::*;

use crate::tile_index::tiled_layer_to_bevy_tile;

/// Cell on Tiled's object grid (pixels ÷ tile size), **before** [`tiled_layer_to_bevy_tile`].
///
/// Do not confuse with [`TilePos`]: convert with [`ObjectGridCell::to_tile_pos`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjectGridCell {
    pub x: i32,
    pub y: i32,
}

impl ObjectGridCell {
    /// Applies the Tiled → Bevy Y flip and bounds check.
    pub fn to_tile_pos(self, map_size: &TilemapSize) -> Option<TilePos> {
        tiled_layer_to_bevy_tile(self.x, self.y, map_size)
    }
}
