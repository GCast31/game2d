use std::collections::HashMap;

use sdl2::keyboard::Keycode;

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Keys {
    Space,
    Up,
    Down,
    Left,
    Right,
}

pub struct Keyboard {
    keys_released: HashMap<u32, bool>,
    keys_up: HashMap<u32, bool>,
    keys_down: HashMap<u32, bool>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard { 
            keys_released: HashMap::new(),
            keys_up: HashMap::new(),
            keys_down: HashMap::new(), 
        }
    }
}

impl Keyboard {
    /*
     * reset()
     * 
     * @Brief : Reset Keyboard state
     */
    pub fn reset(&mut self) {
        self.keys_up.clear();
        self.keys_down.clear();
        self.keys_released.clear();
    }

    /*
     * add_key_down()
     * 
     * @Brief : Add a new key down
     */
    pub fn add_key_down(&mut self, key: Option<Keys>) {

        if let Some(key) = key {
            let x = key as u32;

            self.keys_down.insert(x, true);
            if self.keys_released.contains_key(&x) == false {
                self.keys_released.insert(x, true);
            }
        }
    }

    /*
     * add_key_up()
     * 
     * @Brief : Add a new key UP
     */
    pub fn add_key_up(&mut self, key: Option<Keys>) {

        if let Some(key) = key {
            let x = key as u32;

            self.keys_up.insert(x, true);
            self.keys_released.remove(&x).unwrap();

            // A Key can't be up and down in same time
            self.keys_down.remove_entry(&x);
        }
    }

    /*
     * is_down()
     * 
     * @Brief : Check if a key is down
     */
    pub fn is_down(&self, key: &Keys) -> bool {
        let x: u32 = *key as u32;
        self.keys_down.contains_key(&x)
    }

    /*
     * is_up()
     * 
     * @Brief : Check if a key is up
     */
    pub fn is_up(&self, key: &Keys) -> bool {
        let x: u32 = *key as u32;
        self.keys_up.contains_key(&x)
    }

    /*
     * get_keys_pressed()
     * 
     * @Brief : Get all keys pressed
     */
    pub fn get_keys_pressed(&mut self) -> Vec<Keys> {
        let mut vec = Vec::new();

        for (k, v) in self.keys_released.iter_mut() {
            if *v {
                let key = match *k {
                    0 => Some(Keys::Space),
                    1 => Some(Keys::Up),
                    2 => Some(Keys::Down),
                    3 => Some(Keys::Left),
                    4 => Some(Keys::Right),
                    _ => Some(Keys::Down),
                };
                if let Some(k) = key {   
                    vec.push(k);
                }
                *v = false;
            }
        }

        vec
    }

    /*
     * _sdl_keycode_to_key() 
     * 
     * @Brief : Bind SDL KeyCode to Game2D Key
     */
    pub(crate) fn _sdl_keycode_to_key(keycode: Keycode) -> Option<Keys> {
        match keycode {
            Keycode::Left   => { Some(Keys::Left) },
            Keycode::Right  => { Some(Keys::Right) },
            Keycode::Up     => { Some(Keys::Up) },
            Keycode::Down   => { Some(Keys::Down)},
            _               => Option::None,
        }
    }
}