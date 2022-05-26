use super::gpu_texture::GPUTexture;
use super::{PixelKind};
use std::ops::Deref;
use bevy::reflect::TypeUuid;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::path::PathBuf;
use std::ops::DerefMut;

#[derive(TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5053"]
pub struct Texture {
    pub(crate)gpu_texutre: Option<GPUTexture>,
    pub(crate)data: Option<TextureData>,
}

impl Texture {
    pub fn from_texture_data(data: TextureData)->Self{
        Self{
            gpu_texutre: None,
            data: Some(data),
        }
    }
}

impl Default for Texture{
    fn default()->Self{
        return Self{
           gpu_texutre:None,
            data:None,
        };
    }
}

unsafe impl Send for Texture{}
unsafe impl Sync for Texture{}

#[derive(Debug,TypeUuid)]
#[uuid = "7494888b-c082-457b-aacf-517228cc0c23"]
pub struct TextureData {
    pub(crate)path: PathBuf,
    pub(crate)kind: TextureKind,
    pub(crate)bytes: TextureBytes,
    pub(crate)pixel_kind: TexturePixelKind,
    pub(crate)minification_filter: TextureMinificationFilter,
    pub(crate)magnification_filter: TextureMagnificationFilter,
    pub(crate)s_wrap_mode: TextureWrapMode,
    pub(crate)t_wrap_mode: TextureWrapMode,
    pub(crate)mip_count: u32,
    pub(crate)anisotropy: f32,
    //serialize_content: bool,
    //data_hash: u64,
    pub(crate)is_render_target: bool,
}

impl TextureData{
    pub fn new(path:PathBuf,bytes: Vec<u8>)->Self{
        Self{
            path,
            kind: TextureKind::Cube{width:1,height:0},
            bytes: TextureBytes(bytes),
            pixel_kind: TexturePixelKind::RGB8,
            minification_filter: TextureMinificationFilter::Linear,
            magnification_filter: TextureMagnificationFilter::Linear,
            s_wrap_mode: TextureWrapMode::Repeat,
            t_wrap_mode: TextureWrapMode::Repeat,
            mip_count: 1,
            anisotropy:0.0,
            //serialize_content: bool,
            //data_hash: u64,
            is_render_target:false,
        }
    }
}


/// Texture kind.
#[derive(Copy, Clone, Debug)]
pub enum TextureKind {
    /// 1D texture.
    Line {
        /// Length of the texture.
        length: u32,
    },
    /// 2D texture.
    Rectangle {
        /// Width of the texture.
        width: u32,
        /// Height of the texture.
        height: u32,
    },
    /// Cube texture.
    Cube {
        /// Width of the cube face.
        width: u32,
        /// Height of the cube face.
        height: u32,
    },
    /// Volume texture (3D).
    Volume {
        /// Width of the volume.
        width: u32,
        /// Height of the volume.
        height: u32,
        /// Depth of the volume.
        depth: u32,
    },
}

impl Default for TextureKind {
    fn default() -> Self {
        Self::Rectangle {
            width: 0,
            height: 0,
        }
    }
}

#[derive(Default, Clone)]
pub(crate) struct TextureBytes(pub(crate)Vec<u8>);

impl Debug for TextureBytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Texture has {} bytes", self.0.len())
    }
}

impl From<Vec<u8>> for TextureBytes {
    fn from(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }
}

impl Deref for TextureBytes {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TextureBytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
/// Texture kind defines pixel format of texture.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TexturePixelKind {
    /// 1 byte red.
    R8 = 0,

    /// Red, green, and blue components, each by 1 byte.
    RGB8 = 1,

    /// Red, green, blue, and alpha components, each by 1 byte.
    RGBA8 = 2,

    /// Red and green, each by 1 byte.
    RG8 = 3,

    /// 2 byte red.
    R16 = 4,

    /// Red and green, each by 2 byte.
    RG16 = 5,

    /// Blue, green, and red components, each by 1 byte.
    BGR8 = 6,

    /// Blue, green, red and alpha components, each by 1 byte.
    BGRA8 = 7,

    /// Red, green, and blue components, each by 2 byte.
    RGB16 = 8,

    /// Red, green, blue, and alpha components, each by 2 byte.
    RGBA16 = 9,

    /// Compressed S3TC DXT1 RGB (no alpha).
    DXT1RGB = 10,

    /// Compressed S3TC DXT1 RGBA.
    DXT1RGBA = 11,

    /// Compressed S3TC DXT3 RGBA.
    DXT3RGBA = 12,

    /// Compressed S3TC DXT5 RGBA.
    DXT5RGBA = 13,

    /// Compressed R8 texture (RGTC).
    R8RGTC = 14,

    /// Compressed RG8 texture (RGTC).
    RG8RGTC = 15,

    /// Floating-point RGB texture with 32bit depth.
    RGB32F = 16,

    /// Floating-point RGBA texture with 32bit depth.
    RGBA32F = 17,
}

impl TexturePixelKind {
    fn new(id: u32) -> Result<Self, String> {
        match id {
            0 => Ok(Self::R8),
            1 => Ok(Self::RGB8),
            2 => Ok(Self::RGBA8),
            3 => Ok(Self::RG8),
            4 => Ok(Self::R16),
            5 => Ok(Self::RG16),
            6 => Ok(Self::BGR8),
            7 => Ok(Self::BGRA8),
            8 => Ok(Self::RGB16),
            9 => Ok(Self::RGBA16),
            10 => Ok(Self::DXT1RGB),
            11 => Ok(Self::DXT1RGBA),
            12 => Ok(Self::DXT3RGBA),
            13 => Ok(Self::DXT5RGBA),
            14 => Ok(Self::R8RGTC),
            15 => Ok(Self::RG8RGTC),
            16 => Ok(Self::RGB32F),
            17 => Ok(Self::RGBA32F),
            _ => Err(format!("Invalid texture kind {}!", id)),
        }
    }

