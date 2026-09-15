use bevy_ecs_tiled::prelude::*;
use bevy_tiled_coords::tile_drawable_y_offset;

#[test]
fn drawable_y_offset_for_tall_tiles() {
    assert_eq!(tile_drawable_y_offset(32.0, 16), 8.0);
    assert_eq!(tile_drawable_y_offset(16.0, 16), 0.0);
}
