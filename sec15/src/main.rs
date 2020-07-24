extern crate sec15;
use sec15::{list_rc::ref_count, mybox::drop_mybox};

mod tree;
use tree::{tree, tree_weak};

mod circular_ref;
use circular_ref::circular_ref;

fn main() {
    drop_mybox();
    ref_count();
    circular_ref();
    tree();
    tree_weak();
}
