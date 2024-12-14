use day12::farm::Farm;

fn main() {
    let mut farm = Farm::new("input.txt");
    farm.populate();
    println!("Fence price:\n- By perimeter: {}\n- By edge count: {}", farm.fence_price(false), farm.fence_price(true));

    for plot in farm.plots {
        println!("Farm plot {} has {} edges", plot.kind, plot.safe_edges());
    }
}
