use day9::filesystem::Filesystem;

fn main() {
    let mut filesystem = Filesystem::new("input.txt");
    println!("Filesystem:\n- Size: {}\n- Free space: {}", filesystem.size(), filesystem.free_space());
    filesystem.reorder_blocks();
    println!("Checksum: {}", filesystem.calculate_checksum());
    let mut filesystem = Filesystem::new("input.txt");
    println!("Filesystem:\n- Size: {}\n- Free space: {}", filesystem.size(), filesystem.free_space());
    filesystem.reorder_files();
    println!("Checksum: {}", filesystem.calculate_checksum());
}
