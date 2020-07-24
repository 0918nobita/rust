extern crate sec15;

mod tree;
use tree::{tree, tree_weak};

mod circular_ref;
use circular_ref::circular_ref;

fn main() {
    circular_ref();
    tree();
    tree_weak();
}
