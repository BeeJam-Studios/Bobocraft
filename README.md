# Bobocraft

The goal of this project is to create a game with fully customisable robot battle vehicles.

## License

This project is dual-licensed under the AGPL-3.0 License and a commercial license.

- **AGPL-3.0 License**: You are free to use, modify, and distribute the code under the terms of the AGPL-3.0 License. See the [LICENSE](./LICENSE.md) file for details.
- **Commercial License**: If you do not wish to comply with the AGPL-3.0’s copyleft requirements, you can obtain a commercial license. For more information, contact us on [discord](https://discord.gg/6Ft534jh5e).

## Linting

Bobocraft uses [Clippy](https://doc.rust-lang.org/clippy/) for checking and linting Rust code. If you use VS Code or a derivative thereof, this should be enabled already (see `.vscode` in this repository) If not, check if your IDE supports changing the rust analyzer check command or simply run `cargo clippy` from a shell. If you installed Rust via `rustup` with its default configuration, Clippy should already be installed. If not, you can run `rustup component add clippy` from your shell to install it.

## Build and running

1. Linux:  
Clone Bobocraft and robocraft-1 repo next to each other and run from client/src/main.rs the function `main()`

2. Windows
Clone Bobocraft and robocraft-1 repo next to each other. Because of windows file system you should make a manual softlink between the asset folder and the game. You can do this from the cmd running:

```cmd
mklink /D "Drive_location\bobocraft\client\assets\gltf" "Drive_location\robocraft-1\gltf"
```
After that you can run the game the same way as `Linux`

> [!WARNING]
> If you work on the project while being on Windows and you push the symlink by accident, notify the people working on Linux to recreate their softlinks.


 