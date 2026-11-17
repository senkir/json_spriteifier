
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

`SpriteAnimationAsset`
asset which contains animation specs from parsed json.

````aiignore
- from
- to
- tag
- looping
````


`SpriteTag`
asset which contains a mapping of a **named** sprite frame to an index in the overall **sprite sheet**.

```aiignore

```