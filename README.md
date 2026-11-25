
# Json-Spriteifier

a bevy utility to simplify setting up sprite assets.

Reads from exported JSON format in apps like `Aseprite` and `Pixquare`.

[![Rust](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml/badge.svg)](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/senkir/json_spriteifier/graph/badge.svg?token=I3NMG75RL0)](https://codecov.io/github/senkir/json_spriteifier)

# Objective

This lightweight library speeds up the workflow of mapping sprite sheets from apps like `Aseprite` and `Pixquare`.
These apps generate helpful sprite sheets for modern games but require manual setup for each asset.  
They already support exporting both the image and json but handling these files is still done by hand per game project.

In a project where we have to do this for hundreds of assets we can save a lot of time by parsing `json` directly out of these apps.
As this is a data driven approach, the source `json` can be edited for tweaking, rather than having to go back to the source application.
Since this project supports both app standards, there's no need to add multiple plugins as a dependency.

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
| 0.20 | main |
| 0.20 |	0.5.0 |

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
let img_handle = asset_server.load::("my_sprite.png")
```

### Rendering Sprite
Convenience methods are provided to render sprite through the asset template.

```
    commands.spawn_scene(bsn! {
        #scene
        Transform::from_xyz(1.0, 1.0, 1.0).with_scale(Vec3::splat(2.0))
        ~SpriteSheetAssetTemplate::from(texture)
        .with_image(image)
        .with_index(7)
    });
```

### Using Animation Data
Attaching loaded animations from the sheet are done by name.
name must match an existing tag on the source `json`.
This will provide specific frame data such as the from/to target frames from the sheet.

```
     let anim = sheet.get_animation_tag("Blink");
```

For convenience this can also be loaded directly as a template via a convenience method.
This will set up the animation frames automatically and pull duration data as well.

```
    let anim_scene = spriteifier::sprite_animation(sheet.clone(), "Blink");
```
using the `SpriteAnimationTemplate` expects the `SpriteTimer` component in order to time out frames.
```
spriteifier::sheet_timer()
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

- [ ] consider supporting chained animations.
- [ ] consider supporting finding animation frames by name.
- [x] Complete Schema parsing for both JSONObject and JSONArray
- [x] Finish setting up examples for the common use cases.
- [x] The return in animate_sprite stops the system for every sprite after the first finished one.
- [x] Loop counting is never incremented, so counted animations can't finish.
- [ ] The todo!() in the old loader path, if it's still reachable.
- [x] The animation_finished comparison against the frame index.