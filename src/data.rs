use bytemuck::{Pod, Zeroable};
use vulkano::impl_vertex;
use std::f64::consts::PI;

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

#[allow(dead_code)]
pub fn make_circle(corners: usize) -> Vec<Vertex>{
    let mut result: Vec<Vertex> = vec![];
    for i in 0..corners {
        result.push(
            Vertex{
                position: [(PI as f32 * 2.0 / (i as f32)).cos(), (PI as f32 * 2.0 / (i as f32)).sin()]
            }
        );
        result.push(
            Vertex{
                position: [(PI as f32 * 2.0 / ((i + 1) as f32)).cos(), (PI as f32 * 2.0 / ((i + 1) as f32)).sin()]
            }
        );
        result.push(Vertex{position: [0.0, 0.0]});
    };
    result
}