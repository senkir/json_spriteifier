
# Expected Schemas

`Aseprite`
```json
{ "frames": {
   "[key name]": {
    "frame": { 
      "x": [int], 
      "y": [int], 
      "w": [int],
      "h": [int]
    },
    "rotated": [boolean],
    "trimmed": [boolean],
    "spriteSourceSize": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "sourceSize": {
      "w": [int],
      "h": [int]
    },
    "duration": [int]
   },
   "Crow 1.aseprite": {
    "frame": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "rotated": [bool],
    "trimmed": [bool],
    "spriteSourceSize": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "sourceSize": { 
      "w": [int], 
      "h": [int]
    },
    "duration": [int]
   }
 },
 "meta": {
  "app": [String],
  "version": [String],
  "image": [String],
  "format": [String],
  "size": { 
    "w": [int], 
    "h": [int]
  },
  "scale": [String], // discrepancy with Pixquare
  "frameTags": [
   { "name": [String], 
     "from": [int], 
     "to": [int], 
     "direction": [String],  
     "color": [String], //parsed as an srgba color
     "looping": Optional[int], // discrepancy with Aseprite
   }
  ],
  "layers": [
//    not used in this project
  ],
  "slices": [
    //    not used in this project
  ]
 }
}

```


`Pixquare`
```json
{ "frames": {
   "[key name]": {
    "frame": { 
      "x": [int], 
      "y": [int], 
      "w": [int],
      "h": [int]
    },
    "rotated": [boolean],
    "trimmed": [boolean],
    "spriteSourceSize": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "sourceSize": {
      "w": [int],
      "h": [int]
    },
    "duration": [int]
   },
   "Crow 1.aseprite": {
    "frame": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "rotated": [bool],
    "trimmed": [bool],
    "spriteSourceSize": {
      "x": [int],
      "y": [int],
      "w": [int],
      "h": [int]
    },
    "sourceSize": { 
      "w": [int], 
      "h": [int]
    },
    "duration": [int]
   }
 },
 "meta": {
  "app": [String],
  "version": [String],
  "image": [String],
  "format": [String],
  "size": { 
    "w": [int], 
    "h": [int]
  },
  "scale": [int], // discrepancy with Aseprite
  "frameTags": [
   { "name": [String], 
     "from": [int], 
     "to": [int], 
     "direction": [String],
     "color": [String], //parsed as an srgba color
     "looping": [int], // discrepancy with Aseprite
   }
  ],
  "layers": [
//    not used in this project],
  "slices": [
    //    not used in this project
  ]
 }
}

```