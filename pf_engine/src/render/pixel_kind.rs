#[derive(Copy, Clone, Debug)]
pub enum PixelKind {
    F32,
    F16,
    D32F,
    D16,
    D24S8,
    RGBA8,
    SRGBA8,
    RGB8,
    SRGB8,
    BGRA8,
    BGR8,
    RG8,
    RG16,
    R8,
    R8UI,
    R16,
    RGB16,
    RGBA16,
    DXT1RGB,
    DXT1RGBA,
    DXT3RGBA,
    DXT5RGBA,
    RGB32F,
    RGBA32F,
    RGBA16F,
    R8RGTC,
    RG8RGTC,
    R11G11B10F,
    RGB10A2,
}

impl PixelKind{
    pub fn is_compressed(self) -> bool {
        match self {
            Self::DXT1RGB
            | Self::DXT1RGBA
            | Self::DXT3RGBA
            | Self::DXT5RGBA
            | Self::R8RGTC
            | Self::RG8RGTC => true,
            // Explicit match for rest of formats instead of _ will help to not forget
            // to add new entry here.
            Self::RGBA16
            | Self::RGBA16F
            | Self::RGB16
            | Self::RGBA8
            | Self::SRGBA8
            | Self::RGB8
            | Self::SRGB8
            | Self::BGRA8
            | Self::BGR8
            | Self::RG16
            | Self::R16
            | Self::D24S8
            | Self::D32F
            | Self::F32
            | Self::RG8
            | Self::D16
            | Self::F16
            | Self::R8
            | Self::R8UI
            | Self::RGB32F
            | Self::RGBA32F
            | Self::R11G11B10F
            | Self::RGB10A2 => false,
        }
    }

    pub fn unpack_alignment(self) -> Option<i32> {
        match self {
            Self::RGBA16 | Self::RGBA16F | Self::RGB16 | Self::RGBA32F | Self::RGB32F => Some(8),
            Self::RGBA8
            | Self::SRGBA8
            | Self::RGB8
            | Self::SRGB8
            | Self::BGRA8
            | Self::BGR8
            | Self::RG16
            | Self::R16
            | Self::D24S8
            | Self::D32F
            | Self::F32
            | Self::R11G11B10F
            | Self::RGB10A2 => Some(4),
            Self::RG8 | Self::D16 | Self::F16 => Some(2),
            Self::R8 | Self::R8UI => Some(1),
            Self::DXT1RGB
            | Self::DXT1RGBA
            | Self::DXT3RGBA
            | Self::DXT5RGBA
            | Self::R8RGTC
            | Self::RG8RGTC => None,
        }
    }
}
