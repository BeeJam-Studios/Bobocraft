# Bobocraft Architecture
This repository contains a Cargo workspace with various crates used in the Bobocraft project

High-level overview of the crates:
* `format`: handles serialization and deserialization of .bobo and .json files containing bot data. It also provides common data definitions like `Robot` and `Placement` that are used throughoght the rest of the project
* `graph`: contains the damage propagation implementation.
* `cubes`: contains a system for looking up cube properties. Lookups return an `&'static Cube`, which contains connection information, tier and other part `Stats` like weight or health
* `client`: Contains the current bot viewer/eventual game client, which is written in Bevy.

In-depth overviews
