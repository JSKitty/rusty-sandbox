<h1 align="center">
  Rusty Sandbox
</h1>

<p align="center">
  A lightweight sandbox sim written in Rust.
</p>

<p align="center">
  <a href="https://stakecubecoin.net/wasm-sandbox/">Play via Browser</a> (WASM) | <a href="https://github.com/JSKitty/rusty-sandbox#developer-installation">Compile by yourself</a>
</p>

<p align="center">
  <img src="https://user-images.githubusercontent.com/42538664/185786815-206a50f7-5223-4ebd-aeb1-46479519c5cb.gif" />
</p>

<p align="center">
  This is a quick hobby project written to practice three things: Rust, <a href="https://macroquad.rs/">Macroquad</a> and Maths!
</p>

---

# Dev Builds

Prerequisites: The Rust Toolchain (stable preferred).

<details><summary><i><b>Local Compile</b></i> (For your architecture)</summary>

```bash
git clone https://github.com/JSKitty/rusty-sandbox.git && cd rusty-sandbox
cargo run --release
cargo build --release
```
</details>


<details><summary><i><b>WASM Compile</b></i> (For <a href="https://github.com/not-fl3/miniquad/#wasm">web-based</a> usage like <a href="https://stakecubecoin.net/wasm-sandbox/">this!</a>)</summary>
  
```bash
git clone https://github.com/JSKitty/rusty-sandbox.git && cd rusty-sandbox
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```
</details>

---

# Aim / Goals

The primary aims of the project being:
- **Minimalistic codebase:** easy to follow, easy to learn from, a 'living' tutorial.
- **Low Dependency:** as much written in-house as possible, such as physics algorithms, etc.
- **Lightweight:** should compile super fast, and execute super fast by users.
- **Fun:** should be pretty fun to play with! Both in code and in user-land.
