use strithms::data_structures::doubly_linked_list::DoublyLinkedList;

#[test]
fn test_new() {
    let list: DoublyLinkedList<i32> = DoublyLinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn test_push_front() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
}

// #[test]
// fn test_push_back() {
//     let mut list = DoublyLinkedList::new();
//     list.push_back(1);
//     list.push_back(2);
//     list.push_back(3);
//     list.print();
//     println!("{:?}", list.head);
//     assert_eq!(list.len(), 3);
//     assert!(!list.is_empty());
// }

#[test]
fn test_pop_front() {
    let mut list = DoublyLinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop_front(), Some(3));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.len(),1);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), None);
    assert!(list.is_empty());
}

// #[test]
// fn test_pop_back() {
//     let mut list = DoublyLinkedList::new();
//     list.push_back(1);
//     list.push_back(2);
//     list.push_back(3);
//     assert_eq!(list.pop_back(), Some(3));
//     assert_eq!(list.pop_back(), Some(2));
//     assert_eq!(list.pop_back(), Some(1));
//     assert_eq!(list.pop_back(), None);
//     assert!(list.is_empty());
//     list = DoublyLinkedList::from(&[1,2,3,4]);
//     list.print();
//     // assert_eq!(list.pop_front(), Some(1));
//     // assert_eq!(list.pop_back(), Some(2));
//     // assert_eq!(list.pop_back(), Some(3));
//     // assert_eq!(list.pop_back(), Some(4));
// }

#[test]
fn test_from_slice() {
    let slice = [1, 2, 3];
    let list = DoublyLinkedList::from(&slice);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
}

#[test]
fn test_print() {
    let list = DoublyLinkedList::from(&[1,2,3]);
    list.print(); // This will print: head -> 1 <-> 2 <-> 3 <-> tail

    let mut list2 = DoublyLinkedList::new();
    list2.push_front(1);
    list2.push_front(2);
    list2.push_front(3);
    list2.print();
}


#[test]
fn test_merge() {

    let mut list1 = DoublyLinkedList::new();
    list1.push_front(1);
    list1.push_front(2);

    let mut list2 = DoublyLinkedList::new();
    list2.push_front(3);
    list2.push_front(4);

    let merged_list = list1.margin(&list2);
    merged_list.print();
    assert_eq!(merged_list.len(), 4);
    assert!(!merged_list.is_empty());

    let mut current = merged_list.head;
    assert_eq!(current.as_ref().unwrap().data, 1);
    current = current.unwrap().next;
    assert_eq!(current.as_ref().unwrap().data, 2);
    current = current.unwrap().next;
    assert_eq!(current.as_ref().unwrap().data, 3);
    current = current.unwrap().next;
    assert_eq!(current.as_ref().unwrap().data, 4);

    let list3 = DoublyLinkedList::from(&[1,2,2,4]);
    let list4 = DoublyLinkedList::from(&[6,8]);
    let new_list = list3.margin(&list4);
    assert_eq!(new_list.len(), 6);
}