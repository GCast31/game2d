

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

