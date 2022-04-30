use std::{
    hash::{Hash, Hasher},
};

/// See module docs.
#[derive(Clone, Default, Debug)]
pub struct VertexBuffer {
    dense_layout: Vec<VertexAttribute>,
    sparse_layout: [Option<VertexAttribute>; 13],
    vertex_size: u8,
    vertex_count: u32,
    data: Vec<u8>,
    data_hash: u64,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct VertexAttribute {
    /// Claimed usage of the attribute. It could be Position, Normal, etc.
    pub usage: VertexAttributeUsage,
    /// Data type of every component of the attribute. It could be F32, U32, U16, etc.
    pub data_type: VertexAttributeDataType,
    /// Size of attribute expressed in components. For example, for `Position` it could
    /// be 3 - which means there are 3 components in attribute of `data_type`.
    pub size: u8,
    /// Sets a "fetch rate" for vertex shader at which it will read vertex attribute:
    ///  0 - per vertex (default)
    ///  1 - per instance
    ///  2 - per 2 instances and so on.
    pub divisor: u8,
    /// Offset in bytes from beginning of the vertex.
    pub offset: u8,
    /// Defines location of the attribute in a shader (`layout(location = x) attrib;`)
    pub shader_location: u8,
}


/// An usage for vertex attribute. It is a fixed set, but there are plenty
/// room for any custom data - it may be fit into `TexCoordN` attributes.
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
#[repr(u32)]
pub enum VertexAttributeUsage {
    /// Vertex position. Usually Vector2<f32> or Vector3<f32>.
    Position = 0,
    /// Vertex normal. Usually Vector3<f32>, more rare Vector3<u16> (F16).
    Normal = 1,
    /// Vertex tangent. Usually Vector3<f32>.
    Tangent = 2,
    /// First texture coordinates. Usually Vector2<f32>.
    /// It may be used for everything else, not only for texture coordinates.
    TexCoord0 = 3,
    /// Second texture coordinates.
    TexCoord1 = 4,
    /// Third texture coordinates.
    TexCoord2 = 5,
    /// Fourth texture coordinates.
    TexCoord3 = 6,
    /// Fifth texture coordinates.
    TexCoord4 = 7,
    /// Sixth texture coordinates.
    TexCoord5 = 8,
    /// Seventh texture coordinates.
    TexCoord6 = 9,
    /// Eighth texture coordinates.
    TexCoord7 = 10,
    /// Bone weights. Usually Vector4<f32>.
    BoneWeight = 11,
    /// Bone indices. Usually Vector4<u8>.
    BoneIndices = 12,
    /// Maximum amount of attribute kinds.
    Count,
}

impl Default for VertexAttributeUsage {
    fn default() -> Self {
        Self::Position
    }
}


/// Data type for a vertex attribute component.
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq, Ord, Hash, Debug)]
#[repr(u8)]
pub enum VertexAttributeDataType {
    /// 32-bit floating-point.
    F32,
    /// 32-bit unsigned integer.
    U32,
    /// 16-bit unsigned integer.
    U16,
    /// 8-bit unsigned integer.
    U8,
}

impl Default for VertexAttributeDataType {
    fn default() -> Self {
        Self::F32
    }
}

impl VertexAttributeDataType {
    /// Returns size of data in bytes.
    pub fn size(self) -> u8 {
        match self {
            VertexAttributeDataType::F32 | VertexAttributeDataType::U32 => 4,
            VertexAttributeDataType::U16 => 2,
            VertexAttributeDataType::U8 => 1,
        }
    }
}


///// A buffer for data that defines connections between vertices.
//#[derive(Default, Clone, Debug)]
//pub struct TriangleBuffer {
//    triangles: Vec<TriangleDefinition>,
//    data_hash: u64,
//}
