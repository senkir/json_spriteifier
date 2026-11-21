use bevy::math::UVec2;
use bevy::prelude::{Asset, Srgba, TypePath, URect};
use serde::de::{Error, MapAccess, SeqAccess};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
// region schema

// Aseprite and Pixquare
// both can export using a `json` schema that is pretty similar
//
#[derive(Asset, TypePath, Deserialize, Debug, PartialEq, Clone)]
#[derive(Default)]
pub struct SpriteSheet {
    /// raw frame data
    #[serde(deserialize_with = "deserialize_sprite_sheet")]
    pub frames: Vec<SpriteAtFrame>,

    ///point name keys to frame indices
    #[serde(skip)]
    pub indices: HashMap<String, usize>,

    /// useful metadata
    pub meta: Meta,
}


impl SpriteSheet {
    /// overall sheet size of source image
    pub fn sheet_size(&self) -> UVec2 {
        self.meta.size
    }

    /// attempt to get the SpriteSheet frame for the given name key
    pub fn frame_for_name(&self, key: String) -> Option<SpriteAtFrame> {
        if let Some(index) = self.indices.get(&key) {
            self.frames.get(*index).cloned()
        } else {
            None
        }
    }
}

/// individual image definitions within a sheet
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteAtFrame {
    //duration in Ms this sprite should show in animation context.
    pub duration: usize,
    #[serde(deserialize_with = "deserialize_rect")]
    pub frame: URect,
    pub rotated: bool,
    #[serde(deserialize_with = "deserialize_size")]
    pub source_size: UVec2,
    #[serde(rename = "spriteSourceSize", deserialize_with = "deserialize_rect")]
    pub sprited_source_size: URect,
    pub trimmed: bool,

    /// Key from the object form of `frames`; `None` for the array form.
    #[serde(skip)]
    pub name: Option<String>,
}

impl Default for SpriteAtFrame {
    fn default() -> Self {
        Self {
            name: None,
            trimmed: false,
            sprited_source_size: URect::EMPTY,
            source_size: UVec2::ZERO,
            frame: URect::EMPTY,
            rotated: false,
            duration: 0,
        }
    }
}

/// metadata
///
/// general information to organize frame data
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// the app that generated the json sprite sheet
    app: String,
    /// version of app that generated json
    version: String,
    /// color image format eg: RGBA8888
    format: String,
    /// tagged sections used in specific animations
    pub frame_tags: Vec<TaggedSprites>,
    /// related sheet image name
    pub image: String,
    /// scaling value
    #[serde(deserialize_with = "deserialize_scale")]
    pub scale: usize,
    /// image size for sheet
    #[serde(deserialize_with = "deserialize_size")]
    pub size: UVec2,
}

impl Default for Meta {
    fn default() -> Self {
        Self {
            app: "".to_string(),
            version: "".to_string(),
            scale: 1,
            size: UVec2::ZERO,
            image: "".to_string(),
            frame_tags: Vec::new(),
            format: "".to_string(),
        }
    }
}

/// tagged frame information
#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaggedSprites {
    /// tag name
    pub name: String,
    /// the color filter for the sprite
    #[serde(deserialize_with = "de_color")]
    pub color: Srgba,
    /// render direction such as "forward"
    pub direction: String,
    /// start frame
    pub from: usize,
    /// end frame
    pub to: usize,

    /// looping flag for animation counts
    #[serde(default, rename = "loop")]
    pub looping: usize,
}

/// raw frame dimensions and start params
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
struct Frame {
    pub h: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {} {})", self.h, self.w, self.x, self.y)
    }
}

impl From<Frame> for URect {
    fn from(frame: Frame) -> Self {
        
        URect::new(
            frame.x as u32,
            frame.y as u32,
            frame.x as u32 + frame.w as u32,
            frame.y as u32 + frame.h as u32,
        )
    }
}

/// size data in pixels
#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
struct Size {
    pub h: i32,
    pub w: i32,
}

impl From<Size> for UVec2 {
    fn from(size: Size) -> UVec2 {
        UVec2::new(size.w as u32, size.h as u32)
    }
}

