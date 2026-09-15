//! World anchors for objects and hovered tiles.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::geometry::{TileWorldRole, TiledMapGeometry};

/// World position and depth hints for drawing on a hovered tile.
pub struct TileDrawableAtEntity {
    pub world: Vec3,
    pub layer_z: f32,
    pub tile_height: f32,
}

/// Drawable center in world space for a [TiledTile](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/prelude/struct.TiledTile.html) entity.
pub fn tile_drawable_at_entity(
    tile_entity: Entity,
    tiles: &Query<(&TilePos, &ChildOf), With<TiledTile>>,
    tilemaps: &Query<
        (
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &GlobalTransform,
        ),
        With<TiledTilemap>,
    >,
    maps: &Query<(&TiledMap, &TilemapAnchor)>,
    map_assets: &Assets<TiledMapAsset>,
) -> Option<TileDrawableAtEntity> {
    let (tile_pos, child_of) = tiles.get(tile_entity).ok()?;
    let (grid_size, tile_size, map_type, tilemap_global) = tilemaps.get(child_of.parent()).ok()?;
    let (tiled_map, anchor) = maps.single().ok()?;
    let map_asset = map_assets.get(&tiled_map.0)?;
    let geometry = TiledMapGeometry::new(
        map_asset,
        anchor,
        grid_size,
        tile_size,
        map_type,
        tilemap_global,
    );
    let world = geometry.tile_to_world(*tile_pos, TileWorldRole::DrawableCenter);
    Some(TileDrawableAtEntity {
        world,
        layer_z: tilemap_global.translation().z,
        tile_height: tile_size.y,
    })
}

/// Grid-center world position for a TMX object on the given tilemap geometry.
pub fn object_grid_anchor_world(
    geometry: &TiledMapGeometry,
    object: &tiled::Object,
) -> Option<Vec3> {
    let tile = geometry.tile_pos_from_object(object)?;
    Some(geometry.tile_to_world(tile, TileWorldRole::GridCenter))
}

/// Grid-center world position for a TMX object (same as [`object_grid_anchor_world`]).
pub fn object_path_anchor_world(
    geometry: &TiledMapGeometry,
    object: &tiled::Object,
) -> Option<Vec3> {
    object_grid_anchor_world(geometry, object)
}

/// Layer-local translation that places an object at its grid cell anchor.
pub fn object_grid_anchor_local(
    geometry: &TiledMapGeometry,
    object: &tiled::Object,
    layer_global: &GlobalTransform,
    preserve_z: f32,
) -> Option<Vec3> {
    let world = object_grid_anchor_world(geometry, object)?;
    let mut local = layer_global.affine().inverse().transform_point3(world);
    local.z = preserve_z;
    Some(local)
}
