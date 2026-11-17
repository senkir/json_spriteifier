use bevy::math::UVec2;
use bevy::prelude::{Asset, Component, TypePath, URect};
use serde::de::{Error, MapAccess};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
// region schema

// Aseprite and Pixquare
// both can export using a `json` schema that is pretty similar
//
#[derive(Asset, TypePath, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpriteSheet {
    /// raw frame data
    #[serde(deserialize_with = "deserialize_sprite_sheet")]
    pub frames: Vec<SpriteSheetIndex>,
    /// useful metadata
    pub meta: Meta,
}

impl SpriteSheet {
    /// overall sheet size of source image
    pub fn sheet_size(&self) -> UVec2 {
        self.meta.size.as_vec()
    }

    pub fn sprite_size(&self) -> UVec2 {
        if self.frames.is_empty() {
            UVec2::ZERO
        } else {
            let frame = self.frames.first().unwrap();
            frame.source_size.as_vec()
        }
    }

    pub fn col_count(&self) -> u32 {
        let mut count: u32 = 0;
        let mut last_x: i32 = -1;
        for frame in self.frames.iter() {
            if last_x < frame.frame.x {
                count += 1;
                last_x = frame.frame.x
            }
        }
        count
    }

    pub fn row_count(&self) -> u32 {
        let mut count: u32 = 0;
        let mut last_x: i32 = -1;
        for frame in self.frames.iter() {
            if last_x < frame.frame.y {
                count += 1;
                last_x = frame.frame.y
            }
        }
        count
    }
}

/// individual image definitions within a sheet
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpriteSheetIndex {
    pub duration: usize,
    pub filename: Option<String>,
    pub frame: Frame,
    pub rotated: bool,
    pub source_size: Size,
    #[serde(rename = "spriteSourceSize")]
    pub sprited_source_size: Frame,
    trimmed: bool,
}

impl SpriteSheetIndex {

    pub fn rect(&self) -> URect {
        self.frame.rect()
    }
}

/// metadata
///
/// general information to organize frame data
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// the app that generated the json sprite sheet
    app: String,
    /// version of app that generated json
    version: String,
    /// color image format eg: RGBA8888
    format: String,
    /// tagged sections used in specific animations
    pub frame_tags: Vec<FrameTag>,
    /// related sheet image name
    pub image: String,
    /// scaling value
    #[serde(deserialize_with = "deserialize_scale")]
    pub scale: usize,
    /// image size for sheet
    pub size: Size,
}

#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
struct ColorTag(String);

/// tagged frame information
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FrameTag {
    /// tag name
    pub name: String,
    /// the color in hex
    pub color: ColorTag,
    /// render direction such as "forward"
    pub direction: String,
    /// start frame
    pub from: usize,
    /// end frame
    pub to: usize,

    /// looping flag
    /// Pixquare writes 0 = loop forever
    /// Asprite omits it entirely
    #[serde(default, rename = "loop", deserialize_with = "deserialize_loop")]
    pub looping: Option<usize>,
}

/// raw frame dimensions and start params
#[derive(Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Frame {
    pub h: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
}

impl Frame {
    pub fn size(&self) -> UVec2 {
        UVec2::new(self.w as u32, self.h as u32)
    }
    pub fn offset(&self) -> UVec2 {
        UVec2::new(self.x as u32, self.y as u32)
    }

    pub fn rect(&self) -> URect {
        URect::new(self.x as u32, self.y as u32, self.w as u32, self.h as u32)
    }
}

/// size data in pixels
#[derive(Deserialize, Debug, Eq, PartialEq, Clone, Copy)]
pub struct Size {
    pub h: i32,
    pub w: i32,
}

impl Size {
    const ZERO: Self = Self { h: 0, w: 0 };
    pub fn as_vec(&self) -> UVec2 {
        UVec2::new(self.w as u32, self.h as u32)
    }
}

// endregion

fn deserialize_loop<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: Deserializer<'de>,
{
    let option: Option<usize> = Option::deserialize(deserializer)?;
    Ok(option.map(|looped| looped))
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

fn deserialize_sprite_sheet<'de, D>(deserializer: D) -> Result<Vec<SpriteSheetIndex>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(SpriteSheetVisitor)
}

/// visitor to unwrap mapped frames into a linear `Vec`
struct SpriteSheetVisitor;

impl<'de> serde::de::Visitor<'de> for SpriteSheetVisitor {
    type Value = Vec<SpriteSheetIndex>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an array of spritesheets")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut frames: Vec<SpriteSheetIndex> = Vec::new();

