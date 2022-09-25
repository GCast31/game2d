/*================================================================
██████   █████  ███    ███ ███████ ██████  ██████         
██       ██   ██ ████  ████ ██           ██ ██   ██        
██   ███ ███████ ██ ████ ██ █████    █████  ██   ██        
██    ██ ██   ██ ██  ██  ██ ██      ██      ██   ██        
 ██████  ██   ██ ██      ██ ███████ ███████ ██████         
                                                           
                                                        
*******************************************************************/
use sdl2::image::LoadTexture;
use std::collections::HashMap;

use super::graphics::Drawable;

/*================================================================
 *                         _ I M A G E
 *================================================================*/
pub struct _Image {
    filename: String,
    width: u32,
    height: u32,
    pub(in crate::game2d) texture: sdl2::render::Texture,
}

impl Drawable for _Image {

    fn get_filename(&self) -> String {
        self.filename.clone()
    }
    fn get_quad(&self) -> Option<Quad> {
        Option::None
    }
    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}

/*
 * Image 
 */
pub struct Image {
    filename: String,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(filename: String, width: u32, height: u32) -> Image {
        Image { filename, width, height }
    }
}

impl Drawable for Image {

    fn get_filename(&self) -> String {
        self.filename.clone()
    }
    fn get_quad(&self) -> Option<Quad> {
        Option::None
    }
    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}

/*
 * Quad : A part of an image
 */
pub struct Quad {
    filename: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}


impl Quad {
    pub fn new(filename: String, x: i32, y: i32, width: u32, height: u32) -> Quad {
        Quad { filename, x, y, width, height }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
}

impl Drawable for Quad {

    fn get_filename(&self) -> String {
        self.filename.clone()
    }
    fn get_quad(&self) -> Option<Quad> {
        Some(Quad {
            filename: self.filename.clone(),
            height: self.height,
            width: self.width,
            x: self.x,
            y: self.y,
        })
    }
    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_height(&self) -> u32 {
        self.height
    }
}

//=======================================================================
//                            Images MANAGER
//=======================================================================
pub(in super) struct ImagesManager {
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    images: HashMap<String, _Image>,
}

#[allow(dead_code)]
impl ImagesManager {
    /*
     * new()
     * 
     * @Brief : Create a new ImagesManager
     */
    pub(crate) fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> ImagesManager {
        ImagesManager {
            texture_creator,
            images: HashMap::new(),
        }
    }

    /*
     * new_image()
     * 
     * @Brief : Try to load a new image in the images manager
     */
    pub(crate) fn new_image(
        &mut self,
        filename: &str,
    ) -> Result<Image, String> {
        if let Some(image) = self.images.get(&filename.to_string()) {
            return Ok(Image {
                filename: image.get_filename(),
                height: image.get_height(),
                width: image.get_width(),
            });
        }

        let texture_result = self.texture_creator.load_texture(filename);
        let texture: sdl2::render::Texture;
        match texture_result {
            Err(e) => {
                return Err(e);
            }
            Ok(t) => {
                texture = t;
            }
        }

        let height = texture.query().height;
        let width = texture.query().width;

        let image = _Image {
            filename: filename.to_string(),
            width,
            height,
            texture,
        };

        self.images.insert(filename.to_string(), image);

        Ok(Image { filename: filename.to_string(), width, height })
    }

    /*
     * get_image()
     * 
     * @Brief : Get a image from the image manager
     */
    pub fn get_image(&self, filename: &str) -> Option<&_Image> {
        self.images.get(&filename.to_string())
    }
}