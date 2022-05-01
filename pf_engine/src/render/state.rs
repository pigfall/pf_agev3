use crate::core::{color::Color, math::Rect};

use glow::HasContext;


pub struct PipelineState {
    pub gl: glow::Context,

    blend: bool,

    depth_test: bool,
    depth_write: bool,
    depth_func: CompareFunc,

    color_write: ColorMask,
    stencil_test: bool,
    cull_face: CullFace,
    culling: bool,
    stencil_mask: u32,
    clear_color: Color,
    clear_stencil: i32,
    clear_depth: f32,
    scissor_test: bool,

    framebuffer: Option<glow::Framebuffer>,
    viewport: Rect<i32>,

    blend_func: BlendFunc,

    program: Option<glow::Program>,
    //texture_units: [TextureUnit; 32],

    //stencil_func: StencilFunc,
    //stencil_op: StencilOp,

    vao: Option<glow::VertexArray>,
    vbo: Option<glow::Buffer>,

    //frame_statistics: PipelineStatistics,
}

impl PipelineState{
    pub fn new(context: glow::Context) -> Self {
        unsafe {
            context.depth_func(CompareFunc::default() as u32);
        }

        Self {
            gl: context,
            blend: false,
            depth_test: false,
            depth_write: true,
            depth_func: Default::default(),
            color_write: Default::default(),
            stencil_test: false,
            cull_face: CullFace::Back,
            culling: false,
            stencil_mask: 0xFFFF_FFFF,
            clear_color: Color::from_rgba(0, 0, 0, 0),
            clear_stencil: 0,
            clear_depth: 1.0,
            scissor_test: false,
            framebuffer: None,
            blend_func: Default::default(),
            viewport: Rect::new(0, 0, 1, 1),
            program: Default::default(),
            //texture_units: [Default::default(); 32],
            //stencil_func: Default::default(),
            //stencil_op: Default::default(),
            vao: Default::default(),
            vbo: Default::default(),
            //frame_statistics: Default::default(),
        }
    }
    pub fn set_vertex_buffer_object(&mut self, vbo: Option<glow::Buffer>) {
        if self.vbo != vbo {
            self.vbo = vbo;

            //self.frame_statistics.vbo_binding_changes += 1;

            unsafe {
                self.gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo);
            }
        }
    }

    pub fn set_vertex_array_object(&mut self, vao: Option<glow::VertexArray>) {
        if self.vao != vao {
            self.vao = vao;

            //self.frame_statistics.vao_binding_changes += 1;

            unsafe {
                self.gl.bind_vertex_array(self.vao);
            }
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
#[repr(u32)]
pub enum CompareFunc {
    /// Never passes.
    Never = glow::NEVER,

    /// Passes if the incoming value is less than the stored value.
    Less = glow::LESS,

    /// Passes if the incoming value is equal to the stored value.
    Equal = glow::EQUAL,

    /// Passes if the incoming value is less than or equal to the stored value.
    LessOrEqual = glow::LEQUAL,

    /// Passes if the incoming value is greater than the stored value.
    Greater = glow::GREATER,

    /// Passes if the incoming value is not equal to the stored value.
    NotEqual = glow::NOTEQUAL,

    /// Passes if the incoming value is greater than or equal to the stored value.
    GreaterOrEqual = glow::GEQUAL,

    /// Always passes.
    Always = glow::ALWAYS,
}

impl Default for CompareFunc {
    fn default() -> Self {
        Self::LessOrEqual
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Debug )]
pub struct ColorMask {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl Default for ColorMask {
    fn default() -> Self {
        Self {
            red: true,
            green: true,
            blue: true,
            alpha: true,
        }
    }
}

impl ColorMask {
    pub fn all(value: bool) -> Self {
        Self {
            red: value,
            green: value,
            blue: value,
            alpha: value,
        }
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum CullFace {
    Back = glow::BACK,
    Front = glow::FRONT,
}

impl Default for CullFace {
    fn default() -> Self {
        Self::Back
    }
}

#[derive(Copy, Clone, Hash, PartialOrd, PartialEq, Eq, Ord,  Debug)]
#[repr(u32)]
pub enum BlendFactor {
    Zero = glow::ZERO,
    One = glow::ONE,
    SrcColor = glow::SRC_COLOR,
    OneMinusSrcColor = glow::ONE_MINUS_SRC_COLOR,
    DstColor = glow::DST_COLOR,
    OneMinusDstColor = glow::ONE_MINUS_DST_COLOR,
    SrcAlpha = glow::SRC_ALPHA,
    OneMinusSrcAlpha = glow::ONE_MINUS_SRC_ALPHA,
    DstAlpha = glow::DST_ALPHA,
    OneMinusDstAlpha = glow::ONE_MINUS_DST_ALPHA,
    ConstantColor = glow::CONSTANT_COLOR,
    OneMinusConstantColor = glow::ONE_MINUS_CONSTANT_COLOR,
    ConstantAlpha = glow::CONSTANT_ALPHA,
    OneMinusConstantAlpha = glow::ONE_MINUS_CONSTANT_ALPHA,
    SrcAlphaSaturate = glow::SRC_ALPHA_SATURATE,
    Src1Color = glow::SRC1_COLOR,
    OneMinusSrc1Color = glow::ONE_MINUS_SRC1_COLOR,
    Src1Alpha = glow::SRC1_ALPHA,
    OneMinusSrc1Alpha = glow::ONE_MINUS_SRC1_ALPHA,
}

impl Default for BlendFactor {
    fn default() -> Self {
        Self::Zero
    }
}

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash,  Debug)]
pub struct BlendFunc {
    pub sfactor: BlendFactor,
    pub dfactor: BlendFactor,
}

impl Default for BlendFunc {
    fn default() -> Self {
        Self {
            sfactor: BlendFactor::One,
            dfactor: BlendFactor::Zero,
        }
    }
}

