use position::Position;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: Position,
    pub texture_coordinate: Position
}