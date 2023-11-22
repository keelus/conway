<h1 align="center">Conway's Game Of Life</h1>

<p align="center">
  <a href="./LICENSE.md"><img src="https://img.shields.io/badge/⚖️ license-MIT-blue" alt="MIT License"></a>
  <img src="https://img.shields.io/github/stars/keelus/conway?color=red&logo=github" alt="stars">
</p>

## ℹ️ Description
An implementation of [Conway's Game Of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) in Rust, using SDL2 and SDL2_TTF for graphics.
> Known v1.0.1 errors: Drawing while iteration is paused won't work as expected once resumed the iteration. Drawing before starting the iteration will work correctly.

## 📸 Screenshots
<img src="https://github.com/keelus/conway/assets/86611436/c88c0809-c78d-414e-99b5-0f74a155561d" width=400 />
<img src="https://github.com/keelus/conway/assets/86611436/9b9eb294-543a-4984-93a8-b234430a3006" width=400 />

## 🔨 Requirements
This project requires to have the libraries [SDL2](https://github.com/libsdl-org/SDL) and [SDL2_TTF](https://github.com/libsdl-org/SDL_ttf) properly installed.
## ⬇️ Install & run it
### 🪟 Windows
Download the [latest release](https://github.com/keelus/conway/releases/latest) and execute the binary `conway.exe` by double clicking or in the console:
```bash
.\conway.exe
```
### 🐧 Linux or macOS
I didn't compile any build to execute outside Windows, but you can do it yourself, like stated below.
## 📦 Build it
While being in the root directory, execute in the terminal:
```bash
cargo build
```
Remember to check that [🔨 Requirements](#-requirements) are properly installed.
> If you are building it in Windows, make sure you have `SDL2.dll` and `SDL2_TTF.ddl` in the root folder (next to `Cargo.toml` while `cargo run`, and next to the binary when running the build).
## ⚖️ License
This project is open source under the terms of the [MIT License](./LICENSE)

<br />
Made by <a href="https://github.com/keelus">keelus</a> ✌️
