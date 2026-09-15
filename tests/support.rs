//! Shared test helpers (fixture paths and TMX loading).

use std::path::PathBuf;

pub fn fixture_dir() -> PathBuf {
    PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
}

pub fn load_iso_fixture_map() -> tiled::Map {
    let path = fixture_dir().join("iso_map.tmx");
    tiled::Loader::new()
        .load_tmx_map(&path)
        .expect("load iso_map.tmx")
}
