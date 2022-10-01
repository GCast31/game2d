/*
  ████████ ██ ████████ ██      ███████ 
     ██    ██    ██    ██      ██      
     ██    ██    ██    ██      █████   
     ██    ██    ██    ██      ██      
     ██    ██    ██    ███████ ███████ 
 
    
    @Author : GCast31
*/

use game2d::game::common::{GAME_FONT_DEFAULT_, GAME_FONT_DEFAULT_SIZE, DeltaTime};
use game2d::game::game::*;
use game2d::graphics::fonts::FontsManager;
use game2d::graphics::graphics::Graphics;

use game2d::inputs::keyboard::Keyboard;
use game2d::inputs::keyboard::Keys;


// ################################################################################################################
// #                                      C O N S T R A N T E S  FOR  G A M E                                     #
// ################################################################################################################
pub const GAME_WINDOW_HEIGHT: u32 = 600;
pub const GAME_WINDOW_WIDTH: u32 = 800;


// ################################################################################################################
// #                                        S T R U C T U R E    G A M E                                          #
// ################################################################################################################
pub struct TemplateGame {
}

#[allow(dead_code)]
impl Default for TemplateGame {
    fn default() -> Self {
        TemplateGame {
            
        }
    }
}

// ################################################################################################################
// #                                                   M A I N                                                    #
// ################################################################################################################
fn main() {

    let mut graphics = Graphics::new(
        "Title", 
        GAME_WINDOW_WIDTH, 
        GAME_WINDOW_HEIGHT, 
        false
    ).unwrap();

    // Fonts
    let mut font_context = Graphics::create_fonts_context();
    let mut fonts_manager: FontsManager = FontsManager::new(graphics.get_fonts_creator());
    let font_detail = fonts_manager.load_font(&mut font_context, GAME_FONT_DEFAULT_.to_string(), GAME_FONT_DEFAULT_SIZE).unwrap();
    graphics.set_font(font_detail);

    // Game
    Game::new(graphics)
        .set_params(TemplateGame::default())
        .set_max_fps(Some(144.))
        .set_callback_draw(draw)
        .set_callback_load(load)
        .set_callback_key_pressed(keypressed)
        .set_callback_update(update)
        .set_callback_quit(quit)
        .run(&mut Some(fonts_manager));

}

// ################################################################################################################
// #                                                    L O A D                                                   #
// ################################################################################################################
#[allow(unused_variables)]
pub fn load(graphics: &mut Graphics, game: &mut Option<TemplateGame>) {
}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<TemplateGame>, keyboard: &mut Keyboard, dt: DeltaTime) {
 
}


// ################################################################################################################
// #                                               K E Y P R E S S E D                                            #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn keypressed(graphics: &mut Graphics, game: &mut Option<TemplateGame>, key: &Keys) {
    
}

// ################################################################################################################
// #                                                    D R A W                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn draw(graphics: &mut Graphics, game: &mut Option<TemplateGame>, fonts_manager: &mut Option<FontsManager>) {
  
}

// ################################################################################################################
// #                                                    Q U I T                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn quit(graphics: &mut Graphics, game: &mut Option<TemplateGame>) {
    println!("Bye");
}