use crate::game::common::ColorT;




#[derive(Copy, Clone)]
pub struct Color {
    pub r: ColorT,
    pub g: ColorT,
    pub b: ColorT,
    pub a: ColorT,
}

#[allow(dead_code)]
impl Color {

    pub(crate) fn to_sdl_color(&self) -> sdl2::pixels::Color {
        sdl2::pixels::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };

    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    pub const BLACK: Color = Color {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
}