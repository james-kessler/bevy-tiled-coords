# Contributing

Thank you for helping improve `bevy_tiled_coords`.

## Development

```text
python3 tests/fixtures/generate_tile.py
cargo test
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo doc --open
```

## Releases

1. Bump the version in `Cargo.toml`.
2. Commit and push to `main`.
3. Create and push a tag: `git tag v0.1.1 && git push origin v0.1.1`.
4. GitHub Actions publishes to crates.io when the tag lands.

Set repository secret `CARGO_REGISTRY_TOKEN` to a crates.io API token with publish scope.
