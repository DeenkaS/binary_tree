mod tree;
use std::vec;

use rand::Rng;

fn main() {
    let mut binary_tree  = tree::Tree::new();
    let mut vector: Vec<i32> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..10{
        vector.push(rng.gen_range(1..100));
    }

    for number in vector{
        binary_tree.add(number);
    }
}
