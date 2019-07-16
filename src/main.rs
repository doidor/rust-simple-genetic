mod individual;
mod target;

use individual::Individual;

fn main() {
    let ind = Individual::new();

	println!("{:?}", ind);
}
