

// Primitives
pub type Angle = f64;
pub type Fps = f32;
pub type Position = f32;
pub type Dimension = u32;
pub type Transformation = f32;
pub type Velocity = f32;
pub type DeltaTime = f32;
pub type ColorT = u8;
pub type FontSize = u16;

// 2D
#[derive(Clone, Copy)]
pub struct Velocity2d{pub x: Velocity, pub y: Velocity }

#[derive(Clone, Copy)]
pub struct Point2d{ pub x: Position, pub y: Position }

#[derive(Clone, Copy)]
pub struct Dimension2d{ pub h: Dimension, pub w: Dimension }

#[derive(Clone, Copy)]
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
