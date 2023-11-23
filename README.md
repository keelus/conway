<h1 align="center">Conway's Game Of Life</h1>

<p align="center">
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/⚖️ license-MIT-blue" alt="MIT License"></a>
  <img src="https://img.shields.io/github/stars/keelus/conway?color=red&logo=github" alt="stars">
</p>

## ℹ️ Description
An implementation of [Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust, using SDL2 and SDL2_TTF for graphics.

## 📸 Screenshots
<img src="https://github.com/keelus/conway/assets/86611436/c88c0809-c78d-414e-99b5-0f74a155561d" width=400 />
<img src="https://github.com/keelus/conway/assets/86611436/9b9eb294-543a-4984-93a8-b234430a3006" width=400 />

## 🔨 Requirements
This project requires to have the libraries [SDL2](https://github.com/libsdl-org/SDL) and [SDL2_TTF](https://github.com/libsdl-org/SDL_ttf) properly installed. Check [🐧 Linux and macOS SDL2 build](#-linux-and-macOS-SDL2-build) before using.
## ⬇️ Install & run it
### 🪟 Windows
Download the [latest release](https://github.com/keelus/conway/releases/latest) and execute the binary `conway.exe` by double clicking or in the console:
```bash
.\conway.exe
```
### 🐧 Linux or macOS
First check [🐧 Linux and macOS SDL2 build](#-linux-and-macOS-SDL2-build).

Then, download the [latest release](https://github.com/keelus/conway/releases/latest) and execute the binary `conway` by double clicking or in the console:
```bash
.\conway
```
## 📦 Build it
While being in the root directory, execute in the terminal:
```bash
cargo build
```
Remember to check that [🔨 Requirements](#-requirements) are properly installed.
> If you are building it in Windows, make sure you have `SDL2.dll` and `SDL2_TTF.dll` in the root folder (next to `Cargo.toml` while `cargo run`, and next to the binary `conway.exe` when running the build).
## 🐧 Linux and macOS SDL2 build
SDL2 is required to render the graphics of conway. Because of the problems that it generates with Rust and macOS (specially) I recommend building SDL2 from the source code to prevent any errors.

[CMake](https://cmake.org/download/) is required to be able to build from source code. 

Once installed, download the `SDL2(_ttf)-x.xx.x.tar.gz` or `SDL2(_ttf)-x.xx.x.zip` versions from [SDL2 repository](https://github.com/libsdl-org/SDL/releases/latest) and [SDL2_TTF repository](https://github.com/libsdl-org/SDL_ttf/releases/latest) and unzip both. Then execute:
```bash
cd ./SDL2(_ttf)-x.x.x # SDL2-... or SDL2_ttf-..., depending on which one you are building (both required)
./configure
make
sudo make install
```
Then you will be able to run and build conway.

## ⚖️ License
This project is open source under the terms of the [MIT License](./LICENSE)

<br />
Made by <a href="https://github.com/keelus">keelus</a> ✌️
