use rand::{self, Rng, rngs::ThreadRng};

#[derive(Clone)]
enum Tile {
    Wall,
    Hallway,
    Room,
}

pub struct Dungeon {
    cells: Vec<Vec<Tile>>,
}

impl Dungeon {
    pub fn print_to_terminal(&self) {
        for x in &self.cells {
            let mut line = "".to_string();
            for tile in x {
                match tile {
                    Tile::Wall => line = line + "x",
                    Tile::Hallway => line = line + "o",
                    Tile::Room => line = line + "O",
                }
            }
            println!("{}", line);
        }
    }
}

pub struct PreDungeon {
    rooms: Vec<Room>,
    hallways: Vec<Hallway>,
    length: u32,
    height: u32,
}

impl PreDungeon {
    pub fn init_dungeon(length: u32, height: u32) -> Self {
        PreDungeon {
            rooms: Vec::new(),
            hallways: Vec::new(),
            length,
            height,
        }
    }

    pub fn generate_rooms(
        &mut self,
        min_rooms: u32,
        max_rooms: u32,
        min_room_side_length: u32,
        max_room_side_length: u32,
        retries_per_room: u32,
    ) {
        let mut rng = rand::rng();
        let room_count = rng.random_range(min_rooms..max_rooms);
        for _ in 0..room_count {
            for _ in 0..retries_per_room {
                let new_room = Room {
                    loc: PreDungeon::pick_point(0, self.length - 1, 0, self.height - 1, &mut rng),
                    length: rng.random_range(min_room_side_length..max_room_side_length),
                    height: rng.random_range(min_room_side_length..max_room_side_length),
                };
                //println!(
                //    "Loc: {},{}|height: {}|length: {}",
                //    new_room.loc.x, new_room.loc.y, new_room.height, new_room.length
                //);
                if self.check_potantial_room(&new_room) {
                    self.rooms.push(new_room);
                    break;
                }
                //println!("Failed to place new room, attempt: {}", c + 1);
            }
        }
    }

    fn check_potantial_room(&self, new_room: &Room) -> bool {
        let space = 3;
        if self.rooms.len() == 0
            && new_room.loc.x + new_room.length + space < self.length - 1
            && new_room.loc.y + new_room.height + space < self.height - 1
        {
            return true;
        }
        for room in &self.rooms {
            let l1 = Coord {
                x: room.loc.x,
                y: room.loc.y + room.height,
            };
            let r1 = Coord {
                x: room.loc.x + room.length,
                y: room.loc.y,
            };
            let l2 = Coord {
                x: new_room.loc.x,
                y: new_room.loc.y + new_room.height,
            };
            let r2 = Coord {
                x: new_room.loc.x + new_room.length,
                y: new_room.loc.y,
            };
            if (l1.x > r2.x + space || l2.x + space > r1.x)
                && (r1.y > l2.y + space || r2.y + space > l1.y)
                && (new_room.loc.x + new_room.length + space < self.length - 1
                    && new_room.loc.y + new_room.height + space < self.height - 1)
            {
                return true;
            }
        }
        return false;
    }

    fn generate_hallways(&self) {}

    pub fn render_to_dungeon(&self) -> Dungeon {
        let mut tiles = vec![vec![Tile::Wall; self.height as usize]; self.length as usize];
        for room in &self.rooms {
            println!(
                "Loc: {},{} | height: {} | length: {}",
                room.loc.x, room.loc.y, room.height, room.length
            );
            for x in room.loc.x..room.loc.x + room.length {
                for y in room.loc.y..room.loc.y + room.height {
                    tiles[x as usize][y as usize] = Tile::Room;
                }
            }
        }
        return Dungeon { cells: tiles };
    }

    fn pick_point(x_min: u32, x_max: u32, y_min: u32, y_max: u32, rand: &mut ThreadRng) -> Coord {
        Coord {
            x: rand.random_range(x_min..x_max),
            y: rand.random_range(y_min..y_max),
        }
    }
}

struct Room {
    loc: Coord,
    length: u32,
    height: u32,
}

struct Hallway {}

struct Coord {
    x: u32,
    y: u32,
}
