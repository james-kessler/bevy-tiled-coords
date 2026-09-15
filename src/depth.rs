//! Freestanding sprites that must sort with `y_sort` tilemaps.

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::y_sort::y_sort_z_offset;

/// Inputs for [`y_sort_z_offset`] plus layer base Z and a small bias.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct YSortDepth {
    pub layer_z: f32,
    pub map_size_y: u32,
    pub tile_height: f32,
    pub bias: f32,
}

impl YSortDepth {
    /// World Z that matches tilemap `y_sort` at `world_y`.
    pub fn z_at_world_y(&self, world_y: f32) -> f32 {
        self.layer_z + y_sort_z_offset(world_y, self.map_size_y, self.tile_height) + self.bias
    }
}

/// Bakes isometric y-sort into [`Transform::translation`] z for sprites.
#[derive(Component, Clone, Copy, Debug)]
pub struct IsoDepthSort {
    /// Global z of the hovered tilemap layer (or parent layer entity).
    pub layer_z: f32,
    /// Tilemap tile height in the y-sort formula (`0` = map header `tile_height`).
    pub tile_height: f32,
    /// Small offset above peers on the same layer and y position.
    pub bias: f32,
}

impl IsoDepthSort {
    pub const CURSOR_BIAS: f32 = 0.1;
    pub const ENTITY_BIAS: f32 = 0.1;

    pub fn cursor(layer_z: f32) -> Self {
        Self {
            layer_z,
            tile_height: 0.0,
            bias: Self::CURSOR_BIAS,
        }
    }

    pub fn entity(layer_z: f32) -> Self {
        Self {
            layer_z,
            tile_height: 0.0,
            bias: Self::ENTITY_BIAS,
        }
    }

    /// Resolves `tile_height` when the component stores `0`.
    pub fn z_at_world_y(&self, world_y: f32, map_size_y: u32, default_tile_height: f32) -> f32 {
        let tile_height = if self.tile_height > 0.0 {
            self.tile_height
        } else {
            default_tile_height
        };
        YSortDepth {
            layer_z: self.layer_z,
            map_size_y,
            tile_height,
            bias: self.bias,
        }
        .z_at_world_y(world_y)
    }
}

/// Pick depth for [bevy_picking](https://docs.rs/bevy/latest/bevy/picking/index.html) backends — matches isometric tile render order.
///
/// Lower depth sorts first; front-most tiles should have the most negative depth.
pub fn tile_pick_depth(
    world_y: f32,
    tilemap_global_z: f32,
    tile_height: f32,
    map_size_y: u32,
) -> f32 {
    -IsoDepthSort {
        layer_z: tilemap_global_z,
        tile_height,
        bias: 0.0,
    }
    .z_at_world_y(world_y, map_size_y, tile_height)
}

/// Runs in [`PostUpdate`] after movement so z tracks the latest x/y.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct IsoDepthSortSet;

fn apply_iso_depth_sort(
    maps: Query<&TiledMap>,
    map_assets: Res<Assets<TiledMapAsset>>,
    mut sprites: Query<(&mut Transform, &IsoDepthSort)>,
) {
    let Ok(tiled_map) = maps.single() else {
        return;
    };
    let Some(map_asset) = map_assets.get(&tiled_map.0) else {
        return;
    };
    let map_size_y = map_asset.tilemap_size.y;
    let default_tile_height = map_asset.map.tile_height as f32;

    for (mut transform, depth) in &mut sprites {
        transform.translation.z = depth.z_at_world_y(
            transform.translation.y,
            map_size_y,
            default_tile_height,
        );
    }
}

/// Applies [`IsoDepthSort`] to transforms each frame.
pub struct IsoDepthPlugin;

impl Plugin for IsoDepthPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, IsoDepthSortSet)
            .add_systems(PostUpdate, apply_iso_depth_sort.in_set(IsoDepthSortSet));
    }
}
