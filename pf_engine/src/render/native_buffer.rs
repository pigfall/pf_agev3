use std::{ marker::PhantomData, mem::size_of};
use super::{
    state::PipelineState,
    error::FrameworkError,
};
use glow::HasContext;

use crate::systems::surface::buffer::{VertexAttributeDataType, VertexBuffer};

pub struct NativeBuffer {
    pub state: *mut PipelineState,
    pub id: glow::Buffer,
    pub kind: GeometryBufferKind,
    pub element_size: usize,
    pub size_bytes: usize,
    // Force compiler to not implement Send and Sync, because OpenGL is not thread-safe.
    thread_mark: PhantomData<*const u8>,
}

impl Drop for NativeBuffer {
    fn drop(&mut self) {
        unsafe {
            (*self.state).gl.delete_buffer(self.id);
        }
    }
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum GeometryBufferKind {
    StaticDraw = glow::STATIC_DRAW,
    DynamicDraw = glow::DYNAMIC_DRAW,
}

pub struct BufferBuilder {
    pub element_size: usize,
    pub kind: GeometryBufferKind,
    pub attributes: Vec<AttributeDefinition>,
    pub data: *const u8,
    pub data_size: usize,
}

impl BufferBuilder {
    pub fn new<T: Sized>(kind: GeometryBufferKind, data: Option<&[T]>) -> Self {
        let (data, data_size) = if let Some(data) = data {
            (data as *const _ as *const u8, data.len() * size_of::<T>())
        } else {
            (std::ptr::null(), 0)
        };

        Self {
            kind,
            attributes: Default::default(),
            element_size: size_of::<T>(),
            data,
            data_size,
        }
    }

    pub fn from_vertex_buffer(buffer: &VertexBuffer, kind: GeometryBufferKind) -> Self {
        Self {
            element_size: buffer.vertex_size() as usize,
            kind,
            attributes: buffer
                .layout()
                .iter()
                .map(|a| AttributeDefinition {
                    location: a.shader_location as u32,
                    kind: match (a.data_type, a.size) {
                        (VertexAttributeDataType::F32, 1) => AttributeKind::Float,
                        (VertexAttributeDataType::F32, 2) => AttributeKind::Float2,
                        (VertexAttributeDataType::F32, 3) => AttributeKind::Float3,
                        (VertexAttributeDataType::F32, 4) => AttributeKind::Float4,
                        (VertexAttributeDataType::U32, 1) => AttributeKind::UnsignedInt,
                        (VertexAttributeDataType::U32, 2) => AttributeKind::UnsignedInt2,
                        (VertexAttributeDataType::U32, 3) => AttributeKind::UnsignedInt3,
                        (VertexAttributeDataType::U32, 4) => AttributeKind::UnsignedInt4,
                        (VertexAttributeDataType::U16, 1) => AttributeKind::UnsignedShort,
                        (VertexAttributeDataType::U16, 2) => AttributeKind::UnsignedShort2,
                        (VertexAttributeDataType::U16, 3) => AttributeKind::UnsignedShort3,
                        (VertexAttributeDataType::U16, 4) => AttributeKind::UnsignedShort4,
                        (VertexAttributeDataType::U8, 1) => AttributeKind::UnsignedByte,
                        (VertexAttributeDataType::U8, 2) => AttributeKind::UnsignedByte2,
                        (VertexAttributeDataType::U8, 3) => AttributeKind::UnsignedByte3,
                        (VertexAttributeDataType::U8, 4) => AttributeKind::UnsignedByte4,
                        _ => unreachable!(),
                    },
                    normalized: false,
                    divisor: 0,
                })
                .collect(),
            data: buffer.raw_data().as_ptr(),
            data_size: buffer.raw_data().len(),
        }
    }

    pub fn with_attribute(mut self, attribute: AttributeDefinition) -> Self {
        self.attributes.push(attribute);
        self
    }

    pub fn build(self, state: &mut PipelineState) -> Result<NativeBuffer, FrameworkError> {
        let vbo = unsafe { state.gl.create_buffer()? };

        state.set_vertex_buffer_object(Some(vbo));

        if self.data_size > 0 {
            unsafe {
                state.gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    std::slice::from_raw_parts(self.data, self.data_size),
                    self.kind as u32,
                );
            }
        }

