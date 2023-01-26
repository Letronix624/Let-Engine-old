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
pub struct TextVertex {
    pub position: [f32; 2],
    pub tex_position: [f32; 2],
}

//struct object with position, size, rotation.

impl_vertex!(Vertex, position);
impl_vertex!(TextVertex, position, tex_position);

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
pub const BACKGROUND_ID: [u16; 12] = [0, 1, 2, 1, 3, 2, 4, 0, 5, 0, 2, 5];

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
pub const TRIANGLE_ID: [u16; 3] = [0, 1, 2];

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
pub const SQUARE_ID: [u16; 6] = [0, 1, 2, 1, 2, 3];

#[allow(dead_code)]
pub fn make_circle(corners: usize) -> Vec<Vertex> {
    let mut result: Vec<Vertex> = vec![];
    for i in 0..corners {
        result.push(Vertex {
            position: [0.0, 0.0],
        });
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
    }
    result
}

#[allow(dead_code)]
pub fn make_circle_id(corners: usize) -> Vec<u16> {
    let mut result: Vec<u16> = vec![];
    for i in 1..corners {
        result.push(0);
        result.push(i as u16);
        result.push(i as u16 + 1);
    }
    result.push(0);
    result.push(result.last().cloned().unwrap());
    result.push(result[1]);
    result
}
