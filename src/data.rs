use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 2],
}

impl_vertex!(Vertex, position);

#[allow(dead_code)]
pub const BACKGROUND: [Vertex; 12] = [
    Vertex {
        position: [-1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 0.0],
    },
    Vertex {
        position: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];
#[allow(dead_code)]
pub const TRIANGLE: [Vertex; 3] = [
    Vertex {
        position: [0.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
];
#[allow(dead_code)]
pub const SQUARE: [Vertex; 6] = [
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
];
