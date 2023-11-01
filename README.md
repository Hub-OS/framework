# WGSL Bindings

location(id) for Vertex attributes increments starting from 0
`[[location(0)]] vertex: vec4<f32>`
`[[location(1)]] vertex_color: vec4<f32>`

location(id) for InstanceData attributes continue from Vertex attributes
`[[location(2)]] opacity: f32`

group(0) is used for uniforms incrementing from 0
`[[group(0), binding(0)]]`

group(1) is used for Instance BindingResources with binding(id) incrementing from 0
`[[group(1), binding(0)]]`

## Building for Web

These instructions use [wasm-pack](https://rustwasm.github.io/wasm-pack/)

In Cargo.toml:

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

Create a function in lib.rs and decorate it with the `#[wasm_bindgen(start)]` macro to mark it as an entry:

```rust
use framework::prelude::*;

// note: async main, as blocking within a web browser will freeze the page + event loop
// using cfg_attr to use this macro only when building for wasm
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn web_main() {
  // normal main
}
```

Run: `wasm-pack build --target web`,
wasm-pack will create a folder named pkg for the generated files

Include the .js module from the pkg folder in your .html file:

```html
<body></body>

<script type="module">
  import init from "./pkg/[package_name].js";
  init();
</script>
```

## Building for Android (with Winit)

Requires Android SDK, Android NDK, and [cargo-apk](https://crates.io/crates/cargo-apk) installed
Make sure to `rustup target add [architecture]-linux-android` for your target architecture

In Cargo.toml:

```toml
[package.metadata.android.sdk]
target_sdk_version = 30
min_sdk_version = 30
max_sdk_version = 30
```

In Cargo.toml:

```toml
[lib]
crate-type = ["cdylib"]
```

Add this to lib.rs:

```rust
// note lack of async
#[cfg(target_os = "android")]
pub fn android_main(app: PlatformApp) {
    default_logger::init!();

    Game::new("Android", (800, 600))
      .with_platform_app(app) // pass app
      .run()
}
```

Use `cargo apk run --lib` to install your project to a connected device or emulator, you'll see errors from missing std if you don't have the correct target installed
Use `cargo apk build --lib --release` to create an apk
