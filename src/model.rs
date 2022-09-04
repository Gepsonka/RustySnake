
use rand::prelude::*;

/// Trait for game elements.
/// Every element can move on the canvas, so
/// every element has to obtain the ability to update its position
/// both in the model and in the view
trait Update {
    pub fn update(&mut self, x: u8, y:u8);
    pub fn update_rand(&mut self) -> Self {}
}


enum MapState {
    SnakeHead,
    SnakeBody,
    Food,
    Empty
}

enum SnakeDirection{
    Up,
    Down,
    Left,
    Right
}


struct GameModel {
    map: Map,
    snake: Snake,
    food: Food,

}


#[derive(Debug, Clone)]
struct Map {
    map: [[MapState; 100]; 100],
}


impl Default for Map {
    fn default() -> Map {
        Map { map : [[Empty; 100]; 100] }
    }
}

impl Map {
    pub fn new() -> Map {
        Map::default() // Use the default method when creating new Map
    }

}


/// Snake representation on the model
struct Snake {
    snake_head: SnakeBody,
    snake_body: Vec<SnakeBody>,
    snake_direction: SnakeDirection,

}

struct SnakeBody {
    x: u8,
    y: u8
}

impl SnakeBody {
    pub fn new(x: u8, y: u8) -> SnakeBody{
        SnakeBody{
            x,
            y,
        }
    }
}

impl Update for SnakeBody {
    fn update(&mut self, x: u8, y: u8){
        self.x = x;
        self.y = y;
    }
}



struct Food {
    x: u8,
    y: u8
}


impl Food {
    pub fn new(x: u8, y: u8) -> Food {
        Food {
            x,
            y,
        }
    }

    pub fn update(&mut self, x: u8, y:u8){
        self.x = x;
        self.y = y;
    }

    pub fn update_rand(&mut self){
        self.x = (rng.gen() * 100) as u8;
        self.y = (rng.gen() * 100) as u8;
    }
}

impl Default for Food {
    fn default() -> Food {
        Food {
            20,
            20,
        }
    }
}
