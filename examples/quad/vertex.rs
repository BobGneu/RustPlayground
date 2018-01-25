#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Position,
    pub texture_coordinate: Position
}

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32
}