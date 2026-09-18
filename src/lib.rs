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
//!
//! ## When to use this crate
//!
//! Reach for `bevy_tiled_coords` when your project matches most of these:
//!
//! - Diamond isometric `.tmx` maps loaded with `bevy_ecs_tiled` 0.13 on Bevy 0.19.
//! - Gameplay uses [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html) on a specific tilemap layer (terrain, collision, and similar).
//! - Tiled **object layers** define spawn points, towers, or props that should sit on grid cells.
//! - You pick tiles with the mouse or touch and need stable cell indices under the cursor.
//! - Tile art height differs from the map grid height, or layers use pixel offsets in Tiled.
//!
//! Staggered isometric maps fall outside what `bevy_ecs_tiled` 0.13 loads; this crate targets
//! diamond isometric maps.
//!
//! ## Coordinate pipeline
//!
//! ```text
//! Tiled object (x, y) pixels
//!        │  ÷ tile_height on diamond iso
//!        ▼
//! Object grid cell (tx, ty)
//!        │  tiled_layer_to_bevy_tile (Y flip)
//!        ▼
//! TilePos  ──TiledMapGeometry──►  world (GridCenter or DrawableCenter)
//!        ▲                           │
//!        └──── world_to_tile ────────┘
//!              (same TileWorldRole; uses tilemap GlobalTransform)
//! ```
//!
//! ## Recipes
//!
//! Each recipe states a goal, then shows the usual helper. Wire queries and assets to match your
//! map hierarchy.
//!
//! ### Snap Tiled objects to the gameplay grid
//!
//! **Goal:** After load, a point object should sit at the **center of its grid cell** on the
//! tilemap you use for pathfinding (not only at raw Tiled pixel coordinates).
//!
//! Build [`TiledMapGeometry`] from that tilemap’s components, then move the object in **layer**
//! space:
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_ecs_tiled::prelude::*;
//! use bevy_tiled_coords::{object_grid_anchor_local, TiledMapGeometry, TileWorldRole};
//!
//! fn snap_object(
//!     object: &tiled::Object,
//!     geometry: &TiledMapGeometry,
//!     layer_global: &GlobalTransform,
//!     transform: &mut Transform,
//! ) {
//!     if let Some(local) = object_grid_anchor_local(
//!         &geometry,
//!         object,
//!         layer_global,
//!         transform.translation.z,
//!     ) {
//!         transform.translation = local;
//!     }
//! }
//! ```
//!
//! Cell index only: [`bevy_tile_pos_from_object_grid`]. World anchor on the tilemap:
//! [`object_grid_anchor_world`] or [`object_path_anchor_world`].
//!
//! ### Pick a tile under the cursor
//!
//! **Goal:** Convert camera world position to a [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html) on the **gameplay** tilemap (the one
//! your pathfinding uses).
//!
//! 1. `camera.viewport_to_world_2d` → world `Vec2`.
//! 2. Build [`TiledMapGeometry`] from that tilemap’s `GlobalTransform` and sizes.
//! 3. Call [`TiledMapGeometry::world_to_tile`] with [`TileWorldRole::DrawableCenter`] for hover
//!    highlights on tall iso art, or [`TileWorldRole::GridCenter`] for logic.
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_ecs_tiled::prelude::*;
//! use bevy_tiled_coords::{TileWorldRole, TiledMapGeometry};
//!
//! fn tile_under_cursor(
//!     cursor_world: Vec2,
//!     geometry: &TiledMapGeometry,
//! ) -> Option<TilePos> {
//!     geometry.world_to_tile(cursor_world, TileWorldRole::DrawableCenter)
//! }
//! ```
//!
//! For a hovered [TiledTile](https://docs.rs/bevy_ecs_tiled/latest/bevy_ecs_tiled/prelude/struct.TiledTile.html) entity, [`tile_drawable_at_entity`] returns drawable world position
//! and layer depth hints.
//!
//! ### Pathfinding anchors vs visuals
//!
//! **Goal:** A* and tower placement use **grid** centers; cursors and selection rings use
//! **drawable** centers on the same [TilePos](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html).
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_ecs_tiled::prelude::TilePos;
//! use bevy_tiled_coords::{TileWorldRole, TiledMapGeometry};
//!
//! fn anchor_points(geometry: &TiledMapGeometry, tile: TilePos) -> (Vec3, Vec3) {
//!     let grid = geometry.tile_to_world(tile, TileWorldRole::GridCenter);
//!     let draw = geometry.tile_to_world(tile, TileWorldRole::DrawableCenter);
//!     (grid, draw)
//! }
//! ```
//!
//! Use the same [`TileWorldRole`] for `tile_to_world` and `world_to_tile` so positions round-trip.
//!
//! ### Layer offsets with mixed tileset heights
//!
//! **Goal:** A Tiled layer has `offset_y`, and tilemaps under it mix 16px-tall terrain with
//! 32px-tall floor tiles. Apply a small per-tilemap Y tweak so short tiles line up with Tiled.
//!
//! ```rust,no_run
//! use bevy::prelude::*;
//! use bevy_ecs_tiled::prelude::*;
//! use bevy_tiled_coords::{accumulated_tiled_layer_offset, tilemap_height_offset_adjustment};
//!
//! fn apply_height_fixup(
//!     map_asset: &TiledMapAsset,
//!     layer_id: u32,
//!     tile_size: &TilemapTileSize,
//!     transform: &mut Transform,
//! ) {
//!     let offset = accumulated_tiled_layer_offset(&map_asset.map, layer_id).unwrap();
//!     let y_fix = tilemap_height_offset_adjustment(map_asset, offset, tile_size.y); // offset: LayerPixelOffset
//!     transform.translation.y += y_fix;
//! }
//! ```
//!
//! Pure math without the asset: [`mixed_height_layer_y_adjustment`].
//!
//! ### Sort freestanding sprites with `y_sort` tilemaps
//!
//! **Goal:** Units or cursors draw in the same depth order as isometric tiles when
//! `TilemapRenderSettings::y_sort` is enabled.
//!
//! ```rust,no_run
//! use bevy_tiled_coords::{IsoDepthSort, TiledCoordsPlugin};
//! use bevy::prelude::*;
//!
//! fn setup(app: &mut App) {
//!     app.add_plugins(TiledCoordsPlugin);
//!     // Spawn cursor with IsoDepthSort::cursor(layer_z)
//! }
//! ```
//!
//! Or call [`YSortDepth::z_at_world_y`] / [`y_sort_z_offset`] directly without ECS.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(html_logo_url = "https://bevyengine.org/assets/icon.png")]

