use std::any::{Any, TypeId}; 
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::graphics::graphics::{Drawable, Graphics};
use super::game::Updatable;
use super::inputs::Inputs;
use super::common::{DeltaTime, WithSize, WithPosition, Movable};


pub trait SpriteTrait: 
    Any + Drawable + Updatable + WithSize + WithPosition + Movable {}

pub struct Sprites {
    datas: HashMap<TypeId, Vec<Rc<RefCell<dyn SpriteTrait>>>>,
}

impl Sprites {
    pub fn new() -> Self {
        Self { datas: HashMap::new() }
    }

    pub fn add(&mut self, sprite: impl SpriteTrait) {
        let typeid = sprite.type_id();
        if !self.datas.contains_key(&typeid) {
            self.datas.insert(typeid, Vec::new());
        }
        if let Some(lst) = self.datas.get_mut(&typeid) {
            lst.push(Rc::new(RefCell::new(sprite)));
        }
    }

    pub fn get<T: SpriteTrait>(&self, entity: T) -> Option<&Vec<Rc<RefCell<dyn SpriteTrait>>>> {
        let typeid = entity.type_id();
        self.datas.get(&typeid)
    }

    pub fn get_mut<T: SpriteTrait>(&mut self, entity: T) -> Option<&mut Vec<Rc<RefCell<dyn SpriteTrait>>>> {
        let typeid = entity.type_id();
        self.datas.get_mut(&typeid)
    }

    pub fn get_all(&self) -> &HashMap<TypeId, Vec<Rc<RefCell<dyn SpriteTrait>>>> {
        &self.datas
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<TypeId, Vec<Rc<RefCell<dyn SpriteTrait>>>> {
        &mut self.datas
    }

    pub fn draw(&mut self, graphics: &mut Graphics) {
        for typeid in self.datas.iter_mut() {
            for entity in typeid.1.iter_mut() {
                entity.borrow_mut().draw(graphics);
            }
        }
    }

    pub fn update(&mut self, graphics: &mut Graphics, inputs: &mut Inputs, dt: &DeltaTime) {
        for typeid in self.datas.iter_mut() {
            for entity in typeid.1.iter_mut() {
                entity.borrow_mut().update(graphics, inputs, &dt);
            }
        }
    }
}