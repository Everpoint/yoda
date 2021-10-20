use glow::{Buffer, Context, HasContext, VertexArray};

pub enum AttributeValueType {
    Boolean,
    Integer,
    UnsignedInteger,
    Float,
    Double,
}

impl AttributeValueType {
    pub fn glow_type(&self) -> u32 {
        match self {
            Self::Boolean => glow::BOOL,
            Self::Integer => glow::INT,
            Self::UnsignedInteger => glow::UNSIGNED_INT,
            Self::Float => glow::FLOAT,
            Self::Double => glow::DOUBLE,
        }
    }

    pub fn size(&self) -> i32 {
        match self {
            AttributeValueType::Boolean => 1,
            AttributeValueType::Integer => 4,
            AttributeValueType::UnsignedInteger => 4,
            AttributeValueType::Float => 4,
            AttributeValueType::Double => 8,
        }
    }

    pub fn is_int(&self) -> bool {
        match self {
            AttributeValueType::Boolean => true,
            AttributeValueType::Integer => true,
            AttributeValueType::UnsignedInteger => true,
            AttributeValueType::Float => false,
            AttributeValueType::Double => false,
        }
    }
}

pub struct VertexAttribute {
    pub location: u32,
    pub size: i32,
    pub value_type: AttributeValueType,
}

pub trait Vertex {
    fn attributes() -> Vec<VertexAttribute>;
}

pub struct GlBuffer {
    pub vertex_array: VertexArray,
    pub vertex_buffer: Buffer,
    pub index_buffer: Option<Buffer>,
    pub vertex_count: u32,
}

impl GlBuffer {
    pub fn create<V: Vertex>(gl: &Context, vertices: &[V], indices: Option<&[u32]>) -> Self {
        unsafe {
            let vertex_array = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vertex_array));

            let vertex_size = std::mem::size_of::<V>();
            let vertex_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                to_bytes(&vertices[0], vertex_size * vertices.len()),
                glow::STATIC_DRAW,
            );

            let mut offset = 0;
            for attrib in V::attributes() {
                if attrib.value_type.is_int() {
                    gl.vertex_attrib_pointer_i32(
                        attrib.location,
                        attrib.size,
                        attrib.value_type.glow_type(),
                        vertex_size as i32,
                        offset,
                    );
                } else {
                    gl.vertex_attrib_pointer_f32(
                        attrib.location,
                        attrib.size,
                        attrib.value_type.glow_type(),
                        false,
                        vertex_size as i32,
                        offset,
                    );
                }
                gl.enable_vertex_attrib_array(attrib.location);

                offset += attrib.size as i32 * attrib.value_type.size();
            }

            let vertex_buffer = vertex_buffer;
            let mut vertex_count = vertices.len() as u32;

            let index_buffer = indices.map(|indices| {
                let index_buffer = gl.create_buffer().unwrap();
                gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(index_buffer));
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    to_bytes(&indices[0], 4 * indices.len()),
                    glow::STATIC_DRAW,
                );

                vertex_count = indices.len() as u32;
                index_buffer
            });

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            Self {
                vertex_array,
                vertex_buffer,
                index_buffer,
                vertex_count,
            }
        }
    }
}

unsafe fn to_bytes<T>(p: &T, size: usize) -> &[u8] {
    std::slice::from_raw_parts((p as *const T) as *const u8, size)
}
