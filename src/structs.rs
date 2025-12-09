use rand::{self, Rng};

enum Tile {
    Wall,
    Hallway,
    Room
}

struct Dungeon {
    cells: Vec<Vec<Tile>>
}

impl Dungeon {
    fn render_to_terminal() {
    
    }
}

struct PreDungeon {
    rooms: Vec<Room>,
    hallways: Vec<Hallway>,
    length: u32,
    height: u32
}

impl PreDungeon {
    fn init_dungeon(length: u32, height: u32) -> Self {
        PreDungeon {
            rooms: Vec::new(),
            hallways: Vec::new(),
            length,
            height
        }
    }

    fn generate_rooms(&self, min_rooms: u32, max_rooms: u32, min_room_side_length: u32, max_room_side_length: u32, retries_per_room: u32) {
        let mut rng = rand::rng();
        let room_count = rng.random_range(min_rooms..max_rooms);
        for i in 0..room_count {
            // Pick random point
        }
    }

    fn generate_hallways(&self) {

    }

    fn render_to_dungeon(&self) -> Dungeon {
        
    }
}

struct Room {
    loc: Coord,
    length: u32,
    height: u32
}

struct Hallway {

}

struct Coord {
    x: u32,
    y: u32
}
