use super::{TextureMinificationFilter,TextureMagnificationFilter};

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Hash,Debug)]
#[repr(u32)]
pub enum MinificationFilter {
    Nearest = glow::NEAREST,
    NearestMipMapNearest = glow::NEAREST_MIPMAP_NEAREST,
    NearestMipMapLinear = glow::NEAREST_MIPMAP_LINEAR,
    Linear = glow::LINEAR,
    LinearMipMapNearest = glow::LINEAR_MIPMAP_NEAREST,
    LinearMipMapLinear = glow::LINEAR_MIPMAP_LINEAR,
}

impl MinificationFilter {
    pub fn into_gl_value(self) -> i32 {
        self as i32
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Hash,Debug)]
#[repr(u32)]
pub enum MagnificationFilter {
    Nearest,
    Linear,
}

impl MagnificationFilter {
    pub fn into_gl_value(self) -> i32 {
        (match self {
            Self::Nearest => glow::NEAREST,
            Self::Linear => glow::LINEAR,
        }) as i32
    }
}


#[derive(Copy, Clone, Eq, PartialEq,Debug)]
#[repr(u32)]
pub enum WrapMode {
    Repeat = glow::REPEAT,
    ClampToEdge = glow::CLAMP_TO_EDGE,
    ClampToBorder = glow::CLAMP_TO_BORDER,
    MirroredRepeat = glow::MIRRORED_REPEAT,
    MirrorClampToEdge = glow::MIRROR_CLAMP_TO_EDGE,
}

impl From<TextureMinificationFilter> for MinificationFilter {
    fn from(v: TextureMinificationFilter) -> Self {
        match v {
            TextureMinificationFilter::Nearest => Self::Nearest,
            TextureMinificationFilter::NearestMipMapNearest => Self::NearestMipMapNearest,
            TextureMinificationFilter::NearestMipMapLinear => Self::NearestMipMapLinear,
            TextureMinificationFilter::Linear => Self::Linear,
            TextureMinificationFilter::LinearMipMapNearest => Self::LinearMipMapNearest,
            TextureMinificationFilter::LinearMipMapLinear => Self::LinearMipMapLinear,
        }
    }
}

impl From<TextureMagnificationFilter> for MagnificationFilter {
    fn from(v: TextureMagnificationFilter) -> Self {
        match v {
            TextureMagnificationFilter::Nearest => Self::Nearest,
            TextureMagnificationFilter::Linear => Self::Linear,
        }
    }
}
