use crate::game::common::Position;



pub struct Mouse {
    x: Position,
    y: Position,
}

impl Default for Mouse {
    fn default() -> Self {
        Mouse { 
            x: 0.,
            y: 0. 
        }
    }
}

impl Mouse {
    pub(crate) fn set_x(&mut self, x: Position) {
        self.x = x;
    }

    pub fn get_x(&mut self) -> Position {
        self.x
    }

    pub(crate) fn set_y(&mut self, y: Position) {
        self.y = y;
    }

    pub fn get_y(&mut self) -> Position {
        self.y
    }
}