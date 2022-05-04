# Icestocksport Result Analysis Tool (ISRAT)

Icestocksport Result Analysis Tool (Short: ISRAT) is designed to be used in managing and analyse icestocksport
tournaments.
This project is based on [Tectonic](https://github.com/tectonic-typesetting/tectonic) for pdf exports and on [dear imgui rust binding](https://github.com/imgui-rs/imgui-rs), [glium](https://github.com/glium/glium) and [winit](https://github.com/rust-windowing/winit) for the GUI.
Currently under construction!

## Build

In order to build this project you need to have installed a rust toolchain for your system. Currently GNU/Linux and Windows are supported, meaning that those are tested. Maybe MacOS works, maybe not. Please let me know, if you have compiled and tested it for MacOS. If changes need to be made regarding MacOS support, please create a pull request.

The [tectonic project](https://github.com/tectonic-typesetting/tectonic) requires some externally installed dependencies.

### GNU/Linux

The dependencies can be downloaded using the systems package manager. This command works for debian based systems, for other package managers you need to lookup the according package names. 

```sh
sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
```

Now you can build the project.

```sh
cargo build --release
```

The ISART binary can now be found as `./target/release/israt`.
### Windows
For Windows the installation of the dependencies is little bit more complicated.
You need to download [this project](https://github.com/tectonic-typesetting/tectonic) in some folder. In this folder you need to execute following instructions:

First, install [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) if needed:
```sh
cargo install cargo-vcpkg
```

Download and build the required dependencies:
```sh
cargo vcpkg build
```

Copy the dependencies to the build folder of ISRAT (replace `[path_to_israt]` with the path where you downloaded this project):

```sh
cp -r ./vcpkg [path_to_israt]/target/vcpkg
```

Now you can build ISRAT:

```sh
cargo build --release
```

The ISART binary can now be found as `./target/release/israt`.

## Usage

Start the program and the GUI will be self explaining.

## License
ISRAT is licensed under the MIT License, see [LICENSE.txt](https://github.com/Explosiontime202/ISRAT/blob/master/LICENSE.txt) for more information.
