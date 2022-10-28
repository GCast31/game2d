/*================================================================
██████   █████  ███    ███ ███████ ██████  ██████         
██       ██   ██ ████  ████ ██           ██ ██   ██        
██   ███ ███████ ██ ████ ██ █████    █████  ██   ██        
██    ██ ██   ██ ██  ██  ██ ██      ██      ██   ██        
 ██████  ██   ██ ██      ██ ███████ ███████ ██████         
                                                           
                                                        
*******************************************************************/
use sdl2::image::LoadTexture;
use std::collections::HashMap;

use crate::game::common::{Dimension, Position};


/*================================================================
 *                         _ I M A G E
 *================================================================*/
 pub trait ImageInformations {

    fn get_filename(&self) -> String;
    fn get_quad(&self) -> Option<Quad>;
    fn get_width(&self) -> Dimension;
    fn get_height(&self) -> Dimension;
}

 pub(crate) struct _Image {
    filename: String,
    width: Dimension,
    height: Dimension,
    pub(crate) texture: sdl2::render::Texture,
}

impl ImageInformations for _Image {

    fn get_filename(&self) -> String {
        self.filename.clone()
    }
    fn get_quad(&self) -> Option<Quad> {
        Option::None
    }
    fn get_width(&self) -> Dimension {
        self.width
    }

    fn get_height(&self) -> Dimension {
        self.height
    }
}

impl From<sdl2::render::Texture> for _Image{
    fn from(texture: sdl2::render::Texture) -> Self {
        let height = texture.query().height;
        let width = texture.query().width;
        Self {
            texture,
            height,
            width,
            filename: String::new(),
        } 
    }
}

impl _Image {
    pub(crate) fn from_texture(texture: sdl2::render::Texture) -> Self {
        let height = texture.query().height;
        let width = texture.query().width;
        Self {
            texture,
            height,
            width,
            filename: String::new(),
        }
    }
}

/*
 * Image 
 */
pub struct Image {
    filename: String,
    width: Dimension,
    height: Dimension,
}

impl Image {
    pub fn new(filename: String, width: Dimension, height: Dimension) -> Image {
        Image { filename, width, height }
    }
}

impl ImageInformations for Image {

    fn get_filename(&self) -> String {
        self.filename.clone()
    }
    fn get_quad(&self) -> Option<Quad> {
        Option::None
    }
    fn get_width(&self) -> Dimension {
        self.width
    }

    fn get_height(&self) -> Dimension {
        self.height
    }
}

/*
 * Quad : A part of an image
 */
pub struct Quad {
    filename: String,
    x: Position,
    y: Position,
    width: Dimension,
    height: Dimension,
}


impl Quad {
    pub fn new(filename: String, x: Position, y: Position, width: Dimension, height: Dimension) -> Quad {
        Quad { filename, x, y, width, height }
    }
    pub fn get_x(&self) -> Position {
        self.x
    }
    pub fn get_y(&self) -> Position {
        self.y
    }
}

impl ImageInformations for Quad {

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
    fn get_width(&self) -> Dimension {
        self.width
    }
    fn get_height(&self) -> Dimension {
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
            return Ok(Image { filename: image.filename.to_string(), width: image.width, height: image.height });
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
        
        Ok(Image {filename: filename.to_string(), height, width})
    
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