# About

This project is a program designed to facilitate the creation of characters for OSFR. It generates a JSON file with the specified parameters for each character.

# Building

To compile this project, it is necessary to have not only the Rust tools installed, but also Tauri. The installation of Tauri is explained in detail [here](https://tauri.app/v1/guides/getting-started/prerequisites).

## Documentation

A good method to read the documentation is using the `cargo doc --open` command. This will open the documentation in your browser. If you want to specify which documentations you want to view, use `cargo doc --help` for more information.

## Output File

The produced file is saved in the `characters` folder. Please note that the file may not be easily human-readable as it is optimized for machine reading.

## TODO

- [ ] Add the ability to add "Mounts" to characters.
- [ ] Add the ability to add "Clothes" to characters.
