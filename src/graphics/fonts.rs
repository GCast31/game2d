/*================================================================
██████   █████  ███    ███ ███████ ██████  ██████         
██       ██   ██ ████  ████ ██           ██ ██   ██        
██   ███ ███████ ██ ████ ██ █████    █████  ██   ██        
██    ██ ██   ██ ██  ██  ██ ██      ██      ██   ██        
 ██████  ██   ██ ██      ██ ███████ ███████ ██████         
                                                           
                                                        
*******************************************************************/
use std::{collections::HashMap, marker::PhantomData};

use crate::game::common::FontSize;

/*================================================================
 *                         _ F O N T
 *================================================================*/

 #[derive(PartialEq, Eq, Hash)]
 pub struct Font<'a, T: 'a> {
    filename: String,
    size: FontSize,
    phantom: PhantomData<&'a T>
}

pub struct _Font<'a> {
    sdl_font: sdl2::ttf::Font<'a, 'a>,
}

//=======================================================================
//                            Font MANAGER
//=======================================================================
pub(in super) struct FontsManager<'a, T> {
    ttf_context: sdl2::ttf::Sdl2TtfContext,
    texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    fonts: Box<HashMap<Font<'a, T>, _Font<'a>>,
}

#[allow(dead_code)]
impl FontsManager<'_, bool> {
    /*
     * new()
     * 
     * @Brief : Create a new ImagesManager
     */
    pub(crate) fn new(texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>, ttf_context: sdl2::ttf::Sdl2TtfContext) -> FontsManager<'static, bool> {
        FontsManager {
            texture_creator,
            ttf_context,
            fonts: HashMap::new(),
        }
    }

    /*
     * new_font()
     * 
     * @Brief : Try to load a new font in the font manager
     */
    pub(crate) fn new_font<'a>(
        &mut self,
        filename: &str,
        size: FontSize,
    ) -> Result<(), String> {

        let font = Font {
            filename: filename.to_string(),
            size,
            phantom: PhantomData,
        };

        if let Some(_) = self.fonts.get(&font) {
            return Ok(());
        }

        if let Ok(sdl_font) =  self.ttf_context.load_font(filename.to_string(), size) {
            let f = _Font {
                sdl_font,
            };
            self.fonts.insert(font, f);
        }

        Ok(())
    }

    /*
     * get_font()
     * 
     * @Brief : Get a font from the image manager
     */
    pub(super) fn get_font(&mut self, filename: &str, size: FontSize) -> Option<&_Font> {
       let Font = Font {
            filename: filename.to_string(),
            size,
            phantom: PhantomData,
        };
        self.fonts.get(&Font)
    }
}