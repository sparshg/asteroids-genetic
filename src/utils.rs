use macroquad::prelude::*;

pub fn rotate_vec(vec: Vec2, angle: f32) -> Vec2 {
    vec2(angle.cos(), angle.sin()).rotate(vec)
}

pub fn draw_polygon(x: f32, y: f32, points: Vec<Vec2>, color: Color) {
    let points_length = points.len();
    let mut vertices = Vec::with_capacity(points_length as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(points_length as usize * 3);

    for (i, point) in points.iter().enumerate() {
        let vertex = macroquad::models::Vertex {
            position: Vec3::new(x + point.x, y + point.y, 0.0),
            uv: Vec2::default(),
            color,
        };

        vertices.push(vertex);
        indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
    }

    let mesh = Mesh {
        vertices,
        indices,
        texture: None,
    };

    draw_mesh(&mesh);
}
