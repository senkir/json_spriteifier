
# Json-Spriteifier

a bevy utility to simplify setting up sprite assets.

Reads from exported JSON format in apps like `Asprite` and `Pixquare`.

[![Rust](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml/badge.svg)](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/senkir/json_spriteifier/graph/badge.svg?token=I3NMG75RL0)](https://codecov.io/github/senkir/json_spriteifier)

# Objective

This library speeds up the workflow of mapping sprite sheets from apps like `Aseprite` and `Pixquare`.
These apps generate helpful sprite sheets for modern games but require manual setup for each asset.  
They already support exporting both the image and json but handling these files is still done by hand per game project.

In a project where we have to do this for hundreds of assets we can save a lot of time by parsing `json` directly out of these apps.

This library does the work for you by reading the raw `json` and passing back a handle to asset data that is easily readable in game output.
It also greatly simplifies animation loops by passing back a name keyed spec which can be handled by rendering engines.

- `TextureAtlasLayout` is generated automatically from the sprite spec.
- `SpriteFrameAsset` is generated from individual frame data. keyed by name where it is available.
- `SpriteAnimationAsset` is generated from tagged animation block data.

# Compatibility

This project has only been tested on Bevy `0.2.0` at this time.

Bevy Compatibility:

| Bevy Version |	Plugin Version |
| ---- | ----- |
| 0.20 |	0.4.0 |

# Setup

Installing directly via Github into  `cargo.toml`
```
json-spriteifier = { git = "https://github.com/senkir/json_spriteifier", tag = "v0.4.0" }
```

Not yet available on `crates.io`.

# Usage

Simply add the plugin on app initialization and it will bind the required assets.

Plugin
```
app.add_plugins(JsonSpriteifierPlugin);
```

### Loading Sprite

usually it is desired to use the turbofish syntax to avoid clashes with other json parsers.

```
let handle = asset_server.load::<SpriteSheetAsset>("my_sprite.json")
```

### Using Animation Data

```
let handle = asset_server.load::<SpriteSheetAsset>("my_sprite.json")
```

### Supported Extensions

supported extension is

```aiignore
.json
```

## Troubleshooting

If other Json parsers are used (such as `JsonAssetPlugin`) either explicitly declare the type or change the extension to avoid conflicts.

```
let handle = asset_server.load::<SpriteSheetAsset>("my_sprite.json");
//or
let handle: Handle<SpriteSheetAsset> =  asset_server.load::<SpriteSheetAsset>("my_sprite.json");
```

or

```
asset_server.load("my_sprite.sprites.json")
```

### Running CI locally

```
cargo llvm-cov --cobertura --output-path coverage.xml
```

### Open Tasks

- [x] Complete Schema parsing for both JSONObject and JSONArray
- [ ] Finish setting up examples for the common use cases.
- [ ] The return in animate_sprite stops the system for every sprite after the first finished one.
- [ ] Loop counting is never incremented, so counted animations can't finish.
- [ ] The todo!() in the old loader path, if it's still reachable.
- [ ] The animation_finished comparison against the frame index.