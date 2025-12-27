mod structs;

fn main() {
    let mut dungeon = structs::PreDungeon::init_dungeon(50, 50);
    dungeon.generate_rooms(2, 10, 4, 15, 5);
    let temp = dungeon.render_to_dungeon();
    temp.print_to_terminal();
}
