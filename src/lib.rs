//! # bevy_tiled_coords
//!
//! This crate connects three coordinate spaces used with [Tiled](https://www.mapeditor.org/)
//! maps in [Bevy](https://bevyengine.org/) through [`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled):
//!
//! | Space | Meaning |
//! |-------|---------|
//! | **Tiled** | Layer cell indices and object `x` / `y` in the map file |
//! | **Tile** | Logical grid cells as [`TilePos`] from [`bevy_ecs_tilemap`] |
//! | **World** | Bevy global positions for gameplay, picking, and sprites |
//!
//! ## Isometric object grid
//!
//! On diamond isometric maps, Tiled places many objects on an **object grid**. Cell indices come
//! from object pixels divided by map `tile_height`. See [`iso_object_coords_to_tile`].
//!
//! ## Y axis on tile indices
//!
//! Tiled layer `y` grows downward in the editor. Bevy [`TilePos`] `y` uses the tilemap convention.
//! [`tiled_layer_to_bevy_tile`] applies the flip.
//!
//! ## World points on a tile
//!
//! A tile can map to more than one world point. [`TileWorldRole`] picks the grid center for
//! pathfinding or the drawable center for cursors when tile art is taller than the map grid.
//!
//! ## Picking a tilemap
//!
//! Maps often have several [`TiledTilemap`] entities. Build [`TiledMapGeometry`] from the
//! tilemap you care about, then call [`TiledMapGeometry::world_to_tile`] or
//! [`object_grid_anchor_world`].

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_logo_url = "https://bevyengine.org/assets/icon.png")]

mod geometry;
mod layer;
mod picking;
mod tile_index;
mod y_sort;

pub use geometry::{
    tile_drawable_y_offset, tile_center_in_map_space, world_at_bevy_tile_center, TileWorldRole,
    TiledMapGeometry,
};
pub use layer::{
    accumulated_tiled_layer_offset, mixed_height_layer_y_adjustment,
    tilemap_height_offset_adjustment,
};
pub use picking::{
    object_grid_anchor_local, object_grid_anchor_world, object_path_anchor_world,
    object_tile_center_local_translation, tile_drawable_at_entity, TileDrawableAtEntity,
};
pub use tile_index::{
    bevy_tile_pos_from_object_grid, iso_object_coords_to_tile, tiled_layer_to_bevy_tile,
};
pub use y_sort::y_sort_z_offset;
