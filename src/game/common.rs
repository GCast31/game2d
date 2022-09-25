

// Primitives
pub type Angle = f64;
pub type Fps = f32;
pub type Position = i32;
pub type Dimension = u32;
pub type Transformation = f32;
pub type Velocity = f32;
pub type DeltaTime = f32;

// 2D
pub struct Velocity2d(pub Velocity, pub Velocity);
pub struct Point2d(pub Position, pub Position);
pub struct Dimension2d(pub Dimension, pub Dimension);

pub struct Scale2d(pub Transformation, pub Transformation);

impl Point2d {
    pub fn add_velocity(position: Position, velocity: &Velocity) -> Position {
        (position as Velocity + velocity) as Position
    }

    pub fn add_velocity2d(position: Point2d, velocity: &Velocity2d) -> Point2d {
        Point2d(
            (position.0 as Velocity + velocity.0) as Position,
            (position.1 as Velocity + velocity.1) as Position
        )
    }
}