        while let Some((_name, frame)) = map.next_entry::<String, SpriteSheetIndex>()? {
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

    fn assert_data_of_type<T>(json: &str, match_equals: T)
    where
        T: serde::de::DeserializeOwned + PartialEq + std::fmt::Debug,
    {
        let data: T = serde_json::from_str(json).expect("failed to parse json");

        assert_eq!(
            data, match_equals,
            "data doesnt match format. json: {}",
            json
        );
    }

    #[test]
    fn test_full_frame_1() {
        let json = test_read_file!("frame_1.json");
        let mut vec = Vec::new();
        vec.push(SpriteSheetIndex {
            duration: 100,
            filename: None,
            frame: Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            },
            rotated: false,
            source_size: Size { h: 64, w: 64 },
            sprited_source_size: Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            },
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
                size: Size { w: 512, h: 64 },
                frame_tags: tags,
            },
            frames: vec,
        };
        assert_data_of_type(&json, obj);
    }

    #[test]
    fn test_full_frame_alt_pixuare() {
        let json = test_read_file!("frame_pixquare.json");
        let mut vec = Vec::new();
        vec.push(SpriteSheetIndex {
            duration: 100,
            filename: Some("Crow fly 0".to_string()),
            frame: Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            },
            rotated: false,
            source_size: Size { h: 64, w: 64 },
            sprited_source_size: Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            },
            trimmed: false,
        });

        let mut tags = Vec::new();
        tags.push(FrameTag {
            direction: "forward".to_string(),
            from: 0,
            to: 6,
            name: "Takeoff".to_string(),
            color: ColorTag("#FFFFFFFF".to_string()),
            looping: Some(0),
        });
        let obj = SpriteSheet {
            meta: Meta {
                app: "Pixquare".to_string(),
                version: "2.74.1".to_string(),
                format: "RGBA8888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: Size { w: 512, h: 64 },
                frame_tags: tags,
            },
            frames: vec,
        };
        assert_data_of_type(json, obj);
    }

    fn build_sprite_index(x: i32) -> SpriteSheetIndex {
        SpriteSheetIndex {
            duration: 100,
            filename: None,
            frame: Frame {
                h: 64,
                w: 64,
                x,
                y: 0,
            },
            rotated: false,
            source_size: Size { h: 64, w: 64 },
            sprited_source_size: Frame {
                h: 64,
                w: 64,
                x: 0,
                y: 0,
            },
            trimmed: false,
        }
    }

    #[test]
    fn test_full_crow() {
        let json = test_read_file!("crow.json");
        let mut vec = Vec::new();
        vec.push(build_sprite_index(0));
        vec.push(build_sprite_index(64));

        let mut tags = Vec::new();
        tags.push(FrameTag {
            name: "Blink".to_string(),
            color: ColorTag("#ffffffff".to_string()),
            direction: "forward".to_string(),
            from: 2,
            to: 7,
            looping: None,
        });
        let obj = SpriteSheet {
            meta: Meta {
                app: "https://www.aseprite.org/".to_string(),
                version: "1.3.18.6-arm64-trial".to_string(),
                format: "RGBA8888".to_string(),
                image: "Crow.png".to_string(),
                scale: 1,
                size: Size { w: 512, h: 64 },
                frame_tags: tags,
            },
            frames: vec,
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
        let reference = SpriteSheetIndex {
            duration: 100,
            filename: None,
            rotated: false,
            frame: Frame {
                w: 64,
                h: 64,
                x: 0,
                y: 0,
            },
            source_size: Size { w: 64, h: 64 },
            sprited_source_size: Frame {
                w: 64,
                h: 64,
                x: 0,
                y: 0,
            },
            trimmed: false,
        };
        assert_data_of_type(json.as_str(), reference);
    }

    #[test]
    fn test_frame_tag() {
        let json = r#"{ "name": "Blink", "from": 2, "to": 7, "direction": "forward", "color": "ffffffff" }"#;

        let reference = FrameTag {
            name: "Blink".to_string(),
            from: 2,
            to: 7,
            direction: "forward".to_string(),
            color: ColorTag("ffffffff".to_string()), //TODO: should this use bevy white
            looping: None,
        };

        assert_data_of_type(json, reference);
    }
    #[test]
    fn test_frame_tag_looping_0() {
        let json = test_read_file!("frame_tag.json");

        let reference = FrameTag {
            name: "Blink".to_string(),
            from: 2,
            to: 7,
            direction: "forward".to_string(),
            color: ColorTag("#ffffffff".to_string()), //TODO: should this use bevy white
            looping: Some(0),
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
            size: Size { h: 64, w: 512 },
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
            size: Size { h: 64, w: 512 },
            frame_tags: Vec::new(),
        };
        assert_data_of_type(json, matcher);
    }
}

// endregion
