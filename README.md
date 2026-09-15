# bevy_tiled_coords

[![CI](https://github.com/james-kessler/bevy-tiled-coords/actions/workflows/ci.yml/badge.svg)](https://github.com/james-kessler/bevy-tiled-coords/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/bevy_tiled_coords/badge.svg)](https://docs.rs/bevy_tiled_coords)
[![Crates.io](https://img.shields.io/crates/v/bevy_tiled_coords.svg)](https://crates.io/crates/bevy_tiled_coords)

Coordinate helpers for [Tiled](https://www.mapeditor.org/) maps in [Bevy](https://bevyengine.org/) with [`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled).

The crate links three spaces:

- **Tiled** — layer cell indices and object pixel positions in the `.tmx` file
- **Tile** — logical cells as [`TilePos`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html)
- **World** — Bevy global positions for gameplay, pathfinding, and picking

## Isometric maps in the base libraries

[`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled) and [`bevy_ecs_tilemap`](https://docs.rs/bevy_ecs_tilemap) already render diamond isometric maps and expose core math (`TilePos::from_world_pos`, `TiledMapAsset::tile_relative_position`, and related APIs). In practice, isometric Tiled projects still need extra glue:

| Topic | What the stack gives you | Where projects often need help |
|-------|--------------------------|--------------------------------|
| Map orientation | Diamond isometric (`IsoCoordSystem::Diamond`) | Staggered isometric maps are unsupported in `bevy_ecs_tiled` 0.13 |
| Tiled objects | `object_relative_position` uses object pixel coordinates in map space | On diamond iso, Tiled’s **object grid** uses `object.x / tile_height` and `object.y / tile_height` for cell placement |
| Tile indices | `TilePos` on the logical grid | Tiled layer `y` and Bevy `TilePos.y` use opposite directions; you must flip when converting indices |
| World position | Tile centers via `tile_relative_position` | Picking uses the tilemap’s [`GlobalTransform`](https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html); layer and group offsets sit on parent entities |
| Tile art height | Tiles draw from `center - tile_size / 2` | Tall sprites on a short map grid need a separate **drawable** center for cursors and highlights |
| Layer offsets | Uniform translation on layer entities | Mixed tileset heights (16px terrain vs 32px floor) need a small Y correction per tilemap |
| Depth | `y_sort` on tilemap chunks | Freestanding sprites need the same Y-based Z formula as the renderer |

`bevy_tiled_coords` documents these coordinate spaces in one place and ships helpers for the rows above. It builds on the base libraries; it does not replace them.

## When to use this crate

Use it when you have:

- Diamond isometric `.tmx` maps on **Bevy 0.19** and **`bevy_ecs_tiled` 0.13**
- Gameplay logic on [`TilePos`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html) for a specific tilemap layer
- Tiled object layers for spawns, towers, or props tied to grid cells
- Mouse or touch picking that must return stable tile indices
- Mixed tileset heights or layer pixel offsets from Tiled

Staggered isometric maps are outside what `bevy_ecs_tiled` 0.13 loads.

## Coordinate pipeline

```text
Tiled object (x, y) pixels
       │  ÷ tile_height on diamond iso
       ▼
ObjectGridCell (tx, ty)
       │  to_tile_pos / tiled_layer_to_bevy_tile (Y flip)
       ▼
TilePos  ──TiledMapGeometry──►  world (GridCenter or DrawableCenter)
       ▲                           │
       └──── world_to_tile ────────┘
             (same TileWorldRole; uses tilemap GlobalTransform)
```

## Install

```toml
[dependencies]
bevy = "0.19.1"
bevy_ecs_tiled = "0.13.4"
bevy_tiled_coords = "0.2.0"
```

## Recipes

### Snap Tiled objects to the gameplay grid

**Why:** On diamond iso, spawn positions from raw object pixels can miss the cell your pathfinding tilemap uses. You want the object at the **grid cell center** on that tilemap.

Mark the gameplay [`TiledTilemap`](https://docs.rs/bevy_ecs_tiled) with [`GameplayTilemap`], build [`TiledMapGeometry`] via [`geometry_for_gameplay_tilemap`], then set layer-local translation:

```rust
use bevy::prelude::*;
use bevy_tiled_coords::{geometry_for_gameplay_tilemap, object_grid_anchor_local};

if let Some(local) = object_grid_anchor_local(
    &geometry,
    &object,
    layer_global,
    transform.translation.z,
) {
    transform.translation = local;
}
```

For the cell index: [`ObjectGridCell`] or `geometry.tile_pos_from_object(&object)`.

### Pick a tile under the cursor

**Why:** Picking must use the **gameplay** tilemap’s [`GlobalTransform`](https://docs.rs/bevy/latest/bevy/prelude/struct.GlobalTransform.html), and tall iso art needs **drawable** space so the highlight sits on visible tiles.

```rust
use bevy_tiled_coords::{TileWorldRole, TiledMapGeometry};

let tile = geometry.world_to_tile(cursor_world, TileWorldRole::DrawableCenter);
```

Use `TileWorldRole::GridCenter` when the result feeds pathfinding or placement logic.

### Pathfinding anchors vs visuals

**Why:** The same `TilePos` can map to two world points: logical grid center for A* and drawable center for UI.

```rust
use bevy_tiled_coords::{TileWorldRole, TiledMapGeometry};

let grid = geometry.tile_to_world(tile, TileWorldRole::GridCenter);
let draw = geometry.tile_to_world(tile, TileWorldRole::DrawableCenter);
```

Use one `TileWorldRole` for both `tile_to_world` and `world_to_tile` when you need round-trips.

### Layer offsets with mixed tileset heights

**Why:** Tiled layer `offset_y` moves every child tilemap equally; 16px terrain and 32px floor tiles need a per-tilemap Y correction to match the editor.

```rust
use bevy_tiled_coords::{accumulated_tiled_layer_offset, tilemap_height_offset_adjustment};

let offset = accumulated_tiled_layer_offset(&map_asset.map, layer_id).unwrap();
transform.translation.y += tilemap_height_offset_adjustment(&map_asset, offset, tile_size.y);
```

### Sort sprites with `y_sort` tilemaps

**Why:** Tilemaps adjust Z from world Y at render time; units and cursors need the same formula.

```rust
use bevy_tiled_coords::{IsoDepthSort, TiledCoordsPlugin};

app.add_plugins(TiledCoordsPlugin);
// On cursor entity: IsoDepthSort::cursor(layer_z)
```

Or use [`YSortDepth::z_at_world_y`] without ECS.

## Quick reference

| Task | API |
|------|-----|
| Object grid cell | `ObjectGridCell`, `iso_object_coords_to_tile` |
| Object → `TilePos` | `bevy_tile_pos_from_object_grid`, `TiledMapGeometry::tile_pos_from_object` |
| Tile ↔ world | `TiledMapGeometry`, `TilemapGeomParts`, `TileWorldRole` |
| Object snap | `object_grid_anchor_local`, `object_grid_anchor_world` |
| Layer / height fix | `LayerPixelOffset`, `accumulated_tiled_layer_offset`, `tilemap_height_offset_adjustment` |
| Depth | `y_sort_z_offset`, `YSortDepth`, `IsoDepthSort`, `tile_pick_depth` |
| ECS | `GameplayTilemap`, `TiledCoordsContext`, `TiledCoordsPlugin`, `TileDrawableQueries` |
| Hovered tile entity | `tile_drawable_at_entity`, `TileDrawableQueries::drawable_at` |

More detail and copy-paste snippets: [docs.rs/bevy_tiled_coords](https://docs.rs/bevy_tiled_coords) (crate-level **Recipes** section).

## Examples

Generate the test tile image once:

```text
python3 tests/fixtures/generate_tile.py
```

| Command | What it shows |
|---------|----------------|
| `cargo run --example object_grid_cell` | Tiled object pixels → `ObjectGridCell` → `TilePos` |
| `cargo run --example geometry_roles` | Same tile with `GridCenter` vs `DrawableCenter` and a round-trip |

## Documentation

- API reference: [docs.rs/bevy_tiled_coords](https://docs.rs/bevy_tiled_coords)
- Local docs: `cargo doc --open`

## License

Licensed under the [MIT license](LICENSE-MIT).
