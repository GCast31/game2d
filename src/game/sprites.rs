use std::any::{Any, TypeId}; 
use std::collections::HashMap;

use crate::graphics::graphics::{Drawable};
use super::game::Updatable;
use super::common::{WithSize, WithPosition, Movable};


pub trait SpriteTrait: 
    Any +Drawable + Updatable + WithSize + WithPosition + Movable {}

pub struct Sprites {
    datas: HashMap<TypeId, Vec<Box<dyn Any>>>,
}

impl Sprites {
    pub fn new() -> Self {
        Self { datas: HashMap::new() }
    }

    pub fn add(&mut self, sprite: impl SpriteTrait + 'static) {
        let typeid = sprite.type_id();
        if !self.datas.contains_key(&typeid) {
            self.datas.insert(typeid, Vec::new());
        }
        if let Some(lst) = self.datas.get_mut(&typeid) {
            lst.push(Box::new(sprite));
        }
    }

    pub fn get<T: SpriteTrait + 'static>(&self, entity: T) -> Option<&Vec<Box<dyn Any>>> {
        let typeid = entity.type_id();
        self.datas.get(&typeid)
    }

    pub fn get_mut<T: SpriteTrait + 'static>(&mut self, entity: T) -> Option<&mut Vec<Box<dyn Any>>> {
        let typeid = entity.type_id();
        self.datas.get_mut(&typeid)
    }

    pub fn get_all(&self) -> &HashMap<TypeId, Vec<Box<dyn Any>>> {
        &self.datas
    }

    pub fn get_all_mut(&mut self) -> &mut HashMap<TypeId, Vec<Box<dyn Any>>> {
        &mut self.datas
    }
}