mod depth;
mod ecs;
mod geometry;
mod layer;
mod object_grid;
mod picking;
mod tile_index;
mod y_sort;

pub use depth::{tile_pick_depth, IsoDepthPlugin, IsoDepthSort, IsoDepthSortSet, YSortDepth};
pub use ecs::{
    geometry_for_gameplay_tilemap, geometry_from_context, refresh_tiled_coords_context,
    tiled_map_geometry_for_entity, GameplayTilemap, ObjectGridAnchored, TileDrawableQueries,
    TiledCoordsContext, TiledCoordsPlugin, TilemapHeightAdjusted,
};
pub use geometry::{
    tile_center_in_map_space, tile_drawable_y_offset, world_at_bevy_tile_center, TileWorldRole,
    TiledMapGeometry, TilemapGeomParts,
};
pub use layer::{
    accumulated_tiled_layer_offset, mixed_height_layer_y_adjustment,
    tilemap_height_offset_adjustment, LayerPixelOffset,
};
pub use object_grid::ObjectGridCell;
pub use picking::{
    object_grid_anchor_local, object_grid_anchor_world, object_path_anchor_world,
    tile_drawable_at_entity, TileDrawableAtEntity,
};
pub use tile_index::{
    bevy_tile_pos_from_object_grid, iso_object_coords_to_tile, object_grid_cell_from_object,
    tiled_layer_to_bevy_tile,
};
pub use y_sort::y_sort_z_offset;
