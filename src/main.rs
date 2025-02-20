use strithms::data_structures::array_list::ArrayList;

fn main() {
    let mut list = ArrayList::from(&[1,2,3]);
    list.print();
    println!("Locate the elem: {:?}", list.locate_elem(&1));
    println!("List length: {}", list.len());
    println!("First element: {:?}", list.get(0));
    println!("Remove element: {:?}", list.remove(1));
    println!("List length after removal: {}", list.len())
}