// endregion

fn de_color<'de, D>(deserializer: D) -> Result<Srgba, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_s = String::deserialize(deserializer)?;
    Srgba::hex(&hex_s).map_err(D::Error::custom)
}

#[derive(Deserialize)]
#[serde(untagged)]
enum SizeType {
    Num(usize),
    Str(String),
}

fn deserialize_scale<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    match SizeType::deserialize(deserializer)? {
        SizeType::Num(num) => Ok(num),
        SizeType::Str(string) => string.parse().map_err(Error::custom),
    }
}

fn deserialize_size<'de, D>(deserializer: D) -> Result<UVec2, D::Error>
where
    D: Deserializer<'de>,
{
    let size = Size::deserialize(deserializer)?;
    Ok(UVec2::from(size))
}

fn deserialize_rect<'de, D>(deserializer: D) -> Result<URect, D::Error>
where
    D: Deserializer<'de>,
{
    let frame = Frame::deserialize(deserializer)?;

    Ok(URect::from(frame))
}

fn deserialize_sprite_sheet<'de, D>(deserializer: D) -> Result<Vec<SpriteAtFrame>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(SpriteSheetVisitor)
}

/// visitor to unwrap mapped frames into a linear `Vec` or mapped values.
/// will also keep name keys in a separate map for future referencing
struct SpriteSheetVisitor;

impl<'de> serde::de::Visitor<'de> for SpriteSheetVisitor {
    type Value = Vec<SpriteAtFrame>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of spritesheets")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut frames: Vec<SpriteAtFrame> = Vec::new();

        while let Some(frame) = seq.next_element::<SpriteAtFrame>()? {
            frames.push(frame);
        }
        Ok(frames)
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut frames: Vec<SpriteAtFrame> = Vec::with_capacity(map.size_hint().unwrap_or(0));

        while let Some(name) = map.next_key()? {
            let mut frame: SpriteAtFrame = map.next_value()?;
            frame.name = Some(name);
            frames.push(frame);
        }
        Ok(frames)
    }
}

// region tests

