
# Json-Spriteifier

a bevy utility to simplify setting up sprite assets.

Reads from exported JSON format in apps like `Asprite` and `Pixquare`.

[![Rust](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml/badge.svg)](https://github.com/senkir/json_spriteifier/actions/workflows/rust.yml)
[![codecov](https://codecov.io/github/senkir/json_spriteifier/graph/badge.svg?token=I3NMG75RL0)](https://codecov.io/github/senkir/json_spriteifier)


# Usage

Simply add the plugin on app initialization and it will bind the required assets.

Plugin
```
app.add_plugins(JsonSpriteifierPlugin);
```

### Loading Sprite

```
asset_server.load("my_sprite.json")
```

### Supported Extensions

supported extensions are

```aiignore
.json
.sprites.json
```

## Troubleshooting

If other Json parsers are used (such as `JsonAssetPlugin`) either explicitly declare the type or change the extension to avoid conflicts.

```
let handle = asset_server.load<SpriteSchema>("my_sprite.json");
//or
let handle: Handle<SpriteSchema> =  asset_server.load<SpriteSchema>("my_sprite.json");
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