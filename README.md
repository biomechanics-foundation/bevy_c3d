# bevy_c3d

![Crates.io](https://img.shields.io/crates/v/bevy_c3d.svg)

A .c3d asset loader plugin for the [Bevy engine](https://github.com/bevyengine/bevy)

## Usage

1. Add the crate as a dependency through:

```
cargo add bevy_c3d
```

or add it to your `Cargo.toml`

```toml
[dependencies]
bevy = "0.11"
bevy_c3d = "0.11"
```

The major and minor versions should match Bevy

2. Add the plugin:

```rust
use bevy::prelude::*;
use bevy_c3d::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, C3dPlugin))
        .run();
}
```

3. Load the `.c3d` file as an asset:

```rust
fn example_load_c3d(asset_server: Res<AssetServer>, mut c3d_state: ResMut<C3dState>) {
    c3d_state.handle = asset_server.load("test.c3d");
}
```

## Example

A full example is available at `examples/basic.rs`.

Clone this repository and use the command `cargo run --example basic` to run the example.

## Support

bevy_c3d is part of the [biomech.dev](https://biomech.dev) family of open-source libraries. Consider supporting our work to help us contribute more to the body of biomechanics software.


