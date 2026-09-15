//! # bevy_tiled_coords
//!
//! This crate connects three coordinate spaces used with [Tiled](https://www.mapeditor.org/)
//! maps in [Bevy](https://bevyengine.org/) through [`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled):
//!
//! | Space | Meaning |
//! |-------|---------|
//! | **Tiled** | Layer cell indices and object `x` / `y` in the map file |
//! | **Tile** | Logical grid cells as [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html) from [bevy_ecs_tilemap](https://docs.rs/bevy_ecs_tilemap) |
//! | **World** | Bevy global positions for gameplay, picking, and sprites |
//!
//! ## Isometric maps in the base libraries
//!
//! [`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled) and [`bevy_ecs_tilemap`](https://docs.rs/bevy_ecs_tilemap)
//! load and draw **diamond** isometric maps. [TilePos::from_world_pos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html#method.from_world_pos) and
//! [`TiledMapAsset::tile_relative_position`](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/tiled/map/asset/struct.TiledMapAsset.html#method.tile_relative_position)
//! cover much of the tile ↔ world math.
//!
//! Real Tiled isometric games still hit gaps that the crates solve piece by piece:
//!
//! - **Staggered** isometric orientation is unsupported in `bevy_ecs_tiled` 0.13; use diamond maps or
//!   another loader.
//! - **Tiled objects** on diamond maps follow an object **grid** (pixels ÷ `tile_height`). Spawn
//!   helpers that use raw object pixels alone can miss that grid.
//! - **Layer indices** in the `.tmx` file use a different Y direction than [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html); convert with
//!   [`tiled_layer_to_bevy_tile`].
//! - **World picking** must go through each [TiledTilemap](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/prelude/struct.TiledTilemap.html) [GlobalTransform](https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html), including layer
//!   offsets on parent entities.
//! - **Mixed tile heights** and **layer offsets** interact: shorter tilesets sit higher on the cell,
//!   so a uniform layer lift may need [`tilemap_height_offset_adjustment`].
//! - **`y_sort`** depth is applied inside the tilemap renderer; sprites use [`y_sort_z_offset`] to
//!   match.
//!
//! This crate collects those rules and the helpers that apply them.
//!
//! ## Isometric object grid
//!
//! On diamond isometric maps, Tiled places many objects on an **object grid**. Cell indices come
//! from object pixels divided by map `tile_height`. See [`iso_object_coords_to_tile`].
//!
//! ## Y axis on tile indices
//!
//! Tiled layer `y` grows downward in the editor. Bevy [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html) `y` uses the tilemap convention.
//! [`tiled_layer_to_bevy_tile`] applies the flip.
//!
//! ## World points on a tile
//!
//! A tile can map to more than one world point. [`TileWorldRole`] picks the grid center for
//! pathfinding or the drawable center for cursors when tile art is taller than the map grid.
//!
//! ## Picking a tilemap
//!
//! Maps often have several [TiledTilemap](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/prelude/struct.TiledTilemap.html) entities. Build [`TiledMapGeometry`] from the
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
    tile_center_in_map_space, tile_drawable_y_offset, world_at_bevy_tile_center, TileWorldRole,
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
