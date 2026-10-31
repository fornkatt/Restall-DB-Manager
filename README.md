# Restall DB Manager

A desktop editor for Restall's mod database. You can edit entries, add and remove database files,
then commit and push the changes to the database repository.

## Status

Early development.

## Building

You need Rust. The specific version is pinned in `rust-toolchain.toml`, rustup installs it automatically.

On Linux, you also need these development packages (names may differ depending on distro) on Debian/Ubuntu:

```sh
apt install libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libfontconfig-dev
```

Then run:
```sh
cargo run -p restall-db-manager
```

## License

GPL-3.0-only. See [LICENSE](LICENSE)