#[cfg(test)]
macro_rules! test_read_file {
    ($fname:expr) => {
        &fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/test/assets/", $fname))
            .expect("Failed to read fixture")
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn assert_data_of_type<T>(json: &str, expected: T)
    where
        T: serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let mut de = serde_json::Deserializer::from_str(json);

        let actual: T = serde_path_to_error::deserialize(&mut de)
            .unwrap_or_else(|e| panic!("failed at `{}`: {}", e.path(), e.inner()));

        pretty_assertions::assert_eq!(expected, actual, "data doesn't match");
    }
    fn parse_type<T>(json: &str) -> T
    where
        T: serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let mut de = serde_json::Deserializer::from_str(json);

        serde_path_to_error::deserialize(&mut de)
            .unwrap_or_else(|e| panic!("failed at `{}`: {}", e.path(), e.inner()))
    }

    #[test]
    fn test_full_frame_1() {
        let json = test_read_file!("frame_1.json");
        let mut vec = Vec::new();
        vec.push(SpriteAtFrame {
            duration: 100,
            name: Some("Crow 0.aseprite".to_string()),
            frame: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            rotated: false,
            source_size: UVec2::from(Size { h: 64, w: 64 }),
            sprited_source_size: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            trimmed: false,
        });

        let mut tags = Vec::new();
        let obj = SpriteSheet {
            meta: Meta {
                app: "https://www.aseprite.org/".to_string(),
                version: "1.3.18.6-arm64-trial".to_string(),
                format: "RGBA8888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: UVec2::from(Size { w: 512, h: 64 }),
                frame_tags: tags,
            },
            indices: HashMap::new(),
            frames: vec,
        };
        assert_data_of_type(&json, obj);
    }

    #[test]
    fn test_full_frame_alt_pixuare() {
        let json = test_read_file!("frame_pixquare.json");
        let mut vec = Vec::new();
        vec.push(SpriteAtFrame {
            duration: 100,
            name: Some("Crow 0.aseprite".to_string()),
            frame: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            rotated: false,
            source_size: UVec2::from(Size { h: 64, w: 64 }),
            sprited_source_size: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            trimmed: false,
        });

        let mut tags = Vec::new();
        tags.push(TaggedSprites {
            direction: "forward".to_string(),
            from: 0,
            to: 6,
            name: "Takeoff".to_string(),
            color: Srgba::hex("#FFFFFFFF").unwrap(),
            looping: 0,
        });
        let obj = SpriteSheet {
            meta: Meta {
                app: "Pixquare".to_string(),
                version: "2.74.1".to_string(),
                format: "RGBA8888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: UVec2::from(Size { w: 512, h: 64 }),
                frame_tags: tags,
            },
            frames: vec,
            indices: HashMap::new(),
        };
        assert_data_of_type(json, obj);
    }

    fn build_sprite_index(x: i32, name: String) -> SpriteAtFrame {
        SpriteAtFrame {
            duration: 100,
            name: Some(name.to_string()),
            frame: URect::from(Frame {
                h: 64,
                w: 64,
                x,
                y: 0,
            }),
            rotated: false,
            source_size: UVec2::from(Size { h: 64, w: 64 }),
            sprited_source_size: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            trimmed: false,
        }
    }

    fn build_sprite_xy(x: i32, y: i32, file: String) -> SpriteAtFrame {
        SpriteAtFrame {
            duration: 100,
            name: None,
            frame: URect::from(Frame { h: 64, w: 64, x, y }),
            rotated: false,
            source_size: UVec2::from(Size { h: 64, w: 64 }),
            sprited_source_size: URect::from(Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            }),
            trimmed: false,
        }
    }

    #[test]
    fn test_full_crow() {
        let json = test_read_file!("crow.json");
        let mut vec = Vec::new();
        vec.push(build_sprite_index(
            0,
            format!("Crow {}.aseprite", 0).to_string(),
        ));
        vec.push(build_sprite_index(
            64,
            format!("Crow {}.aseprite", 1).to_string(),
        ));

        let mut tags = Vec::new();
        tags.push(TaggedSprites {
            name: "Blink".to_string(),
            color: Srgba::hex("#FFFFFFFF").unwrap(),
            direction: "forward".to_string(),
            from: 2,
            to: 7,
            looping: 0,
        });
        let obj = SpriteSheet {
            meta: Meta {
                app: "https://www.aseprite.org/".to_string(),
                version: "1.3.18.6-arm64-trial".to_string(),
                format: "RGBA8888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: UVec2::from(Size { w: 512, h: 64 }),
                frame_tags: tags,
            },
            frames: vec,
            indices: HashMap::new(),
        };
        assert_data_of_type(json, obj);
    }

    #[test]
    fn test_crow_array() {
        let json = test_read_file!("crow_array.json");
        let mut vec = Vec::new();
        vec.push(build_sprite_xy(30, 30, "Crow 0".to_string()));

        let mut tags = Vec::new();
        tags.push(TaggedSprites {
            name: "Blink".to_string(),
            color: Srgba::hex("#FFFFFFFF").unwrap(),
            direction: "forward".to_string(),
            from: 2,
            to: 7,
            looping: 0,
        });
        let obj = SpriteSheet {
            meta: Meta {
                app: "Pixquare".to_string(),
                version: "2.74.1".to_string(),
                format: "RGBA888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: UVec2::from(Size { w: 104, h: 104 }),
                frame_tags: tags,
            },
            frames: vec,
            indices: HashMap::new(),
        };
        assert_data_of_type(json, obj);
    }

    #[test]
    fn test_frame() {
        let frame = Frame {
            h: 64,
            w: 64,
            x: 0,
            y: 0,
        };
        assert_data_of_type(r#"{ "x": 0, "y": 0, "w": 64, "h": 64 }"#, frame)
    }

    #[test]
    fn test_size() {
        let json = r#"{ "w": 512, "h": 64  }"#;

        let data: Size = serde_json::from_str(json).expect("failed to parse json");

        assert_eq!(data, Size { w: 512, h: 64 });
        assert_ne!(data, Size { w: 100, h: 100 })
    }

    #[test]
    fn test_sprite_sheet_index() {
        let json = test_read_file!("sprite_sheet_index.json");
        let reference = SpriteAtFrame {
            duration: 100,
            rotated: false,
            frame: URect::from(Frame {
                w: 64,
                h: 64,
                x: 0,
                y: 0,
            }),
            source_size: UVec2::from(Size { w: 64, h: 64 }),
            sprited_source_size: URect::from(Frame {
                w: 64,
                h: 64,
                x: 0,
                y: 0,
            }),
            trimmed: false,
            name: None,
        };
        assert_data_of_type(json.as_str(), reference);
    }

    #[test]
    fn test_frame_tag() {
        let json = r#"{ "name": "Blink", "from": 2, "to": 7, "direction": "forward", "color": "ffffffff" }"#;

        let reference = TaggedSprites {
            name: "Blink".to_string(),
            from: 2,
            to: 7,
            direction: "forward".to_string(),
            color: Srgba::hex("#FFFFFFFF").unwrap(),
            looping: 0,
        };

        assert_data_of_type(json, reference);
    }
    #[test]
    fn test_frame_tag_looping() {
        let json = test_read_file!("frame_tag.json");

        let reference = TaggedSprites {
            name: "Blink".to_string(),
            from: 2,
            to: 7,
            direction: "forward".to_string(),
            color: Srgba::hex("#FFFFFFFF").unwrap(),
            looping: 0,
        };

        assert_data_of_type(json, reference);
    }

    #[test]
    fn test_meta_pixquare() {
        let json = r#"{
    "app": "Pixquare",
    "version": "1",
    "image": "Crow.png",
    "format": "RGBA8888",
    "size": { "w": 512, "h": 64 },
    "scale": 1,
    "frameTags": [],
    "layers": [],
    "slices": []
  }"#;
        let matcher = Meta {
            app: "Pixquare".to_string(),
            version: "1".to_string(),
            format: "RGBA8888".to_string(),
            scale: 1,
            image: "Crow.png".to_string(),
            size: UVec2::from(Size { h: 64, w: 512 }),
            frame_tags: Vec::new(),
        };
        assert_data_of_type(json, matcher);
    }
    #[test]
    fn test_meta_asprite() {
        let json = r#"{
    "app": "Pixquare",
    "version": "1",
    "image": "Crow.png",
    "format": "RGBA8888",
    "size": { "w": 512, "h": 64 },
    "scale": "1",
    "frameTags": [],
    "layers": [],
    "slices": []
  }"#;
        let matcher = Meta {
            app: "Pixquare".to_string(),
            version: "1".to_string(),
            format: "RGBA8888".to_string(),
            scale: 1,
            image: "Crow.png".to_string(),
            size: UVec2::from(Size { h: 64, w: 512 }),
            frame_tags: Vec::new(),
        };
        assert_data_of_type(json, matcher);
    }

    #[test]
    fn test_vec_from_size() {
        let size = Size { w: 512, h: 64 };
        let out = UVec2::from(size);
        assert_eq!(UVec2::new(512, 64), out);
    }

    #[test]
    fn test_box_from_frame() {
        let frame = Frame {
            h: 64,
            w: 100,
            x: 0,
            y: 0,
        };
        let out = URect::from(frame);
        assert_eq!(URect::new(0, 0, 100, 64), out);
        assert!(!out.is_empty());
        assert_eq!(out.width(), 100);
        assert_eq!(out.height(), 64);
    }

    #[test]
    fn test_box_from_frame_offset_x() {
        let frame = Frame {
            x: 448,
            y: 0,
            h: 64,
            w: 64,
        };
        let out = URect::from(frame);
        assert_eq!(URect::new(448, 0, 512, 64), out);
        assert!(!out.is_empty());
        assert_eq!(out.width(), 64);
        assert_eq!(out.height(), 64);
    }
}

// endregion
