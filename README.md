# bevy_tiled_coords

[![CI](https://github.com/james-kessler/bevy-tiled-coords/actions/workflows/ci.yml/badge.svg)](https://github.com/james-kessler/bevy-tiled-coords/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/bevy_tiled_coords/badge.svg)](https://docs.rs/bevy_tiled_coords)
[![Crates.io](https://img.shields.io/crates/v/bevy_tiled_coords.svg)](https://crates.io/crates/bevy_tiled_coords)

Coordinate helpers for [Tiled](https://www.mapeditor.org/) maps in [Bevy](https://bevyengine.org/) with [`bevy_ecs_tiled`](https://docs.rs/bevy_ecs_tiled).

The crate links three spaces:

- **Tiled** — layer cell indices and object pixel positions in the `.tmx` file
- **Tile** — logical cells as [`TilePos`](https://docs.rs/bevy_ecs_tilemap/latest/bevy_ecs_tilemap/tiles/struct.TilePos.html)
- **World** — Bevy global positions for gameplay, pathfinding, and picking

## Install

```toml
[dependencies]
bevy = "0.19.1"
bevy_ecs_tiled = "0.13.4"
bevy_tiled_coords = "0.1.0"
```

## Quick start

### Object grid cell to `TilePos`

On diamond isometric maps, divide object `x` and `y` by map `tile_height`, then flip the layer `y` index:

```rust
use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{bevy_tile_pos_from_object_grid, tiled_layer_to_bevy_tile};

// After your map asset is loaded:
let tile = bevy_tile_pos_from_object_grid(&map_asset, &object).expect("on map");

// Or step by step:
let (tx, ty) = bevy_tiled_coords::iso_object_coords_to_tile(&object, &map_asset.map);
let tile = tiled_layer_to_bevy_tile(tx, ty, &map_asset.tilemap_size).expect("on map");
```

### Tile ↔ world on one tilemap

Build [`TiledMapGeometry`](https://docs.rs/bevy_tiled_coords/latest/bevy_tiled_coords/struct.TiledMapGeometry.html) from the tilemap entity you use for gameplay (terrain, floor, and so on):

```rust
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::{TileWorldRole, TiledMapGeometry};

let geometry = TiledMapGeometry::new(
    &map_asset,
    &anchor,
    grid_size,
    tile_size,
    map_type,
    tilemap_global,
);

let world = geometry.tile_to_world(tile_pos, TileWorldRole::GridCenter);
let picked = geometry.world_to_tile(cursor_world, TileWorldRole::DrawableCenter);
```

Use `GridCenter` for path anchors. Use `DrawableCenter` when tile art is taller than the map grid cell.

### Layer offsets and mixed tile heights

```rust
use bevy::prelude::*;
use bevy_tiled_coords::{
    accumulated_tiled_layer_offset, tilemap_height_offset_adjustment,
};

let layer_offset = accumulated_tiled_layer_offset(&map_asset.map, layer_id).unwrap();
let y_fix = tilemap_height_offset_adjustment(&map_asset, layer_offset, tile_size.y);
transform.translation.y += y_fix;
```

### Depth sort with tilemaps

When `TilemapRenderSettings::y_sort` is on, match tile depth for freestanding sprites:

```rust
use bevy_tiled_coords::y_sort_z_offset;

let z = layer_z + y_sort_z_offset(world_y, map_asset.tilemap_size.y, tile_size.y);
```

## Examples

Generate the test tile image once (examples load the same fixture map):

```text
python3 tests/fixtures/generate_tile.py
cargo run --example object_grid_cell
cargo run --example geometry_roles
```

## Documentation

- API reference: [docs.rs/bevy_tiled_coords](https://docs.rs/bevy_tiled_coords)
- Local docs: `cargo doc --open`

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
