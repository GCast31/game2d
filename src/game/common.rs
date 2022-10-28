use std::ops::Add;


pub const GAME_FONT_DEFAULT_: &'static str = "fonts/Vera.ttf";
pub const GAME_FONT_DEFAULT_SIZE: FontSize = 11;

pub trait Sizable {
    fn get_size(&mut self) -> &Size2d;
    fn get_mut_size(&mut self) -> &mut Size2d;
}
pub trait Movable {
    fn mov(&mut self);
}
pub trait Scalable {
    fn get_scale(&mut self) -> &Scale2d;
    fn get_mut_scale(&mut self) -> &mut Scale2d;
}
pub trait Positionable {
    fn get_position(&mut self) -> &Position2d;
    fn get_mut_position(&mut self) -> &mut Position2d;
}

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
pub type Size = u32;
pub type Transformation = f32;
pub type Velocity = f32;
pub type DeltaTime = f32;
pub type ColorT = u8;
pub type FontSize = u16;
pub type Force = f32;

// 2D
#[derive(Clone, Copy)]
pub struct Velocity2d{pub vx: Velocity, pub vy: Velocity }

#[derive(Clone, Copy)]
pub struct Point2d{ pub x: Position, pub y: Position }

#[derive(Clone, Copy)]
pub struct Position2d{ pub x: Position, pub y: Position }

#[derive(Clone, Copy)]
pub struct Size2d{ pub h: Size, pub w: Size }

#[derive(Clone, Copy)]
pub struct Force2d{ pub fx: Force, pub fy: Force }

impl Force2d {
    pub fn new(angle: Angle, speed: Velocity2d) -> Self {
        let angle_radian = angle.to_radians();
        let fx: Velocity = (angle_radian.cos() * speed.vx as Angle) as Velocity;
        let fy: Velocity = (angle_radian.sin() * speed.vy as Angle) as Velocity;
        Self {
            fx,
            fy,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Scale2d{ pub sx: Transformation, pub sy: Transformation }


impl Add<Velocity2d> for Position2d {

    type Output = Position2d;

    fn add(self, rhs: Velocity2d) -> Self::Output {
        Self {
            x: self.x + rhs.vx,
            y: self.y + rhs.vy,
        }
    }
}  
