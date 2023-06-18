use bevy::{prelude::Image, utils::HashMap};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GridType {
    Passable,
    Unpassable,
    Breakable,
}

pub struct GridMap {
    map: HashMap<(u8, u8), GridType>,
    max_x: u8,
    max_y: u8,
}

impl GridMap {
    pub fn new() -> Self {
        GridMap {
            map: HashMap::default(),
            max_x: 0,
            max_y: 0,
        }
    }

    pub fn create_from_image(image: &Image) -> Self {
        let mut map = GridMap::new();
        let size = image.size();
        let mut data = image.data.clone();
        for y in 0..size.y as u8 {
            for x in (0..size.x as u8).rev() {
                let _a = data.pop().unwrap();
                let b =  data.pop().unwrap();
                let g = data.pop().unwrap();
                let r = data.pop().unwrap();

                if r > 0 {
                    map.add(x, y, GridType::Unpassable);
                    continue;
                }

                if g > 0 {
                    map.add(x, y, GridType::Breakable);
                    continue;
                }

                if b > 0 {
                    map.add(x, y, GridType::Passable);
                    continue;
                }
            }
        }
        map
    }

    pub fn add(&mut self, x: u8, y: u8, grid_type: GridType) {
        if self.map.contains_key(&(x, y)) {
            return;
        }

        self.max_x = u8::max(self.max_x, x);
        self.max_y = u8::max(self.max_y, y);
        self.map.insert((x, y), grid_type);
    }

    pub fn get(&self, x: u8, y: u8) -> Option<&GridType> {
        self.map.get(&(x, y))
    }

    pub fn get_map(&self) -> &HashMap<(u8, u8), GridType> {
        &self.map
    }

    pub fn get_max(&self) -> (u8, u8) {
        (self.max_x, self.max_y)
    }
}
