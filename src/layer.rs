//! Tiled layer offsets and mixed tileset heights.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Sum of Tiled layer and group `offset_x` / `offset_y` in **pixels** (not world space).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LayerPixelOffset(Vec2);

impl LayerPixelOffset {
    /// Zero offset.
    pub const ZERO: Self = Self(Vec2::ZERO);

    pub fn new(offset: Vec2) -> Self {
        Self(offset)
    }

    pub fn as_vec2(self) -> Vec2 {
        self.0
    }

    pub fn x(self) -> f32 {
        self.0.x
    }

    pub fn y(self) -> f32 {
        self.0.y
    }
}

/// Sum of `offset_x` and `offset_y` from the map root down to a layer, including groups.
pub fn accumulated_tiled_layer_offset(map: &tiled::Map, layer_id: u32) -> Option<LayerPixelOffset> {
    fn walk<'a>(
        layers: impl Iterator<Item = tiled::Layer<'a>>,
        target_id: u32,
        acc: Vec2,
    ) -> Option<LayerPixelOffset> {
        for layer in layers {
            let layer_acc = acc + Vec2::new(layer.offset_x, layer.offset_y);
            if layer.id() == target_id {
                return Some(LayerPixelOffset(layer_acc));
            }
            if let tiled::LayerType::Group(group) = layer.layer_type() {
                if let Some(found) = walk(group.layers(), target_id, layer_acc) {
                    return Some(found);
                }
            }
        }
        None
    }

    walk(map.layers(), layer_id, Vec2::ZERO)
}

/// Y adjustment when a layer has a vertical offset and tilesets use different heights.
///
/// `layer_offset_y` comes from [`accumulated_tiled_layer_offset`]. `tallest_tile_height` is
/// usually `map_asset.largest_tile_size.y`.
pub fn mixed_height_layer_y_adjustment(
    layer_offset_y: f32,
    tile_height: f32,
    tallest_tile_height: f32,
) -> f32 {
    if layer_offset_y == 0.0 {
        return 0.0;
    }
    -((tallest_tile_height - tile_height).max(0.0) / 2.0)
}

/// Same as [`mixed_height_layer_y_adjustment`] using map metadata for the tallest tile height.
pub fn tilemap_height_offset_adjustment(
    map_asset: &TiledMapAsset,
    accumulated_layer_offset: LayerPixelOffset,
    tile_height: f32,
) -> f32 {
    mixed_height_layer_y_adjustment(
        accumulated_layer_offset.y(),
        tile_height,
        map_asset.largest_tile_size.y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_tile_gets_negative_adjustment() {
        assert_eq!(mixed_height_layer_y_adjustment(10.0, 16.0, 32.0), -8.0);
    }
}
