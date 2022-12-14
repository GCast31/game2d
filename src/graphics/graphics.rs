/*
██████   █████  ███    ███ ███████ ██████  ██████
██       ██   ██ ████  ████ ██           ██ ██   ██
██   ███ ███████ ██ ████ ██ █████    █████  ██   ██
██    ██ ██   ██ ██  ██  ██ ██      ██      ██   ██
 ██████  ██   ██ ██      ██ ███████ ███████ ██████


 ██████  ██████   █████  ██████  ██   ██ ██  ██████ ███████
██       ██   ██ ██   ██ ██   ██ ██   ██ ██ ██      ██
██   ███ ██████  ███████ ██████  ███████ ██ ██      ███████
██    ██ ██   ██ ██   ██ ██      ██   ██ ██ ██           ██
 ██████  ██   ██ ██   ██ ██      ██   ██ ██  ██████ ███████

 */

use crate::game::common::*;

use super::fonts::{FontsManager, FontDetail, FontContext};
use super::images::{ImagesManager, Quad, Image, _Image, ImageInformations, ImageFromString};
use super::color::Color;
use sdl2::render::{Canvas, BlendMode, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::EventPump;

pub type FontsCreator = TextureCreator<WindowContext>;

#[allow(dead_code)]
pub struct Rectangle {
    x: Position,
    y: Position,
    width: Size,
    height: Size,
}

impl Rectangle {
    pub fn new(x: Position, y: Position, width: Size, height: Size) -> Rectangle {
        Rectangle { x, y, width, height }
    }
}

pub trait Drawable {
    fn draw(&mut self, graphics: &mut Graphics);
}

#[allow(dead_code)]
pub enum DrawMode {
    Fill,
    Line,
}

pub struct Graphics {

    //===== SDL2
    sdl_canvas: Canvas<Window>,
    pub(crate) sdl_event_pump: EventPump,

    //==== Images
    images_manager: ImagesManager,

    //==== Color
    actual_color: Color,
    default_color: Color,
    background_color: Color,
    font_color: Color,

    //==== Fonts
    actual_font: Option<FontDetail>,

    //==== Scale
    actuel_scale: Scale2d,
}

#[allow(dead_code)]
impl Graphics {
    //=======================================================================
    //                               GENERAL
    //=======================================================================

    /***********************************************************
     * new()
     *
     * @brief : Create an instance of Graphics
     *
     * @return : Instance of Graphics
     **********************************************************/
    pub fn new(
        title: &str,
        width: Size,
        height: Size,
        fullscreen: bool,
    ) -> Option<Graphics> {

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();


        /* Create the window */
        let mut window = video_subsystem
            .window(title, width, height)
            .position_centered()
            .build()
            .unwrap();

        // Full screen ?
        if fullscreen {
            window
                .set_fullscreen(sdl2::video::FullscreenType::True)
                .unwrap();
        }

        // Textures / Primitives ...
        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_blend_mode(BlendMode::Blend); // Accept Alpha

        let images_manager = ImagesManager::new(canvas.texture_creator());

        // Events
        let event_pump = sdl_context.event_pump().unwrap();

        // Clear screen and show
        canvas.clear();
        canvas.present();

        // Create Graphics
        Some(Graphics {
            sdl_canvas: canvas,
            images_manager,

            sdl_event_pump: event_pump,

            actual_color: Color::BLACK,
            default_color: Color::BLACK,
            background_color: Color::RED,
            font_color: Color::WHITE,

            actual_font: Option::None,
            actuel_scale: Scale2d { sx: 1., sy: 1. }
            
        })
    }

    /***********************************************************
     * set_scale()
     *
     * @Brief : Set scale
     */
    pub fn set_scale(&mut self, scale: Scale2d) {
        self.actuel_scale = scale;
    }

    /***********************************************************
     * get_fonts_creator
     *
     * @Brief : Create a texture for fonts
     */
    pub fn get_fonts_creator(&mut self) -> FontsCreator {
        self.sdl_canvas.texture_creator() as FontsCreator
    }

    /***********************************************************
     * create_fonts_context()
     * 
     */
    pub fn create_fonts_context() -> FontContext<'static> {
        sdl2::ttf::init().unwrap() as FontContext
    }

    /***********************************************************
     * set_default_color()
     *
     * @Brief : Set the default color
     *
     * @parm 1 : Color information
     */
    pub fn set_default_color(&mut self, p_default_color: Color) {
        self.default_color = p_default_color;
    }

    /***********************************************************
     * set_color()
     *
     * @Brief : Set actual color
     *
     * @parm 1 : Color information
     */
    pub fn set_color(&mut self, p_color: Color) {
        self.actual_color = p_color;
        self.sdl_canvas.set_draw_color(self.actual_color.to_sdl_color());
    }

    /***********************************************************
     * set_color_to_default()
     *
     * @Brief : Set actual color with default color
     *
     * @parm 1 : Color information
     */
    pub fn set_color_to_default(&mut self) {
        self.actual_color = self.default_color;
        self.sdl_canvas.set_draw_color(self.default_color.to_sdl_color());
    }

    /***********************************************************
     * set_background_color()
     *
     * @Brief : Set actual background
     *
     * @parm 1 : Color information
     */
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    /***********************************************************
     * set_font_color()
     *
     * @Brief : Set actual font color
     *
     * @parm 1 : Color information
     */
    pub fn set_font_color(&mut self, color: Color) {
        self.font_color = color;
    }

    /***********************************************************
     * set_font()
     *
     * @Brief : Set font
     */
    pub fn set_font(&mut self, font_detail: FontDetail) {
        self.actual_font = Some(font_detail);
    }

    /***********************************************************
     * begin_draw()
     *
     * @brief : Prepare to drawing, call before drawing
     **********************************************************/
    pub(crate) fn begin_draw(&mut self) {

        self.set_color(self.background_color);
        self.sdl_canvas.clear();
        self.set_color_to_default();

    }

    /***********************************************************
     * end_draw()
     *
     * @brief : Call after drawing
     **********************************************************/
    pub(crate) fn end_draw(&mut self) {
        self.sdl_canvas.present();
    }

    //=======================================================================
    //                             PRIMITIVES
    //=======================================================================
    /***********************************************************
     * draw_line()
     *
     * @brief : Draw a line
     */
    pub fn line(&mut self, x1: Position, y1: Position, x2: Position, y2: Position, color: Option<Color>) {

        let actual_color = self.actual_color;

        if let Some(color) = color {
            self.set_color(color);
        }

        self.sdl_canvas
            .draw_line(
                sdl2::rect::Point::new(x1 as i32, y1 as i32),
                sdl2::rect::Point::new(x2 as i32, y2 as i32),
            )
            .unwrap();
        
        if let Some(_) = color {
            self.set_color(actual_color);
        }
    }

    /***********************************************************
     * rectangle()
     *
     * @brief : Draw a rectangle
     */
    pub fn rectangle(&mut self, mode: DrawMode, x: Position, y: Position, width: Size, height: Size, color: Option<Color>) {

        let actual_color = self.actual_color;

        if let Some(color) = color {
            self.set_color(color);
        }

        match mode {
            DrawMode::Fill => {
                self.sdl_canvas
                    .fill_rect(sdl2::rect::Rect::new( x as i32, y as i32, width, height)) 
                    .unwrap();
        
            },
            DrawMode::Line => {
                self.sdl_canvas
                    .draw_rect(sdl2::rect::Rect::new( x as i32, y as i32, width, height)) 
                    .unwrap();
            },
        }

        if let Some(_) = color {
            self.set_color(actual_color);
        }
    }

    //=======================================================================
    //                         IMAGES (TEXTURES)
    //=======================================================================
    /***********************************************************
     * new_image()
     *
     * @brief : Add a image
     **********************************************************/
    pub fn new_image(
        &mut self,
        filename: &str,
    ) -> Result<Image, String> {
        self.images_manager.new_image(filename)
    }

    /***********************************************************
     * new_quad()
     *
     * @brief : Create a new quad from an image
     **********************************************************/
    pub fn new_quad(&self, x: Position, y: Position, width: Size, height: Size, filename: String) -> Result<Quad, String> {
        
        // Image must be loaded in the images manager
        if let Some(image) = self.images_manager.get_image(&filename) {
            let image_w = image.get_width();
            let image_h = image.get_height();
            if x as Size + width > image_w || y as Size + height > image_h {
                return Err(format!("Image {} must contain quad", filename));
            }
            else {
                return Ok(Quad::new(filename, x, y, width, height))
            }
        }
        else {
            return Err(format!("Image {} not loaded", filename));
        }

        
    }

    /***********************************************************
     * draw()
     *
     * @brief : Draw image on screen (simple)
     *
     **********************************************************/
    pub fn draw(
        &mut self,
        drawable: &dyn ImageFromString, 
        x: Position, 
        y: Position, 
        angle: Angle,

    ) {
        self.draw_full(drawable, x, y, angle, 1., 1., 0., 0.);
    }

    /***********************************************************
     * draw_full()
     *
     * @brief : Draw image on screen (full (more options))
     *
     **********************************************************/
    pub fn draw_full(
        &mut self,
        drawable: &dyn ImageFromString, 
        x: Position, 
        y: Position, 
        angle: Angle,
        sx: Transformation,
        sy: Transformation,
        ox: Position,
        oy: Position,

    ) {
        let image = self.images_manager.get_image(drawable.get_filename().as_str());

        let mut scalex = sx * self.actuel_scale.sx;
        let mut scaley = sy * self.actuel_scale.sy;

        match image {
            Some(i) => {
                let mut dst = sdl2::rect::Rect::new((x * self.actuel_scale.sx)as i32,(y * self.actuel_scale.sy) as i32, i.get_width(), i.get_height());
                dst.h = ((dst.h as Transformation) * scalex) as i32;
                dst.w = ((dst.w as Transformation) * scaley) as i32;

                let mut src: Option<sdl2::rect::Rect> = Option::None;

                if let Some(q) = drawable.get_quad() {
                    let rect = sdl2::rect::Rect::new((q.get_x() * self.actuel_scale.sx) as i32, (q.get_y() * self.actuel_scale.sy) as i32 , q.get_width(), q.get_height());
                    src = Some(rect);
                    dst.h = ((rect.h as Transformation) * scalex) as i32;
                    dst.w = ((rect.w as Transformation) * scaley) as i32;
                }

                let mut w_center = Option::None;
                if ox!=0. && oy!=0. {
                    w_center = Some(sdl2::rect::Point::new(ox as i32, oy as i32));
                }

                let flip_h = 
                    if scalex < 0. {
                        scalex *= -1.;
                        true
                    } 
                    else { 
                        false 
                    };

                let flip_v = 
                    if scaley < 0. {
                        scaley *= -1.;
                        true
                    } 
                    else { 
                        false 
                    };

                self.sdl_canvas
                    .copy_ex(
                        &i.texture, 
                        src, 
                        dst, 
                        angle, 
                        w_center, 
                        flip_h,
                        flip_v, 
                    )
                    .unwrap();
            }
            None => {}
        }
    }

    fn _draw_image(
        &mut self,
        _image: &_Image, 
        x: Position, 
        y: Position, 
        angle: Angle,
        sx: Transformation,
        sy: Transformation,
        ox: Position,
        oy: Position,

    ) {

        let mut scalex = sx ;
        let mut scaley = sy ;

        let mut dst = sdl2::rect::Rect::new(x as i32 , y  as i32, _image.get_width(), _image.get_height());
        dst.h = ((dst.h as Transformation) * scalex) as i32;
        dst.w = ((dst.w as Transformation) * scaley) as i32;

        let mut src: Option<sdl2::rect::Rect> = Option::None;

        if let Some(q) = _image.get_quad() {
            let rect = sdl2::rect::Rect::new(q.get_x() as i32 , q.get_y() as i32 , q.get_width(), q.get_height());
            src = Some(rect);
            dst.h = ((rect.h as Transformation) * scalex) as i32;
            dst.w = ((rect.w as Transformation) * scaley) as i32;
        }

        let mut w_center = Option::None;
        if ox!=0. && oy!=0. {
            w_center = Some(sdl2::rect::Point::new(ox as i32, oy as i32));
        }

        let flip_h = 
                    if scalex < 0. {
                        scalex *= -1.;
                        true
                    } 
                    else { 
                        false 
                    };

        let flip_v = 
            if scaley < 0. {
                scaley *= -1.;
                true
            } 
            else { 
                false 
            };

        match  self.sdl_canvas
            .copy_ex(
                &_image.texture, 
                src, 
                dst, 
                angle, 
                w_center, 
                flip_h,
                flip_v, 
            ) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }

    }

    //=======================================================================
    //                                 FONTS
    //=======================================================================
    /***********************************************************
     * new_font()
     *
     * @brief : Add a new font
     **********************************************************/
    //  pub fn new_font(
    //     &'ttf mut self,
    //     filename: &str,
    //     point_size: u16,
    // ) -> Result<FontDetail, String> {
    //     self.fonts_manager.load_font(filename, point_size)
    // }

    /***********************************************************
     * print()
     *
     * @brief : Add a new font
     **********************************************************/
     pub fn print(
        &mut self,
        fonts_manager: &mut FontsManager,
        texte: String,
        x: Position, 
        y: Position, 
        color: Option<Color>,
     ) {

        self.print_full(fonts_manager, texte, x, y, color, 0., 1., 1., 0., 0.)
    }
    
    /***********************************************************
     * print_full()
     *
     * @brief : Add a new font
     **********************************************************/
     pub fn print_full(
        &mut self,
        fonts_manager: &mut FontsManager,
        texte: String,
        x: Position, 
        y: Position, 
        color: Option<Color>,
        angle: Angle,
        sx: Transformation,
        sy: Transformation,
        ox: Position,
        oy: Position,
     ) {

        let mut local_color = self.font_color.clone();
        if let Some(color) = color {
            local_color = color.clone();
        }

        if let Some(font_detail) = &mut self.actual_font {
            if let Some(texture) = fonts_manager.draw_font(&font_detail, texte, &local_color) {
                let image = _Image::from_texture(texture);
                self._draw_image(&image, x, y, angle, sx, sy, ox, oy);
            }
        }
    }
}