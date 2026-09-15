//! Depth sorting with [`TilemapRenderSettings::y_sort`].

/// Z offset used by [`bevy_ecs_tilemap`] when `y_sort` is enabled on chunks.
///
/// Add this to a sprite's layer `z` so it sorts with tiles at the same world `y`.
pub fn y_sort_z_offset(world_y: f32, map_size_y: u32, tile_height: f32) -> f32 {
    1.0 - (world_y / (map_size_y as f32 * tile_height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_chunk_sort_key() {
        let z = y_sort_z_offset(48.0, 8, 16.0);
        assert!((z - 0.625).abs() < 1e-5);
    }
}
