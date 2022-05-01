use crate::core::algebra::{Vector2, Vector3, Vector4};
use std::hash::{Hash, Hasher};

use super::buffer::{
    VertexAttributeDataType, VertexAttributeDescriptor, VertexAttributeUsage,
};

/// A vertex for static meshes.
#[derive(Copy, Clone, Debug, Default)]
#[repr(C)] // OpenGL expects this structure packed as in C
pub struct StaticVertex {
    /// Position of vertex in local coordinates.
    pub position: Vector3<f32>,
    /// Texture coordinates.
    pub tex_coord: Vector2<f32>,
    /// Normal in local coordinates.
    pub normal: Vector3<f32>,
    /// Tangent vector in local coordinates.
    pub tangent: Vector4<f32>,
}


impl StaticVertex {
    /// Creates new vertex from given position and texture coordinates.
    pub fn from_pos_uv(position: Vector3<f32>, tex_coord: Vector2<f32>) -> Self {
        Self {
            position,
            tex_coord,
            normal: Vector3::new(0.0, 1.0, 0.0),
            tangent: Vector4::default(),
        }
    }

    /// Creates new vertex from given position and texture coordinates.
    pub fn from_pos_uv_normal(
        position: Vector3<f32>,
        tex_coord: Vector2<f32>,
        normal: Vector3<f32>,
    ) -> Self {
        Self {
            position,
            tex_coord,
            normal,
            tangent: Vector4::default(),
        }
    }

    /// Returns layout of the vertex.
    pub fn layout() -> &'static [VertexAttributeDescriptor] {
        static LAYOUT: [VertexAttributeDescriptor; 4] = [
            VertexAttributeDescriptor {
                usage: VertexAttributeUsage::Position,
                data_type: VertexAttributeDataType::F32,
                size: 3,
                divisor: 0,
                shader_location: 0,
            },
            VertexAttributeDescriptor {
                usage: VertexAttributeUsage::TexCoord0,
                data_type: VertexAttributeDataType::F32,
                size: 2,
                divisor: 0,
                shader_location: 1,
            },
            VertexAttributeDescriptor {
                usage: VertexAttributeUsage::Normal,
                data_type: VertexAttributeDataType::F32,
                size: 3,
                divisor: 0,
                shader_location: 2,
            },
            VertexAttributeDescriptor {
                usage: VertexAttributeUsage::Tangent,
                data_type: VertexAttributeDataType::F32,
                size: 4,
                divisor: 0,
                shader_location: 3,
            },
        ];
        &LAYOUT
    }
}


impl PartialEq for StaticVertex {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.tex_coord == other.tex_coord
            && self.normal == other.normal
            && self.tangent == other.tangent
    }
}

// This is safe because Vertex is tightly packed struct with C representation
// there is no padding bytes which may contain garbage data. This is strictly
// required because vertices will be directly passed on GPU.
impl Hash for StaticVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        #[allow(unsafe_code)]
        unsafe {
            let bytes = self as *const Self as *const u8;
            state.write(std::slice::from_raw_parts(
                bytes,
                std::mem::size_of::<Self>(),
            ))
        }
    }
}
