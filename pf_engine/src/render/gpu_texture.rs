use crate::render::state::PipelineState;
use super::{PixelKind,MinificationFilter,MagnificationFilter,FrameworkError,WrapMode,TextureBinding,TextureKind};
use glow::HasContext;

#[derive(Debug)]
pub struct GPUTexture{
    pub(crate)state: *mut PipelineState,
    pub(crate)texture: glow::Texture,
    pub(crate)kind: GpuTextureKind,
    pub(crate)min_filter: MinificationFilter,
    pub(crate)mag_filter: MagnificationFilter,
    pub(crate)s_wrap_mode: WrapMode,
    pub(crate)t_wrap_mode: WrapMode,
    pub(crate)r_wrap_mode: WrapMode,
    pub(crate)anisotropy: f32, // TODO 
    pub(crate)pixel_kind: PixelKind,
    // Force compiler to not implement Send and Sync, because OpenGL is not thread-safe.
    //thread_mark: PhantomData<*const u8>,
}

#[derive(Copy, Clone,Debug)]
pub enum GpuTextureKind {
    Line {
        length: usize,
    },
    Rectangle {
        width: usize,
        height: usize,
    },
    Cube {
        width: usize,
        height: usize,
    },
    Volume {
        width: usize,
        height: usize,
        depth: usize,
    },
}

impl From<TextureKind> for GpuTextureKind {
    fn from(v: TextureKind) -> Self {
        match v {
            TextureKind::Line { length } => GpuTextureKind::Line {
                length: length as usize,
            },
            TextureKind::Rectangle { width, height } => GpuTextureKind::Rectangle {
                width: width as usize,
                height: height as usize,
            },
            TextureKind::Cube { width, height } => GpuTextureKind::Cube {
                width: width as usize,
                height: height as usize,
            },
            TextureKind::Volume {
                width,
                height,
                depth,
            } => GpuTextureKind::Volume {
                width: width as usize,
                height: height as usize,
                depth: depth as usize,
            },
        }
    }
}


impl GpuTextureKind {
    pub(crate)fn gl_texture_target(&self) -> u32 {
        match self {
            Self::Line { .. } => glow::TEXTURE_1D,
            Self::Rectangle { .. } => glow::TEXTURE_2D,
            Self::Cube { .. } => glow::TEXTURE_CUBE_MAP,
            Self::Volume { .. } => glow::TEXTURE_3D,
        }
    }
}

impl GPUTexture {
    pub fn new(
        state: &mut PipelineState,
        kind: GpuTextureKind,
        pixel_kind: PixelKind,
        min_filter: MinificationFilter,
        mag_filter: MagnificationFilter,
        mip_count: usize,
        data: Option<&[u8]>,
        )->Result<Self,FrameworkError>{
        let mip_count = mip_count.max(1);

        let target = kind.gl_texture_target();

        unsafe{
            let texture = state.gl.create_texture()?;

            let mut result = Self {
                state,
                texture,
                kind,
                min_filter,
                mag_filter,
                s_wrap_mode: WrapMode::Repeat,
                t_wrap_mode: WrapMode::Repeat,
                r_wrap_mode: WrapMode::Repeat,
                anisotropy: 1.0,
                pixel_kind,
                //thread_mark: PhantomData,
            };

            TextureBinding {
                state,
                texture: &mut result,
            }
            .set_data(kind, pixel_kind, mip_count, data)?;

            state.gl.tex_parameter_i32(
                target,
                glow::TEXTURE_MAG_FILTER,
                mag_filter.into_gl_value(),
            );
            state.gl.tex_parameter_i32(
                target,
                glow::TEXTURE_MIN_FILTER,
                min_filter.into_gl_value(),
            );

            state
                .gl
                .tex_parameter_i32(target, glow::TEXTURE_MAX_LEVEL, mip_count as i32 - 1);

            state.set_texture(0, target, Default::default());


            Ok(result)
        }
    }

    pub fn bind(&self, state: &mut PipelineState, sampler_index: u32) {
        state.set_texture(
            sampler_index,
            self.kind.gl_texture_target(),
            Some(self.texture),
        );
    }
}


