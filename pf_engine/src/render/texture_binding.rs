use super::{PipelineState,GPUTexture,GpuTextureKind,PixelKind,FrameworkError};
use glow::{HasContext, COMPRESSED_RED_RGTC1, COMPRESSED_RG_RGTC2};

pub struct TextureBinding<'a> {
    pub(crate)state: &'a mut PipelineState,
    pub(crate)texture: &'a mut GPUTexture,
}

impl<'a> TextureBinding<'a>{
    pub fn set_data(
        self,
        kind: GpuTextureKind,
        pixel_kind: PixelKind,
        mip_count: usize,
        data: Option<&[u8]>,
    ) -> Result<Self, FrameworkError> {
        let mip_count = mip_count.max(1);

        let mut desired_byte_count = 0;

        'mip_loop: for mip in 0..mip_count {
            match kind {
                GpuTextureKind::Line { length } => {
                    if let Some(length) = length.checked_shr(mip as u32) {
                        desired_byte_count += image_1d_size_bytes(pixel_kind, length);
                    } else {
                        break 'mip_loop;
                    }
                }
                GpuTextureKind::Rectangle { width, height } => {
                    if let (Some(width), Some(height)) = (
                        width.checked_shr(mip as u32),
                        height.checked_shr(mip as u32),
                    ) {
                        desired_byte_count += image_2d_size_bytes(pixel_kind, width, height);
                    } else {
                        break 'mip_loop;
                    }
                }
                GpuTextureKind::Cube { width, height } => {
                    if let (Some(width), Some(height)) = (
                        width.checked_shr(mip as u32),
                        height.checked_shr(mip as u32),
                    ) {
                        desired_byte_count += 6 * image_2d_size_bytes(pixel_kind, width, height);
                    } else {
                        break 'mip_loop;
                    }
                }
                GpuTextureKind::Volume {
                    width,
                    height,
                    depth,
                } => {
                    if let (Some(width), Some(height), Some(depth)) = (
                        width.checked_shr(mip as u32),
                        height.checked_shr(mip as u32),
                        depth.checked_shr(mip as u32),
                    ) {
                        desired_byte_count += image_3d_size_bytes(pixel_kind, width, height, depth);
                    } else {
                        break 'mip_loop;
                    }
                }
            };
        }

        if let Some(data) = data {
            let actual_data_size = data.len();
            if actual_data_size != desired_byte_count {
                return Err(FrameworkError::InvalidTextureData {
                    expected_data_size: desired_byte_count,
                    actual_data_size,
                });
            }
        }

        self.texture.kind = kind;
        self.texture.pixel_kind = pixel_kind;

        let target = kind.gl_texture_target();

