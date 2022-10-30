use std::ops::Add;


pub const GAME_FONT_DEFAULT_: &'static str = "fonts/Vera.ttf";
pub const GAME_FONT_DEFAULT_SIZE: FontSize = 11;

pub trait WithSize {
    fn get_size(&self) -> &Size2d;
}

pub trait Sizable {
    fn set_size(&mut self, size: Size2d);
}
pub trait Movable : Positionable {
    fn get_velocity(&self) -> &Velocity2d;
    fn get_mut_velocity(&mut self) -> &mut Velocity2d;
    fn set_velocity(&mut self, _velocity: Velocity2d);
    fn set_vx(&mut self, vx: Velocity);
    fn set_vy(&mut self, vy: Velocity);
}

pub trait WithScale {
    fn get_scale(&self) -> &Scale2d;
}

pub trait Scalable {
    fn set_scale(&mut self, _scale: Scale2d);
    fn set_sx(&mut self, sx: Transformation);
    fn set_sy(&mut self, sy: Transformation);
}

pub trait WithPosition {
    fn get_position(&self) -> &Position2d;
}

pub trait Positionable {
    fn set_position(&mut self, _position: Position2d);
    fn set_x(&mut self, x: Position);
    fn set_y(&mut self, y: Position);
}

pub trait Standing {
    fn get_standing(&self) -> bool { false }
    fn set_standing(&mut self, _standing: bool) {}
}

pub trait Falling {
    fn get_falling(&self) -> bool { false }
    fn set_falling(&mut self, _falling: bool) {} 
}

pub trait Jumping {
    fn get_jumping(&self) -> bool { false }
    fn set_jumping(&mut self, _jumping: bool) {}
}

pub trait Flying {
    fn get_flying(&self) -> bool { false }
    fn set_flying(&mut self, _jumping: bool) {}
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
