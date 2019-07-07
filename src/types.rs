use crate::color::*;
use crate::texture::PIXEL_SIZE;
use crate::utility::unordered_tracker::*;
use cgmath::*;

// ////////////////////////////////////////////////////////
// Window
// ////////////////////////////////////////////////////////

/// Configuration for the window.
#[derive(Debug, Clone)]
pub struct WindowSettings {
    /// The title of the window.
    pub title: String,
    /// The size of the window.
    pub size: Vector2<i32>,
    /// Flag for if the window is resizable.
    pub resizable: bool,
}

// ////////////////////////////////////////////////////////
// Audio
// ////////////////////////////////////////////////////////

/// Configuration for audio.
#[derive(Debug, Copy, Clone)]
pub struct AudioDesc {}

// ////////////////////////////////////////////////////////
// Batch
// ////////////////////////////////////////////////////////

/// Handle to reference a batch with.
#[derive(Copy, Clone, Debug)]
pub struct BatchToken {
    key: Key<BatchToken>,
}

impl BatchToken {
    pub(crate) fn new(key: Key<BatchToken>) -> BatchToken {
        BatchToken {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> Key<BatchToken> {
        self.key
    }
}

/// Configuration description for a sprite.
#[derive(Copy, Clone, Debug)]
pub struct BatchSettings {
    pub translation: Vector2<f32>,
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
    pub visible: bool,
}

impl Default for BatchSettings {
    fn default() -> BatchSettings {
        BatchSettings {
            translation: Vector2::new(0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
            visible: true,
        }
    }
}

// ////////////////////////////////////////////////////////
// Sprite
// ////////////////////////////////////////////////////////

/// Configuration description for a sprite.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    // Units are measured in pixels.
    pub pos: Vector3<f32>,
    // Units are measured in pixels.
    pub size: Vector2<u16>,
    pub texture: Texture,
    pub color: RGBA8,
    // Units are 1/65536th of a turn.
    pub rotation: u16,
}

impl Default for Sprite {
    fn default() -> Sprite {
        Sprite {
            pos: Vector3::new(0.0, 0.0, 0.0),
            size: Vector2::new(100, 100),
            texture: Texture::default(),
            color: WHITE,
            rotation: 0,
        }
    }
}

impl Sprite {
    pub fn new(
        pos: Vector3<f32>,
        size: Vector2<f32>,
        texture: Texture,
        color: RGBA8,
        rotation: f32,
    ) -> Sprite {
        Sprite {
            pos: pos,
            size: {
                let x = (size.x as u32) & 0xFFFF;
                let y = (size.y as u32) & 0xFFFF;
                Vector2::new(x as u16, y as u16)
            },
            texture: texture,
            color: color,
            rotation: (rotation.fract() * 65536.0) as u16,
        }
    }
}

// ////////////////////////////////////////////////////////
// Text
// ////////////////////////////////////////////////////////

/// Handle to reference an uploaded font with.
#[derive(Copy, Clone, Debug)]
pub struct FontToken {
    key: usize,
}

impl FontToken {
    pub(crate) fn new(key: usize) -> FontToken {
        FontToken {
            key: key,
        }
    }

    pub(crate) fn key(&self) -> usize {
        self.key
    }
}

impl Default for FontToken {
    // The engine default font.
    fn default() -> FontToken {
        FontToken {
            key: 0,
        }
    }
}

/// Configuration description for text.
#[derive(Clone, Debug)]
pub struct Text {
    pub string: String,
    // Units are measured in pixels.
    pub pos: Vector3<f32>,
    pub max_width: Option<f32>,
    pub scale: u32,
    pub color: RGBA8,
    pub font: FontToken,
}

impl Default for Text {
    fn default() -> Text {
        Text {
            string: String::new(),
            pos: Vector3::new(0f32, 0f32, 0f32),
            max_width: None,
            scale: 24,
            color: BLACK,
            font: FontToken::default(),
        }
    }
}

impl Text {
    pub fn new(
        string: String,
        pos: Vector3<f32>,
        max_width: Option<f32>,
        scale: u32,
        color: RGBA8,
        font: FontToken,
    ) -> Text {
        Text {
            string: string,
            pos: pos,
            max_width: max_width,
            scale: scale,
            color: color,
            font: font,
        }
    }

    pub fn set_string(&mut self, string: &str) {
        self.string.clear();
        self.string.push_str(&string);
    }
}

// ////////////////////////////////////////////////////////
// Texture
// ////////////////////////////////////////////////////////

/// Handle to reference an uploaded texture with.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Texture(pub(crate) Vector4<u16>);

impl Default for Texture {
    /// A default texture reference for a basic white square.
    fn default() -> Texture {
        Texture(Vector4::new(0, PIXEL_SIZE as u16, 0, PIXEL_SIZE as u16))
    }
}

impl Texture {
    /// Mirrors the texture along the Y axis. Creates a new texture.
    pub fn mirror_y(&self) -> Texture {
        Texture(Vector4::new(self.0.y, self.0.x, self.0.z, self.0.w))
    }

    /// Mirrors the texture along the X axis. Creates a new texture.
    pub fn mirror_x(&self) -> Texture {
        Texture(Vector4::new(self.0.x, self.0.y, self.0.w, self.0.z))
    }

    /// Returns a sub texture from the given texture. Values are in pixels. The top left of the
    /// texture has the coordinates of 0, 0.
    ///
    /// Returns an error if the size is 0, or the bounds of the sub texture are outside of the
    /// original texture.
    pub fn sub_texture(
        &self,
        minx: u16,
        miny: u16,
        width: u16,
        height: u16,
    ) -> Result<Texture, &'static str> {
        if width == 0 || height == 0 {
            Err("Size must be greater than 0")?
        }

        // UV Layout: xmin xmax ymin ymax
        let bounds = Vector4::new(
            std::cmp::min(self.0.x, self.0.y), // Left
            std::cmp::max(self.0.x, self.0.y), // Right
            std::cmp::min(self.0.z, self.0.w), // Top
            std::cmp::max(self.0.z, self.0.w), // Bottom
        );
        let subset = Vector4::new(
            bounds.x + (minx) * (PIXEL_SIZE as u16),          // Left
            bounds.x + (minx + width) * (PIXEL_SIZE as u16),  // Right
            bounds.z + (miny) * (PIXEL_SIZE as u16),          // Top
            bounds.z + (miny + height) * (PIXEL_SIZE as u16), // Bottom
        );

        if subset.x > bounds.y || subset.y > bounds.y || subset.z > bounds.w || subset.w > bounds.w {
            Err("Requested subtexture is outside the bounds of the source texture.")?
        }

        Ok(Texture(subset))
    }
}