        unsafe {
            self.state
                .set_texture(0, target, Some(self.texture.texture));

            let (type_, format, internal_format) = match pixel_kind {
                PixelKind::F32 => (glow::FLOAT, glow::RED, glow::R32F),
                PixelKind::F16 => (glow::FLOAT, glow::RED, glow::R16F),
                PixelKind::D32F => (glow::FLOAT, glow::DEPTH_COMPONENT, glow::DEPTH_COMPONENT32F),
                PixelKind::D16 => (
                    glow::UNSIGNED_SHORT,
                    glow::DEPTH_COMPONENT,
                    glow::DEPTH_COMPONENT16,
                ),
                PixelKind::D24S8 => (
                    glow::UNSIGNED_INT_24_8,
                    glow::DEPTH_STENCIL,
                    glow::DEPTH24_STENCIL8,
                ),
                PixelKind::RGBA8 => (glow::UNSIGNED_BYTE, glow::RGBA, glow::RGBA8),
                PixelKind::SRGBA8 => (glow::UNSIGNED_BYTE, glow::RGBA, glow::SRGB8_ALPHA8),
                PixelKind::RGB8 => (glow::UNSIGNED_BYTE, glow::RGB, glow::RGB8),
                PixelKind::SRGB8 => (glow::UNSIGNED_BYTE, glow::RGB, glow::SRGB8),
                PixelKind::RG8 => (glow::UNSIGNED_BYTE, glow::RG, glow::RG8),
                PixelKind::R8 => (glow::UNSIGNED_BYTE, glow::RED, glow::R8),
                PixelKind::R8UI => (glow::UNSIGNED_BYTE, glow::RED_INTEGER, glow::R8UI),
                PixelKind::BGRA8 => (glow::UNSIGNED_BYTE, glow::BGRA, glow::RGBA8),
                PixelKind::BGR8 => (glow::UNSIGNED_BYTE, glow::BGR, glow::RGB8),
                PixelKind::RG16 => (glow::UNSIGNED_SHORT, glow::RG, glow::RG16),
                PixelKind::R16 => (glow::UNSIGNED_SHORT, glow::RED, glow::R16),
                PixelKind::RGB16 => (glow::UNSIGNED_SHORT, glow::RGB, glow::RGB16),
                PixelKind::RGBA16 => (glow::UNSIGNED_SHORT, glow::RGBA, glow::RGBA16),
                PixelKind::RGB10A2 => (
                    glow::UNSIGNED_INT_2_10_10_10_REV,
                    glow::RGBA,
                    glow::RGB10_A2,
                ),
                PixelKind::DXT1RGB => (0, 0, GL_COMPRESSED_RGB_S3TC_DXT1_EXT),
                PixelKind::DXT1RGBA => (0, 0, GL_COMPRESSED_RGBA_S3TC_DXT1_EXT),
                PixelKind::DXT3RGBA => (0, 0, GL_COMPRESSED_RGBA_S3TC_DXT3_EXT),
                PixelKind::DXT5RGBA => (0, 0, GL_COMPRESSED_RGBA_S3TC_DXT5_EXT),
                PixelKind::R8RGTC => (0, 0, COMPRESSED_RED_RGTC1),
                PixelKind::RG8RGTC => (0, 0, COMPRESSED_RG_RGTC2),
                PixelKind::RGB32F => (glow::FLOAT, glow::RGB, glow::RGB32F),
                PixelKind::RGBA32F => (glow::FLOAT, glow::RGBA, glow::RGBA32F),
                PixelKind::RGBA16F => (glow::FLOAT, glow::RGBA, glow::RGBA16F),
                PixelKind::R11G11B10F => (glow::FLOAT, glow::RGB, glow::R11F_G11F_B10F),
            };

            let is_compressed = pixel_kind.is_compressed();

            if let Some(alignment) = pixel_kind.unpack_alignment() {
                self.state
                    .gl
                    .pixel_store_i32(glow::UNPACK_ALIGNMENT, alignment);
            }

            let mut mip_byte_offset = 0;
            'mip_loop2: for mip in 0..mip_count {
                match kind {
                    GpuTextureKind::Line { length } => {
                        if let Some(length) = length.checked_shr(mip as u32) {
                            let size = image_1d_size_bytes(pixel_kind, length) as i32;
                            let pixels = data.map(|data| {
                                &data[mip_byte_offset..(mip_byte_offset + size as usize)]
                            });

                            if is_compressed {
                                self.state.gl.compressed_tex_image_1d(
                                    glow::TEXTURE_1D,
                                    mip as i32,
                                    internal_format as i32,
                                    length as i32,
                                    0,
                                    size,
                                    pixels.ok_or(FrameworkError::EmptyTextureData)?,
                                );
                            } else {
                                self.state.gl.tex_image_1d(
                                    glow::TEXTURE_1D,
                                    mip as i32,
                                    internal_format as i32,
                                    length as i32,
                                    0,
                                    format,
                                    type_,
                                    pixels,
                                );
                            }

                            mip_byte_offset += size as usize;
                        } else {
                            // No need to add degenerated mips (0x1, 0x2, 4x0, etc).
                            break 'mip_loop2;
                        }
                    }
                    GpuTextureKind::Rectangle { width, height } => {
                        if let (Some(width), Some(height)) = (
                            width.checked_shr(mip as u32),
                            height.checked_shr(mip as u32),
                        ) {
                            let size = image_2d_size_bytes(pixel_kind, width, height) as i32;
                            let pixels = data.map(|data| {
                                &data[mip_byte_offset..(mip_byte_offset + size as usize)]
                            });

                            if is_compressed {
                                self.state.gl.compressed_tex_image_2d(
                                    glow::TEXTURE_2D,
                                    mip as i32,
                                    internal_format as i32,
                                    width as i32,
                                    height as i32,
                                    0,
                                    size,
                                    pixels.ok_or(FrameworkError::EmptyTextureData)?,
                                );
                            } else {
                                self.state.gl.tex_image_2d(
                                    glow::TEXTURE_2D,
                                    mip as i32,
                                    internal_format as i32,
                                    width as i32,
                                    height as i32,
                                    0,
                                    format,
                                    type_,
                                    pixels,
                                );
                            }

                            mip_byte_offset += size as usize;
                        } else {
                            // No need to add degenerated mips (0x1, 0x2, 4x0, etc).
                            break 'mip_loop2;
                        }
                    }
                    GpuTextureKind::Cube { width, height } => {
                        if let (Some(width), Some(height)) = (
                            width.checked_shr(mip as u32),
                            height.checked_shr(mip as u32),
                        ) {
                            let bytes_per_face = image_2d_size_bytes(pixel_kind, width, height);

                            for face in 0..6 {
                                let begin = mip_byte_offset + face * bytes_per_face;
                                let end = mip_byte_offset + (face + 1) * bytes_per_face;
                                let face_pixels = data.map(|data| &data[begin..end]);

                                if is_compressed {
                                    self.state.gl.compressed_tex_image_2d(
                                        glow::TEXTURE_CUBE_MAP_POSITIVE_X + face as u32,
                                        mip as i32,
                                        internal_format as i32,
                                        width as i32,
                                        height as i32,
                                        0,
                                        bytes_per_face as i32,
                                        face_pixels.ok_or(FrameworkError::EmptyTextureData)?,
                                    );
                                } else {
                                    self.state.gl.tex_image_2d(
                                        glow::TEXTURE_CUBE_MAP_POSITIVE_X + face as u32,
                                        mip as i32,
                                        internal_format as i32,
                                        width as i32,
                                        height as i32,
                                        0,
                                        format,
                                        type_,
                                        face_pixels,
                                    );
                                }
                            }

                            mip_byte_offset += 6 * bytes_per_face as usize;
                        } else {
                            // No need to add degenerated mips (0x1, 0x2, 4x0, etc).
                            break 'mip_loop2;
                        }
                    }
                    GpuTextureKind::Volume {
                        width,
                        height,
                        depth,
                    } => {
                        if let (Some(width), Some(height), Some(depth)) = (
                            width.checked_shr(mip as u32),
                            height.checked_shr(mip as u32),
                            depth.checked_shr(mip as u32),
                        ) {
                            let size = image_3d_size_bytes(pixel_kind, width, height, depth) as i32;
                            let pixels = data.map(|data| {
                                &data[mip_byte_offset..(mip_byte_offset + size as usize)]
                            });

                            if is_compressed {
                                self.state.gl.compressed_tex_image_3d(
                                    glow::TEXTURE_3D,
                                    0,
                                    internal_format as i32,
                                    width as i32,
                                    height as i32,
                                    depth as i32,
                                    0,
                                    size,
                                    pixels.ok_or(FrameworkError::EmptyTextureData)?,
                                );
                            } else {
                                self.state.gl.tex_image_3d(
                                    glow::TEXTURE_3D,
                                    0,
                                    internal_format as i32,
                                    width as i32,
                                    height as i32,
                                    depth as i32,
                                    0,
                                    format,
                                    type_,
                                    pixels,
                                );
                            }

                            mip_byte_offset += size as usize;
                        } else {
                            // No need to add degenerated mips (0x1, 0x2, 4x0, etc).
                            break 'mip_loop2;
                        }
                    }
                }
            }
        }

        Ok(self)
    }
}

