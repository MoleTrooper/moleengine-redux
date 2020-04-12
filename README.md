MoleEngine is a general-purpose 2D game engine written in Rust as a solo hobby project.
Its primary focuses are cool physics tools and a simple interface that allows for quick prototyping.

[Introductory blog post](https://moletrooper.github.io/blog/2018/09/moleengine-part-0-introduction/)

# Current features

- Entity-Component-System inspired object model
  - **a big rewrite is currently in progress**
- 2D rigid body physics
  - narrow phase collision detection
  - rudimentary collision impulse solver
- Graphics
  - Simple 2D mesh rendering with [wgpu](https://github.com/gfx-rs/wgpu-rs)

See the [issues](https://github.com/MoleTrooper/moleengine/issues) for notes on future developments.

# Blog

There used to be a blog link here, but
progress on this project has been slow and I've put it on ice for now.
I plan to get back to it in the fall of 2020 as I begin my master's degree studies;
until then, stay tuned.

# Running the test game

There's not much to show here, but should you wish to check out my tiny test game, here's how:

**The manual way**

1. Install [Rust](https://www.rust-lang.org/learn/get-started)
2. You may need to install `pkgconfig` and drivers for Vulkan, DX12, or Metal depending on your platform
3. Clone and navigate to this repository
4. `cargo run --example testgame`

**The easy way, using [Nix](https://nixos.org/nix/)**

1. Clone and navigate to this repository
2. `nix-shell`
3. `cargo run --example testgame`

### Testgame keybindings

Disclaimer: these might be out of date - the test game changes in quick and dirty ways

```
Space   - pause
Enter   - load / reload level
Esc     - close the game
S       - spawn a box
Arrows  - nudge the "player" box around
PgUp/Dn - spin the "player" box
LShift  - stop the "player" box from moving
```
