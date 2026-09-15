//! [`TilePos`] and world space for one [`TiledTilemap`].

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

/// Which point on a tile a conversion uses.
///
/// Use the same role for `tile_to_world` and `world_to_tile` so positions round-trip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileWorldRole {
    /// Center of the logical grid cell. Use for pathfinding and entity anchors.
    GridCenter,
    /// Center of the visible tile art. Use for cursors and hover highlights.
    DrawableCenter,
}

fn bevy_tile_at_world_pos(
    world_xy: Vec2,
    tilemap_global: &GlobalTransform,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    tile_size: &TilemapTileSize,
    map_type: &TilemapType,
    anchor: &TilemapAnchor,
) -> Option<TilePos> {
    let local = tilemap_global
        .affine()
        .inverse()
        .transform_point3(world_xy.extend(0.0))
        .truncate();
    TilePos::from_world_pos(&local, map_size, grid_size, tile_size, map_type, anchor)
}

/// Extra world `y` for sprites that should sit on the visible tile surface.
///
/// [`bevy_ecs_tilemap`] draws isometric tiles from `center - tile_size / 2`. When `tile_height`
/// is larger than the map grid height, art extends above the diamond center. This value is half
/// of that extra height.
pub fn tile_drawable_y_offset(tile_height: f32, map_tile_height: u32) -> f32 {
    (tile_height - map_tile_height as f32).max(0.0) / 2.0
}

/// Map-local center of a tile in tilemap space.
///
/// Pass the owning tilemap's [`TilemapTileSize`]. Tilesets with different heights change the
/// anchor offset.
pub fn tile_center_in_map_space(
    map_asset: &TiledMapAsset,
    anchor: &TilemapAnchor,
    bevy_tile: TilePos,
    tile_size: &TilemapTileSize,
) -> Vec2 {
    map_asset.tile_relative_position(&bevy_tile, tile_size, anchor)
}

/// World-space center of a tile on a specific tilemap entity.
pub fn world_at_bevy_tile_center(
    map_asset: &TiledMapAsset,
    anchor: &TilemapAnchor,
    bevy_tile: TilePos,
    tile_size: &TilemapTileSize,
    tilemap_global: &GlobalTransform,
) -> Vec3 {
    let local = tile_center_in_map_space(map_asset, anchor, bevy_tile, tile_size);
    tilemap_global.transform_point(local.extend(0.0))
}

/// Geometry for one loaded map and one [`TiledTilemap`] transform.
///
/// Store this on a resource or build it in a system when you know which tilemap layer you use.
#[derive(Clone, Copy)]
pub struct TiledMapGeometry<'a> {
    pub map_asset: &'a TiledMapAsset,
    pub anchor: &'a TilemapAnchor,
    pub grid_size: &'a TilemapGridSize,
    pub tile_size: &'a TilemapTileSize,
    pub map_type: &'a TilemapType,
    pub tilemap_global: &'a GlobalTransform,
}

impl<'a> TiledMapGeometry<'a> {
    /// Builds a geometry handle from the current tilemap components.
    pub fn new(
        map_asset: &'a TiledMapAsset,
        anchor: &'a TilemapAnchor,
        grid_size: &'a TilemapGridSize,
        tile_size: &'a TilemapTileSize,
        map_type: &'a TilemapType,
        tilemap_global: &'a GlobalTransform,
    ) -> Self {
        Self {
            map_asset,
            anchor,
            grid_size,
            tile_size,
            map_type,
            tilemap_global,
        }
    }

    fn drawable_y_offset(&self) -> f32 {
        tile_drawable_y_offset(self.tile_size.y, self.map_asset.tilemap_size.y)
    }

    /// Tile index to world position for the chosen [`TileWorldRole`].
    pub fn tile_to_world(&self, tile: TilePos, role: TileWorldRole) -> Vec3 {
        let mut world = world_at_bevy_tile_center(
            self.map_asset,
            self.anchor,
            tile,
            self.tile_size,
            self.tilemap_global,
        );
        if role == TileWorldRole::DrawableCenter {
            world.y += self.drawable_y_offset();
        }
        world
    }

    /// World `xy` to tile index for the chosen [`TileWorldRole`].
    pub fn world_to_tile(&self, world_xy: Vec2, role: TileWorldRole) -> Option<TilePos> {
        let adjusted = match role {
            TileWorldRole::GridCenter => world_xy,
            TileWorldRole::DrawableCenter => Vec2::new(
                world_xy.x,
                world_xy.y - self.drawable_y_offset(),
            ),
        };
        bevy_tile_at_world_pos(
            adjusted,
            self.tilemap_global,
            &self.map_asset.tilemap_size,
            self.grid_size,
            self.tile_size,
            self.map_type,
            self.anchor,
        )
    }
}