fn image_1d_size_bytes(pixel_kind: PixelKind, length: usize) -> usize {
    match pixel_kind {
        PixelKind::RGBA32F => 16 * length,
        PixelKind::RGB32F => 12 * length,
        PixelKind::RGBA16 | PixelKind::RGBA16F => 8 * length,
        PixelKind::RGB16 => 6 * length,
        PixelKind::RGBA8
        | PixelKind::SRGBA8
        | PixelKind::BGRA8
        | PixelKind::RG16
        | PixelKind::D24S8
        | PixelKind::D32F
        | PixelKind::F32
        | PixelKind::R11G11B10F
        | PixelKind::RGB10A2 => 4 * length,
        PixelKind::RGB8 | PixelKind::SRGB8 | PixelKind::BGR8 => 3 * length,
        PixelKind::RG8 | PixelKind::R16 | PixelKind::D16 | PixelKind::F16 => 2 * length,
        PixelKind::R8 | PixelKind::R8UI => length,
        PixelKind::DXT1RGB | PixelKind::DXT1RGBA | PixelKind::R8RGTC => {
            let block_size = 8;
            ceil_div_4(length) * block_size
        }
        PixelKind::DXT3RGBA | PixelKind::DXT5RGBA | PixelKind::RG8RGTC => {
            let block_size = 16;
            ceil_div_4(length) * block_size
        }
    }
}

