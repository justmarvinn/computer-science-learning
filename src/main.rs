mod bst;
use bst::BinarySearchTree;

fn main() {
    let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
    tree.insert(42);
    tree.insert(39);
    tree.insert(55);
    tree.insert(40);
    tree.insert(777);
    tree.insert(50);
    tree.insert(15);
    tree.insert(41);
    tree.insert(7);
    tree.insert(52);
    dbg!(tree.count());
    dbg!(tree.infix_traverse());
    dbg!(tree.prefix_traverse());
    dbg!(tree.postfix_traverse());

    dbg!(tree.min());
    dbg!(tree.max());
    println!("{}", tree);
    tree.remove(42);
    tree.remove(39);
    tree.remove(50);
    tree.remove(52);
    tree.remove(55);
    tree.remove(777);
    tree.remove(42);
    tree.remove(40).unwrap();
    tree.remove(41);
    tree.remove(15);
    tree.remove(7);
    println!("=================");
    println!("{}", tree);
}
