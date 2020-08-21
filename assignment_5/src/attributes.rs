use glam::Vec4;

#[derive(Copy, Clone)]
pub struct VertexAttributes {
    pub position: Vec4,
}

impl VertexAttributes {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { position: Vec4::new(x, y, z, 1.0) }
    }

    pub fn interpolate(a: VertexAttributes, b: VertexAttributes,
        c: VertexAttributes, alpha: f32, beta: f32, gamma: f32) -> Self {
            let mut r = VertexAttributes::new(0.0, 0.0, 0.0);
            r.position = alpha * a.position + beta * b.position + gamma * c.position;
            r
        }
}

#[derive(Copy, Clone)]
pub struct FragmentAttributes {
    pub color: Vec4,
}

impl FragmentAttributes {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { color: Vec4::new(r, g, b, 1.0) }
    }
}

#[derive(Copy, Clone, Default)]
pub struct UniformAttributes;