
# Class Structure

## Loader and Parser

`JsonSpriteifierPlugin`
primary entry point. registers assets and loaders with system.

`SpriteJsonLoader`
binds to Asset system in Bevy and handles files.


## Model Data
`SpriteSheet`
parsed data from JSON.

supports parsing data from JSONArray or JSONObject structures.

`SpriteAtFrame`
sprite definition at a particular location in the overall sprite sheet.

## Data Handling

on the bevy side data is used this way:

`SpriteSheetAsset`
persisted asset which contains handles to useful metadata.
```aiignore
- tags (tagged animations keyed by name)
- frames (single marked frames keyed by name)
- layout (TextureAtlasLayout)
```

`layout` is auto-generated from frame spec in original JSON.

`SpriteAnimationAsset`
asset which contains animation specs from parsed json.

````aiignore
- from
- to
- tag
- looping
````

tag is the `name` used to reference this animation spec
`from` and `to` map to frame indices from the original sprite sheet.


`SpriteFrameAsset`
asset which contains a mapping of a **named** sprite frame to an index in the overall **sprite sheet**.

```aiignore
- index
- rect
```

All frames have an index based on the order in which they appear in the `json`.