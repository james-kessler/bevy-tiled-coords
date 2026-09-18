//! ECS markers, resources, and [`SystemParam`] helpers.

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_ecs_tiled::prelude::*;

use crate::geometry::{TiledMapGeometry, TilemapGeomParts};
use crate::picking::{tile_drawable_at_entity, TileDrawableAtEntity};

/// Marks the [`TiledTilemap`] used for pathfinding, placement, and cursor picks.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GameplayTilemap;

/// Inserted after snapping a Tiled object to its grid anchor on the gameplay tilemap.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct ObjectGridAnchored;

/// Inserted after applying mixed-height layer Y fixup on a tilemap entity.
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct TilemapHeightAdjusted;

/// Cached map and gameplay tilemap entities (rebuilt in [`refresh_tiled_coords_context`]).
#[derive(Resource, Clone, Debug, Default)]
pub struct TiledCoordsContext {
    pub map_entity: Option<Entity>,
    pub map_handle: Option<Handle<TiledMapAsset>>,
    pub gameplay_tilemap_entity: Option<Entity>,
}

/// Updates [`TiledCoordsContext`] from the scene (single map + [`GameplayTilemap`]).
pub fn refresh_tiled_coords_context(
    mut context: ResMut<TiledCoordsContext>,
    maps: Query<(Entity, &TiledMap)>,
    gameplay: Query<Entity, With<GameplayTilemap>>,
) {
    if let Ok((entity, tiled_map)) = maps.single() {
        context.map_entity = Some(entity);
        context.map_handle = Some(tiled_map.0.clone());
    }
    context.gameplay_tilemap_entity = gameplay.iter().next();
}

/// Builds [`TiledMapGeometry`] for one tilemap entity.
pub fn tiled_map_geometry_for_entity<'a>(
    maps: &'a Query<(&TiledMap, &TilemapAnchor)>,
    map_assets: &'a Assets<TiledMapAsset>,
    tilemap_entity: Entity,
    tilemaps: &'a Query<
        (
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &GlobalTransform,
        ),
        With<TiledTilemap>,
    >,
) -> Option<TiledMapGeometry<'a>> {
    let (tiled_map, anchor) = maps.single().ok()?;
    let map_asset = map_assets.get(&tiled_map.0)?;
    let (grid_size, tile_size, map_type, tilemap_global) = tilemaps.get(tilemap_entity).ok()?;
    Some(TiledMapGeometry::from_map_and_tilemap(
        map_asset,
        anchor,
        TilemapGeomParts {
            grid_size,
            tile_size,
            map_type,
            tilemap_global,
        },
    ))
}

/// Builds geometry for the entity stored in [`TiledCoordsContext::gameplay_tilemap_entity`].
pub fn geometry_from_context<'a>(
    context: &TiledCoordsContext,
    maps: &'a Query<(&TiledMap, &TilemapAnchor)>,
    map_assets: &'a Assets<TiledMapAsset>,
    tilemaps: &'a Query<
        (
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &GlobalTransform,
        ),
        With<TiledTilemap>,
    >,
) -> Option<TiledMapGeometry<'a>> {
    let entity = context.gameplay_tilemap_entity?;
    tiled_map_geometry_for_entity(maps, map_assets, entity, tilemaps)
}

/// Builds geometry for the tilemap marked [`GameplayTilemap`].
#[expect(clippy::type_complexity)]
pub fn geometry_for_gameplay_tilemap<'a>(
    maps: &'a Query<(&TiledMap, &TilemapAnchor)>,
    map_assets: &'a Assets<TiledMapAsset>,
    gameplay_tilemaps: &'a Query<
        (
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &GlobalTransform,
        ),
        (With<TiledTilemap>, With<GameplayTilemap>),
    >,
) -> Option<TiledMapGeometry<'a>> {
    let (tiled_map, anchor) = maps.single().ok()?;
    let map_asset = map_assets.get(&tiled_map.0)?;
    let (grid_size, tile_size, map_type, tilemap_global) = gameplay_tilemaps.single().ok()?;
    Some(TiledMapGeometry::from_map_and_tilemap(
        map_asset,
        anchor,
        TilemapGeomParts {
            grid_size,
            tile_size,
            map_type,
            tilemap_global,
        },
    ))
}

/// Groups queries used by [`tile_drawable_at_entity`].
#[derive(SystemParam)]
pub struct TileDrawableQueries<'w, 's> {
    tiles: Query<'w, 's, (&'static TilePos, &'static ChildOf), With<TiledTile>>,
    tilemaps: Query<
        'w,
        's,
        (
            &'static TilemapGridSize,
            &'static TilemapTileSize,
            &'static TilemapType,
            &'static GlobalTransform,
        ),
        With<TiledTilemap>,
    >,
    maps: Query<'w, 's, (&'static TiledMap, &'static TilemapAnchor)>,
    map_assets: Res<'w, Assets<TiledMapAsset>>,
}

impl TileDrawableQueries<'_, '_> {
    /// Drawable center in world space for a [`TiledTile`] entity.
    pub fn drawable_at(&self, tile_entity: Entity) -> Option<TileDrawableAtEntity> {
        tile_drawable_at_entity(
            tile_entity,
            &self.tiles,
            &self.tilemaps,
            &self.maps,
            &self.map_assets,
        )
    }
}

/// Registers [`TiledCoordsContext`] and [`IsoDepthPlugin`](crate::depth::IsoDepthPlugin).
pub struct TiledCoordsPlugin;

impl Plugin for TiledCoordsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TiledCoordsContext>()
            .add_plugins(crate::depth::IsoDepthPlugin)
            .add_systems(PostUpdate, refresh_tiled_coords_context);
    }
}
