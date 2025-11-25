struct PreDungeon {
    rooms: Vec<Room>,
    hallways: Vec<>,
    length: u32,
    height: u32
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
