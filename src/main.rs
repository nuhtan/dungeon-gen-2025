mod structs;

fn main() {
    let mut dungeon = structs::PreDungeon::init_dungeon(25, 25);
    dungeon.generate_rooms(4, 5, 4, 16, 5);
    let temp = dungeon.render_to_dungeon();
    temp.print_to_terminal();
}
