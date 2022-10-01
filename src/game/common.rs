

pub const GAME_FONT_DEFAULT_: &'static str = "fonts/Vera.ttf";
pub const GAME_FONT_DEFAULT_SIZE: FontSize = 11;

// Primitives
pub type Angle = f64;

pub fn angle_add(angle_src: Angle, angle_to_add: Angle, cap: bool) -> Angle {
    let mut new_angle = angle_src + angle_to_add;
    if cap {
        if new_angle > 360. { 
            new_angle = 0.
        } else if new_angle < 0. {
            new_angle = 360.
        }
    }
    new_angle
}

pub type Fps = f32;
pub type Position = f32;
pub type Dimension = u32;
pub type Transformation = f32;
pub type Velocity = f32;
pub type DeltaTime = f32;
pub type ColorT = u8;
pub type FontSize = u16;
pub type Force = f32;

// 2D
#[derive(Clone, Copy)]
pub struct Velocity2d{pub x: Velocity, pub y: Velocity }

#[derive(Clone, Copy)]
pub struct Point2d{ pub x: Position, pub y: Position }

#[derive(Clone, Copy)]
pub struct Dimension2d{ pub h: Dimension, pub w: Dimension }

#[derive(Clone, Copy)]
pub struct Force2d{ pub fx: Force, pub fy: Force }

impl Force2d {
    pub fn new(angle: Angle, speed: Velocity2d) -> Self {
        let angle_radian = angle.to_radians();
        let fx: Velocity = (angle_radian.cos() * speed.x as Angle) as Velocity;
        let fy: Velocity = (angle_radian.sin() * speed.y as Angle) as Velocity;
        Self {
            fx,
            fy,
        }
    }
}


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
