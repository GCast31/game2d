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

use super::images::{ImagesManager, Quad, Image};
use super::color::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub struct Rectangle {
    x: Position,
    y: Position,
    width: Dimension,
    height: Dimension,
}

impl Rectangle {
    pub fn new(x: Position, y: Position, width: Dimension, height: Dimension) -> Rectangle {
        Rectangle { x, y, width, height }
    }
}

pub trait Draw {
    fn draw(&mut self, graphics: &mut Graphics);
}

pub trait Drawable {

    fn get_filename(&self) -> String;
    fn get_quad(&self) -> Option<Quad>;
    fn get_width(&self) -> Dimension;
    fn get_height(&self) -> Dimension;
}

#[allow(dead_code)]
pub enum DrawMode {
    Fill,
    Line,
}

pub struct Graphics {

    sdl_canvas: Canvas<Window>,
    pub(crate) sdl_event_pump: EventPump,
    images_manager: ImagesManager,

    actual_color: Color,
    default_color: Color,
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
        width: Dimension,
        height: Dimension,
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

        let mut canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let texture_creator = canvas.texture_creator();

        canvas.clear();
        canvas.present();

        Some(Graphics {

            sdl_canvas: canvas,
            sdl_event_pump: event_pump,
            images_manager: ImagesManager::new(texture_creator),

            actual_color: Color::BLACK,
            default_color: Color::BLACK,
        })
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
     * begin_draw()
     *
     * @brief : Prepare to drawing, call before drawing
     **********************************************************/
    pub fn begin_draw(&mut self) {

        self.set_color_to_default();
        self.sdl_canvas.clear();
    }

    /***********************************************************
     * end_draw()
     *
     * @brief : Call after drawing
     **********************************************************/
    pub fn end_draw(&mut self) {
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
                sdl2::rect::Point::new(x1, y1),
                sdl2::rect::Point::new(x2, y2),
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
    pub fn rectangle(&mut self, mode: DrawMode, x: Position, y: Position, width: Dimension, height: Dimension, color: Option<Color>) {

        let actual_color = self.actual_color;

        if let Some(color) = color {
            self.set_color(color);
        }

        match mode {
            DrawMode::Fill => {
                self.sdl_canvas
                    .fill_rect(sdl2::rect::Rect::new( x, y, width, height)) 
                    .unwrap();
        
            },
            DrawMode::Line => {
                self.sdl_canvas
                    .draw_rect(sdl2::rect::Rect::new( x, y, width, height)) 
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
    pub fn new_quad(&self, x: Position, y: Position, width: Dimension, height: Dimension, filename: String) -> Result<Quad, String> {
        
        // Image must be loaded in the images manager
        if let Some(image) = self.images_manager.get_image(&filename) {
            let image_w = image.get_width();
            let image_h = image.get_height();
            if x as Dimension + width > image_w || y as Dimension + height > image_h {
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
     * @brief : Draw image on screen
     *
     **********************************************************/
    pub fn draw(
        &mut self,
        drawable: &dyn Drawable, 
        x: Position, 
        y: Position, 
        angle: Angle,
        sx: Transformation,
        sy: Transformation,
        ox: Position,
        oy: Position,

    ) {
        let image = self.images_manager.get_image(drawable.get_filename().as_str());

        match image {
            Some(i) => {
                let mut dst = sdl2::rect::Rect::new(x, y, i.get_width(), i.get_height());

                let mut src: Option<sdl2::rect::Rect> = Option::None;

                if let Some(q) = drawable.get_quad() {
                    let rect = sdl2::rect::Rect::new(q.get_x(), q.get_y(), q.get_width(), q.get_height());
                    src = Some(rect);
                    dst.h = ((rect.h as Transformation) * sx) as i32;
                    dst.w = ((rect.w as Transformation) * sy) as i32;
                }

                let mut w_center = Option::None;
                if ox!=0 && oy!=0 {
                    w_center = Some(sdl2::rect::Point::new(ox, oy));
                }

                self.sdl_canvas
                    .copy_ex(
                        &i.texture, 
                        src, 
                        dst, 
                        angle, 
                        w_center, 
                        false,
                        false 
                    )
                    .unwrap();
            }
            None => {}
        }
    }
}


