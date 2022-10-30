
use std::collections::HashMap;
use crate::{animations::animation::Animation, graphics::images::ImageFromString};

pub struct AnimationsManager {
    current: Option<String>,
    animations: HashMap<String, Animation>,
}

impl AnimationsManager {
    pub fn new() -> AnimationsManager {
        AnimationsManager {
            current: Option::None,
            animations: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, animation : Animation) -> Result<bool, String> {

        if self.animations.contains_key(&name) {
            return Err(format!("Animation {} already exist", name));
        }
        self.animations.insert(name, animation);

        Ok(true)
    }

    pub fn set_current(&mut self, name: String) -> Result<bool, String> {
        if !self.animations.contains_key(&name) {
            self.current = Option::None;
            return Err(format!("Animation {} not exist", name));
        } 
        match &self.current {
            Some(x) => {
                if *x != name {
                    self.current = Some(name);
                    self.restart_current();
                }
            },
            None => {
                self.current = Some(name);
                self.restart_current();
            }
        }

        Ok(true)
    }

    fn get_current(&mut self) -> Option<&mut Animation> {
        if let Some(current) = &self.current {
            return self.animations.get_mut(current)
        }
        Option::None
    }

    pub fn run_current(&mut self) -> Option<&Box<dyn ImageFromString>> {
        let current = self.get_current();
        match current {
            Some(animation) => {
                animation.run()
            },
            None => {
                Option::None
            }
        }
    }

    pub fn restart_current(&mut self) {
        let current = self.get_current();
        match current {
            Some(animation) => {
                animation.restart()
            },
            None => {
            }
        }
    }

}
