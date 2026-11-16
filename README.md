
# Spritifier

a bevy utility to simplify setting up sprite assets.

Reads from exported JSON format in apps like `Asprite` and `Pixquare`.


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