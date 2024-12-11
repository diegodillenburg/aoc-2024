use day9::filesystem::Filesystem;

fn main() {
    let mut filesystem = Filesystem::new("input_mini.txt");
    filesystem.info();
    filesystem.reorder_blocks();
    println!("Checksum: {}", filesystem.calculate_checksum());
    let mut filesystem = Filesystem::new("input.txt");
    filesystem.info();
    filesystem.reorder_files();
    println!("Checksum: {}", filesystem.calculate_checksum());
}
