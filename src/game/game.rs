
use std::time::{Instant, Duration};

use crate::graphics::fonts::FontsManager;
use crate::graphics::graphics::Graphics;
use crate::inputs::keyboard::{Keys, Keyboard};
use crate::inputs::mouse::Mouse;
use sdl2::event::Event;

use super::common::{Fps, DeltaTime, Position};
use super::inputs::Inputs;

#[allow(dead_code)]
pub type GameCallbackDraw<T>        = fn(&mut Graphics, &mut Option<T>, &mut Inputs, &mut Option<FontsManager>);
pub type GameCallbackKeyPressed<T>  = fn(&mut Graphics, &mut Option<T>, &Keys);
pub type GameCallbackLoad<T>        = fn(&mut Graphics, &mut Option<T>);
pub type GameCallbackQuit<T>        = fn(&mut Graphics, &mut Option<T>);
pub type GameCallbackUpdate<T>      = fn(&mut Graphics, &mut Option<T>, &mut Inputs, DeltaTime);


#[allow(dead_code)]
pub struct Game<T> { 

    graphics: Graphics,

    max_fps: Option<f32>,

    params: Option<T>,

    callback_keypressed : Option<GameCallbackKeyPressed<T>>,
    callback_update : Option<GameCallbackUpdate<T>>,
    callback_draw   : Option<GameCallbackDraw<T>>,
    callback_quit   : Option<GameCallbackQuit<T>>,
    callback_load   : Option<GameCallbackLoad<T>>,

}

#[allow(dead_code)]
impl<T> Game<T> {
    /*
     * new()
     * 
     * @Brief : Create a new game 
     */
    pub fn new(graphics: Graphics) -> Game<T> {

        Game {
            graphics,
            max_fps: Option::None,
            params: Option::None, 
            callback_update: Option::None, 
            callback_draw: Option::None, 
            callback_quit: Option::None, 
            callback_load: Option::None,
            callback_keypressed: Option::None,
        }
    }

    /*
     * set_params()
     * 
     * @Brief : Set params to send to callbacks
     */
    pub fn set_params(&mut self, params: T) -> &mut Self {
        self.params = Some(params);
        self
    }

    /*
     * set_max_fps()
     * 
     * @Brief : Set max FPS 
     */
    pub fn set_max_fps(&mut self, max_fps: Option<Fps>) -> &mut Self {
        self.max_fps = max_fps;
        self
    }

    /*
     * set_callback_draw()
     * 
     * @Brief : Callback to call in each draw
     */
    pub fn set_callback_draw(&mut self, callback: GameCallbackDraw<T>) -> &mut Self {
        self.callback_draw = Some(callback);
        self
    }

    /*
     * set_callback_load()
     * 
     * @Brief : Callback to call in load
     */
    pub fn set_callback_load(&mut self, callback: GameCallbackLoad<T>) -> &mut Self {
        self.callback_load = Some(callback);
        self
    }

    /*
     * set_callback_update()
     * 
     * @Brief : Callback to call in each update
     */
    pub fn set_callback_update(&mut self, callback: GameCallbackUpdate<T>) -> &mut Self {
        self.callback_update = Some(callback);
        self
    }

    /*
     * set_callback_quit()
     * 
     * @Brief : Callback to call before quit
     */
    pub fn set_callback_quit(&mut self, callback: GameCallbackQuit<T>) -> &mut Self {
        self.callback_quit = Some(callback);
        self
    }

     /*
     * set_callback_pressed()
     * 
     * @Brief : Callback to call a each key pressed (UP and DOWN)
     */
    pub fn set_callback_key_pressed(&mut self, callback: GameCallbackKeyPressed<T>) -> &mut Self {
        self.callback_keypressed = Some(callback);
        self
    }

    /*
     * run()
     * 
     * @Brief : Main loop of the game
     */
    pub fn run(&mut self, fonts_manager: &mut Option<FontsManager>) -> &mut Self {

        let mut inputs: Inputs = Inputs {
            keyboard: Keyboard::default(),
            mouse: Mouse::default(),
        };

        // Load
        if let Some(l) = self.callback_load {
             l(&mut self.graphics, &mut self.params);
        }

        let mut timer_start = Instant::now();

        // Main loop
        'mainloop: loop {
            // Before drawing
            self.graphics.begin_draw();

            // Keys 
            for event in self.graphics.sdl_event_pump.poll_iter() {
                match event {
                    Event::KeyDown { timestamp: _, window_id: _, keycode, scancode: _, keymod: _, repeat: _ } => {
                        inputs.keyboard.add_key_down(Keyboard::_sdl_keycode_to_key(keycode.unwrap()));
                    },
                    Event::KeyUp { timestamp: _, window_id: _, keycode, scancode: _, keymod: _, repeat: _ } => {
                        inputs.keyboard.add_key_up(Keyboard::_sdl_keycode_to_key(keycode.unwrap()));
                    },
                    Event::MouseMotion { timestamp: _, window_id: __id, which: _, mousestate: _, x, y, xrel: _, yrel: _ } => {
                        inputs.mouse.set_x(x as Position);
                        inputs.mouse.set_y(y as Position);
                    },
                    Event::Quit { .. } => {
                        if let Some(q) = &mut self.callback_quit {
                            q(&mut self.graphics, &mut self.params);
                        }
                        break 'mainloop;
                    }
                    _ => {
                    }
                }
            }

            // Keys released callback ?
            if let Some(k) = self.callback_keypressed {
                let keys = inputs.keyboard.get_keys_pressed();
                for key in keys.iter() {
                    k(&mut self.graphics,  &mut self.params, key);
                }
            }

            // Update callback ?
            let mut dt =  timer_start.elapsed().as_secs_f32();
            timer_start = Instant::now();
            if let Some(u) = self.callback_update {
               
                // Limit FPS
                if let Some(max_fps) = self.max_fps {
                    let limit_fps = 1. / max_fps;
                    if dt > limit_fps {
                        dt = limit_fps;
                    }
                }

                u(&mut self.graphics, &mut self.params, &mut inputs, dt);
            }

            // Draw callback ?
            if let Some(d) = self.callback_draw {
                d(&mut self.graphics, &mut self.params, &mut inputs, fonts_manager);
            }

            // After drawing
            self.graphics.end_draw();

            // Limit FPS
            if let Some(fps) = self.max_fps {
                if fps > 0. {
                    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps as u32));
                }
            }

        }
        self
    }
}