fn image_2d_size_bytes(pixel_kind: PixelKind, width: usize, height: usize) -> usize {
    let pixel_count = width * height;
    match pixel_kind {
        PixelKind::RGBA32F => 16 * pixel_count,
        PixelKind::RGB32F => 12 * pixel_count,
        PixelKind::RGBA16 | PixelKind::RGBA16F => 8 * pixel_count,
        PixelKind::RGB16 => 6 * pixel_count,
        PixelKind::RGBA8
        | PixelKind::SRGBA8
        | PixelKind::BGRA8
        | PixelKind::RG16
        | PixelKind::D24S8
        | PixelKind::D32F
        | PixelKind::F32
        | PixelKind::R11G11B10F
        | PixelKind::RGB10A2 => 4 * pixel_count,
        PixelKind::RGB8 | PixelKind::SRGB8 | PixelKind::BGR8 => 3 * pixel_count,
        PixelKind::RG8 | PixelKind::R16 | PixelKind::D16 | PixelKind::F16 => 2 * pixel_count,
        PixelKind::R8 | PixelKind::R8UI => pixel_count,
        PixelKind::DXT1RGB | PixelKind::DXT1RGBA | PixelKind::R8RGTC => {
            let block_size = 8;
            ceil_div_4(width) * ceil_div_4(height) * block_size
        }
        PixelKind::DXT3RGBA | PixelKind::DXT5RGBA | PixelKind::RG8RGTC => {
            let block_size = 16;
            ceil_div_4(width) * ceil_div_4(height) * block_size
        }
    }
}

fn image_3d_size_bytes(pixel_kind: PixelKind, width: usize, height: usize, depth: usize) -> usize {
    let pixel_count = width * height * depth;
    match pixel_kind {
        PixelKind::RGBA32F => 16 * pixel_count,
        PixelKind::RGB32F => 12 * pixel_count,
        PixelKind::RGBA16 | PixelKind::RGBA16F => 8 * pixel_count,
        PixelKind::RGB16 => 6 * pixel_count,
        PixelKind::RGBA8
        | PixelKind::SRGBA8
        | PixelKind::BGRA8
        | PixelKind::RG16
        | PixelKind::D24S8
        | PixelKind::D32F
        | PixelKind::F32
        | PixelKind::R11G11B10F
        | PixelKind::RGB10A2 => 4 * pixel_count,
        PixelKind::RGB8 | PixelKind::SRGB8 | PixelKind::BGR8 => 3 * pixel_count,
        PixelKind::RG8 | PixelKind::R16 | PixelKind::D16 | PixelKind::F16 => 2 * pixel_count,
        PixelKind::R8 | PixelKind::R8UI => pixel_count,
        PixelKind::DXT1RGB | PixelKind::DXT1RGBA | PixelKind::R8RGTC => {
            let block_size = 8;
            ceil_div_4(width) * ceil_div_4(height) * ceil_div_4(depth) * block_size
        }
        PixelKind::DXT3RGBA | PixelKind::DXT5RGBA | PixelKind::RG8RGTC => {
            let block_size = 16;
            ceil_div_4(width) * ceil_div_4(height) * ceil_div_4(depth) * block_size
        }
    }
}

const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 0x83F0;
const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 0x83F1;
const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 0x83F2;
const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 0x83F3;

fn ceil_div_4(x: usize) -> usize {
    (x + 3) / 4
}
