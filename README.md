# Bobocraft

The goal of this project is to create a game with fully customisable robot battle vehicles.

## License

This project is dual-licensed under the AGPL-3.0 License and a commercial license.

- **AGPL-3.0 License**: You are free to use, modify, and distribute the code under the terms of the AGPL-3.0 License. See the [LICENSE](./LICENSE.md) file for details.
- **Commercial License**: If you do not wish to comply with the AGPL-3.0’s copyleft requirements, you can obtain a commercial license. For more information, contact us on [discord](https://discord.gg/6Ft534jh5e).

## Getting Started

1. clone this repo and run `git lfs fetch --all`
2. aquire game assets
3. in `client/assets` create a sym-link to the `gtlf`-folder of the assets
4. run `BEVY_ASSET_PATH=$PWD/client/assets cargo r --features sped`

## Linting

Bobocraft uses [Clippy](https://doc.rust-lang.org/clippy/) for checking and linting Rust code. If you use VS Code or a derivative thereof, this should be enabled already (see `.vscode` in this repository) If not, check if your IDE supports changing the rust analyzer check command or simply run `cargo clippy` from a shell. If you installed Rust via `rustup` with its default configuration, Clippy should already be installed. If not, you can run `rustup component add clippy` from your shell to install it.
