/*
  ████████ ██ ████████ ██      ███████ 
     ██    ██    ██    ██      ██      
     ██    ██    ██    ██      █████   
     ██    ██    ██    ██      ██      
     ██    ██    ██    ███████ ███████ 
 
    
    @Author : GCast31
*/


mod game2d;

use game2d::game::game::*;
use game2d::graphics::graphics::{Graphics, DrawMode};

use crate::game2d::inputs::keyboard::Keyboard;

use game2d::graphics::color::*;
use game2d::inputs::keyboard::Keys;


// ################################################################################################################
// #                                      C O N S T R A N T E S  FOR  G A M E                                     #
// ################################################################################################################
pub const GAME_WINDOW_HEIGHT: u32 = 600;
pub const GAME_WINDOW_WIDTH: u32 = 800;

// ################################################################################################################
// #                                        S T R U C T U R E    M Y G A M E                                      #
// ################################################################################################################
pub struct MyGame {
}

#[allow(dead_code)]
impl Default for MyGame {
    fn default() -> Self {
        MyGame {
        }
    }
}

// ################################################################################################################
// #                                                   M A I N                                                    #
// ################################################################################################################
fn main() {

    // Game
    Game::new(
              Graphics::new(
                "Title", 
                GAME_WINDOW_WIDTH, 
                GAME_WINDOW_HEIGHT, 
                false
            )
        .unwrap())

        .set_params(MyGame::default())
        .set_max_fps(Some(144.))
        .set_callback_draw(draw)
        .set_callback_load(load)
        .set_callback_key_pressed(keypressed)
        .set_callback_update(update)
        .set_callback_quit(quit)
        .run();

}

// ################################################################################################################
// #                                                    L O A D                                                   #
// ################################################################################################################
#[allow(unused_variables)]
pub fn load(graphics: &mut Graphics, game: &mut Option<MyGame>) {

}

// ################################################################################################################
// #                                                   U P D A T E                                                #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn update(graphics: &mut Graphics, game: &mut Option<MyGame>, keyboard: &mut Keyboard, dt: f32) {
 
}


// ################################################################################################################
// #                                               K E Y P R E S S E D                                            #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn keypressed(graphics: &mut Graphics, game: &mut Option<MyGame>, key: &Keys) {
    
}

// ################################################################################################################
// #                                                    D R A W                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn draw(graphics: &mut Graphics, game: &mut Option<MyGame>) {
  
    graphics.rectangle(DrawMode::Fill, 0, 0, GAME_WINDOW_WIDTH, 32, Some(Color::GREEN));
    graphics.rectangle(DrawMode::Fill, 0, GAME_WINDOW_HEIGHT as i32 - 32, GAME_WINDOW_WIDTH, 32, Some(Color::RED));

}

// ################################################################################################################
// #                                                    Q U I T                                                   #
// ################################################################################################################ 
#[allow(unused_variables)]
pub fn quit(graphics: &mut Graphics, game: &mut Option<MyGame>) {
    println!("Bye");
}