    fn id(self) -> u32 {
        self as u32
    }
}

impl From<TexturePixelKind> for PixelKind {
    fn from(texture_kind: TexturePixelKind) -> Self {
        match texture_kind {
            TexturePixelKind::R8 => Self::R8,
            TexturePixelKind::RGB8 => Self::RGB8,
            TexturePixelKind::RGBA8 => Self::RGBA8,
            TexturePixelKind::RG8 => Self::RG8,
            TexturePixelKind::R16 => Self::R16,
            TexturePixelKind::RG16 => Self::RG16,
            TexturePixelKind::BGR8 => Self::BGR8,
            TexturePixelKind::BGRA8 => Self::BGRA8,
            TexturePixelKind::RGB16 => Self::RGB16,
            TexturePixelKind::RGBA16 => Self::RGBA16,
            TexturePixelKind::DXT1RGB => Self::DXT1RGB,
            TexturePixelKind::DXT1RGBA => Self::DXT1RGBA,
            TexturePixelKind::DXT3RGBA => Self::DXT3RGBA,
            TexturePixelKind::DXT5RGBA => Self::DXT5RGBA,
            TexturePixelKind::R8RGTC => Self::R8RGTC,
            TexturePixelKind::RG8RGTC => Self::RG8RGTC,
            TexturePixelKind::RGB32F => Self::RGB32F,
            TexturePixelKind::RGBA32F => Self::RGBA32F,
        }
    }
}

/// The texture minifying function is used whenever the pixel being textured maps to an area
/// greater than one texture element.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialOrd,
    PartialEq,
    )]
#[repr(u32)]
pub enum TextureMinificationFilter {
    /// Returns the value of the texture element that is nearest to the center of the pixel
    /// being textured.
    Nearest = 0,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and
    /// uses the Nearest criterion (the texture element nearest to the center of the pixel)
    /// to produce a texture value.
    NearestMipMapNearest = 1,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured
    /// and uses the Nearest criterion (the texture element nearest to the center of the pixel)
    /// to produce a texture value from each mipmap. The final texture value is a weighted average
    /// of those two values.
    NearestMipMapLinear = 2,

    /// Returns the weighted average of the four texture elements that are closest to the
    /// center of the pixel being textured.
    Linear = 3,

    /// Chooses the mipmap that most closely matches the size of the pixel being textured and
    /// uses the Linear criterion (a weighted average of the four texture elements that are
    /// closest to the center of the pixel) to produce a texture value.
    LinearMipMapNearest = 4,

    /// Chooses the two mipmaps that most closely match the size of the pixel being textured
    /// and uses the Linear criterion (a weighted average of the four texture elements that
    /// are closest to the center of the pixel) to produce a texture value from each mipmap.
    /// The final texture value is a weighted average of those two values.
    LinearMipMapLinear = 5,
}

impl TextureMinificationFilter {
    /// Returns true if minification filter is using mip mapping, false - otherwise.
    pub fn is_using_mip_mapping(self) -> bool {
        match self {
            TextureMinificationFilter::Nearest | TextureMinificationFilter::Linear => false,
            TextureMinificationFilter::NearestMipMapNearest
                | TextureMinificationFilter::LinearMipMapLinear
                | TextureMinificationFilter::NearestMipMapLinear
                | TextureMinificationFilter::LinearMipMapNearest => true,
        }
    }
}


#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialOrd,
    PartialEq,
    )]
#[repr(u32)]
pub enum TextureMagnificationFilter {
    /// Returns the value of the texture element that is nearest to the center of the pixel
    /// being textured.
    Nearest = 0,

    /// Returns the weighted average of the four texture elements that are closest to the
    /// center of the pixel being textured.
    Linear = 1,
}

impl Default for TextureMagnificationFilter {
    fn default() -> Self {
        Self::Linear
    }
}


/// Defines a law of texture coordinate modification.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    PartialOrd,
    PartialEq,
    )]
#[repr(u32)]
pub enum TextureWrapMode {
    /// Causes the integer part of a coordinate to be ignored; GPU uses only the fractional part,
    /// thereby creating a repeating pattern.
    Repeat = 0,

    /// Causes a coordinates to be clamped to the range range, where N is the size of the texture
    /// in the direction of clamping
    ClampToEdge = 1,

    /// Evaluates a coordinates in a similar manner to ClampToEdge. However, in cases where clamping
    /// would have occurred in ClampToEdge mode, the fetched texel data is substituted with the values
    /// specified by border color.
    ClampToBorder = 2,

    /// Causes the a coordinate to be set to the fractional part of the texture coordinate if the integer
    /// part of coordinate is even; if the integer part of coordinate is odd, then the coordinate texture
    /// coordinate is set to 1-frac, where frac represents the fractional part of coordinate.
    MirroredRepeat = 3,

    /// Causes a coordinate to be repeated as for MirroredRepeat for one repetition of the texture, at
    /// which point the coordinate to be clamped as in ClampToEdge.
    MirrorClampToEdge = 4,
}

impl Default for TextureWrapMode {
    fn default() -> Self {
        Self::Repeat
    }
}
