# Icestocksport Result Analysis Tool (ISRAT)

Icestocksport Result Analysis Tool (Short: ISRAT) is designed to be used in managing and analyze icestocksport competitions.
This project is based on [Tectonic](https://github.com/tectonic-typesetting/tectonic) for pdf exports and on [GTK4 Rust Binding](https://github.com/gtk-rs/gtk4-rs) and therefore on [GTK4](https://gitlab.gnome.org/GNOME/gtk).

## Build

In order to build this project you need to have installed a rust toolchain for your system. Currently GNU/Linux and Windows are supported, meaning that those are tested. Maybe MacOS works, maybe not. Please let me know, if you have compiled and tested it for MacOS. If changes need to be made regarding MacOS support, please create a pull request.

GTK4 has to be installed. Please refer to the official GTK4 installation instructions.

The [Tectonic project](https://github.com/tectonic-typesetting/tectonic) requires some externally installed dependencies.

### GNU/Linux

The dependencies can be downloaded using the systems package manager. This command works for debian based systems, for other package managers you need to lookup the according package names. 

For distros using deb packages:

```sh
sudo apt-get install libfontconfig1-dev libgraphite2-dev libharfbuzz-dev libicu-dev libssl-dev zlib1g-dev
```

For distros using dnf package manager:
```sh
sudo dnf install libxcb-devel fontconfig-devel graphite2-devel harfbuzz-devel libicu-devel openssl-devel zlib-devel 
```

Now you can build the project.

```sh
cargo build --release
```

The ISART binary can now be found as `./target/release/israt`.

### Windows
For Windows the installation of the dependencies is little bit more complicated.
You need to download [the Tectonic project](https://github.com/tectonic-typesetting/tectonic) in some folder.
In this folder you need to execute following instructions:

First, install [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg) if needed:
```sh
cargo install cargo-vcpkg
```

Download and build the required dependencies:
```sh
cargo vcpkg build
```

Copy the dependencies to the build folder of ISRAT (replace `[path_to_israt]` with the path where you downloaded the ISRAT source files):

```sh
cp -r ./vcpkg [path_to_israt]/target/vcpkg
```

Now we need to install gtk and it's components. We use [gvsbuild](https://github.com/wingtk/gvsbuild) which needs to be installed.
Therefore please follow to the installation guide on the Github page.

After the installation of `gvsbuild` you can build `gtk` and `libadwaita`:
```sh
gvsbuild build gtk
gvsbuild build libadwaita
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
