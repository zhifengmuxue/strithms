use strithms::data_structures::array_list::ArrayList;
use strithms::data_structures::doubly_linked_list::DoublyLinkedList;
use strithms::data_structures::linked_list::LinkedList;

fn main() {
    // 使用 ArrayList
    let mut array_list = ArrayList::from(&[1, 2, 3]);
    array_list.print();
    println!("Locate the elem: {:?}", array_list.locate_elem(&1));
    println!("List length: {}", array_list.len());
    println!("First element: {:?}", array_list.get(0));
    println!("Remove element: {:?}", array_list.remove(1));
    println!("List length after removal: {}", array_list.len());

    // 使用 LinkedList
    let mut linked_list = LinkedList::from(&[1,2,3]);
    linked_list.print();
    println!("List length: {}", linked_list.len());
    println!("Is empty: {}", linked_list.is_empty());
    println!("Pop front: {:?}", linked_list.pop_front());
    linked_list.print();

    let mut doubly_linked_list = DoublyLinkedList::new();
    doubly_linked_list.push_front(1);
    doubly_linked_list.push_front(2);
    doubly_linked_list.push_front(3);
    
    doubly_linked_list.print();
    let doubly_linked_list2 = DoublyLinkedList::from(&[4,5,6]);
    doubly_linked_list2.print();

    let link = doubly_linked_list.margin(&doubly_linked_list2);
    link.print();
}

