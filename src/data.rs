use bytemuck::{Pod, Zeroable};
use std::f64::consts::PI;
use vulkano::impl_vertex;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct Vertex {
    pub position: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
pub struct VertexColor {
    pub color: [f32; 4],
}

//struct object with position, size, rotation.

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

#[allow(dead_code)]
pub fn make_circle(corners: usize) -> Vec<Vertex> {
    let mut result: Vec<Vertex> = vec![];
    for i in 0..corners {
        result.push(Vertex {
            position: [
                (PI * 2.0 * ((i as f64) / corners as f64)).cos() as f32,
                (PI * 2.0 * ((i as f64) / corners as f64)).sin() as f32,
            ],
        });
        result.push(Vertex {
            position: [
                (PI * 2.0 * (((i + 1) as f64) / corners as f64)).cos() as f32,
                (PI * 2.0 * (((i + 1) as f64) / corners as f64)).sin() as f32,
            ],
        });
        result.push(Vertex {
            position: [0.0, 0.0],
        });
    }
    result
}
