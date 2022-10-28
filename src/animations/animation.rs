
use std::time::SystemTime;

use crate::graphics::images::ImageInformations;

pub struct Animation {
    actual_sprite: usize,
    images: Vec<Box<dyn ImageInformations>>,
    delay_between_two_images_in_ms: Option<u128>,
    last_change: Option<SystemTime>,
}

#[allow(dead_code)]
impl Animation {
    pub fn new() -> Animation {

        Animation {
            actual_sprite: 0,
            images: Vec::new(),
            delay_between_two_images_in_ms: Option::None,
            last_change: Option::None,
        }
    }

    pub fn add<T: ImageInformations + 'static>(&mut self, image: Box::<T>) {
        self.images.push(image);
    }

    pub fn set_timer(&mut self, timer_in_ms: u128) {
        self.delay_between_two_images_in_ms = Some(timer_in_ms);
    }

    pub fn remove_timer(&mut self) {
        self.delay_between_two_images_in_ms = Option::None;
    }

    pub fn run(&mut self) -> Option<&Box<dyn ImageInformations>> {

        let mut must_change = true;

        /* If we have set a delay between two sprites, we check than
           delay is passed
         */
        if let Some(d) = self.delay_between_two_images_in_ms {
            if let Some(l) = self.last_change {
                match l.elapsed() {
                    Ok(elapsed) => {
                        if elapsed.as_millis() < d {
                            must_change = false;
                        }
                    },
                    Err(_) => {
                        must_change = false;
                    }
                }
            }
        }

        /* In each change, we begin again*/
        if must_change {
          self.actual_sprite += 1;
          self.last_change = Some(SystemTime::now());
        }

        if self.actual_sprite > self.images.len() {
            self.actual_sprite = 1;
        }
        if self.actual_sprite > self.images.len() {
            return Option::None;
        }
        return Some(&self.images[self.actual_sprite - 1]);
    }

    pub fn restart(&mut self) {
        self.actual_sprite = 0;
        self.last_change = Option::None;
    }
}