        let native_buffer = NativeBuffer {
            state,
            id: vbo,
            kind: self.kind,
            element_size: self.element_size,
            size_bytes: self.data_size,
            thread_mark: Default::default(),
        };

        let mut offset = 0usize;
        for definition in self.attributes {
            unsafe {
                state.gl.vertex_attrib_pointer_f32(
                    definition.location,
                    definition.kind.length() as i32,
                    definition.kind.get_type(),
                    definition.normalized,
                    self.element_size as i32,
                    offset as i32,
                );
                state
                    .gl
                    .vertex_attrib_divisor(definition.location, definition.divisor);
                state.gl.enable_vertex_attrib_array(definition.location);

                offset += definition.kind.size_bytes();

                if offset > self.element_size {
                    state.set_vertex_buffer_object(Default::default());
                    return Err(FrameworkError::InvalidAttributeDescriptor);
                }
            }
        }

        Ok(native_buffer)
    }
}

pub struct AttributeDefinition {
    pub location: u32,
    pub kind: AttributeKind,
    pub normalized: bool,
    pub divisor: u32,
}


#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum AttributeKind {
    Float,
    Float2,
    Float3,
    Float4,

    UnsignedByte,
    UnsignedByte2,
    UnsignedByte3,
    UnsignedByte4,

    UnsignedShort,
    UnsignedShort2,
    UnsignedShort3,
    UnsignedShort4,

    UnsignedInt,
    UnsignedInt2,
    UnsignedInt3,
    UnsignedInt4,
}

impl AttributeKind {
    pub fn size_bytes(self) -> usize {
        match self {
            AttributeKind::Float => size_of::<f32>(),
            AttributeKind::Float2 => size_of::<f32>() * 2,
            AttributeKind::Float3 => size_of::<f32>() * 3,
            AttributeKind::Float4 => size_of::<f32>() * 4,

            AttributeKind::UnsignedByte => size_of::<u8>(),
            AttributeKind::UnsignedByte2 => size_of::<u8>() * 2,
            AttributeKind::UnsignedByte3 => size_of::<u8>() * 3,
            AttributeKind::UnsignedByte4 => size_of::<u8>() * 4,

            AttributeKind::UnsignedShort => size_of::<u16>(),
            AttributeKind::UnsignedShort2 => size_of::<u16>() * 2,
            AttributeKind::UnsignedShort3 => size_of::<u16>() * 3,
            AttributeKind::UnsignedShort4 => size_of::<u16>() * 4,

            AttributeKind::UnsignedInt => size_of::<u32>(),
            AttributeKind::UnsignedInt2 => size_of::<u32>() * 2,
            AttributeKind::UnsignedInt3 => size_of::<u32>() * 3,
            AttributeKind::UnsignedInt4 => size_of::<u32>() * 4,
        }
    }

    fn get_type(self) -> u32 {
        match self {
            AttributeKind::Float
            | AttributeKind::Float2
            | AttributeKind::Float3
            | AttributeKind::Float4 => glow::FLOAT,

            AttributeKind::UnsignedByte
            | AttributeKind::UnsignedByte2
            | AttributeKind::UnsignedByte3
            | AttributeKind::UnsignedByte4 => glow::UNSIGNED_BYTE,

            AttributeKind::UnsignedShort
            | AttributeKind::UnsignedShort2
            | AttributeKind::UnsignedShort3
            | AttributeKind::UnsignedShort4 => glow::UNSIGNED_SHORT,

            AttributeKind::UnsignedInt
            | AttributeKind::UnsignedInt2
            | AttributeKind::UnsignedInt3
            | AttributeKind::UnsignedInt4 => glow::UNSIGNED_INT,
        }
    }

    fn length(self) -> usize {
        match self {
            AttributeKind::Float
            | AttributeKind::UnsignedByte
            | AttributeKind::UnsignedShort
            | AttributeKind::UnsignedInt => 1,

            AttributeKind::Float2
            | AttributeKind::UnsignedByte2
            | AttributeKind::UnsignedShort2
            | AttributeKind::UnsignedInt2 => 2,

            AttributeKind::Float3
            | AttributeKind::UnsignedByte3
            | AttributeKind::UnsignedShort3
            | AttributeKind::UnsignedInt3 => 3,

            AttributeKind::Float4
            | AttributeKind::UnsignedByte4
            | AttributeKind::UnsignedShort4
            | AttributeKind::UnsignedInt4 => 4,
        }
    }
}
