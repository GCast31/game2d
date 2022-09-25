

// Primitives
pub type Angle = f64;
pub type Fps = f32;
pub type Position = f32;
pub type Dimension = u32;
pub type Transformation = f32;
pub type Velocity = f32;
pub type DeltaTime = f32;

// 2D
pub struct Velocity2d{pub x: Velocity, pub y: Velocity }
pub struct Point2d{ pub x: Position, pub y: Position }
pub struct Dimension2d{ pub h: Dimension, pub w: Dimension }

pub struct Scale2d{ pub sx: Transformation, pub sy: Transformation }

impl Point2d {
    pub fn add_velocity(position: Position, velocity: &Velocity) -> Position {
        (position as Velocity + velocity) as Position
    }

    pub fn add_velocity2d(position: &Point2d, velocity: &Velocity2d) -> Point2d {
        Point2d {
            x: (position.x as Velocity + velocity.x) as Position,
            y: (position.y as Velocity + velocity.y) as Position
        }
    }
}
