use day12::farm::Farm;

fn main() {
    let mut farm = Farm::new("input_mini.txt");
    farm.populate();
    println!("Fence price: {}", farm.fence_price());
}
