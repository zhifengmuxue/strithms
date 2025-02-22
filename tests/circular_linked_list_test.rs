use strithms::data_structures::circular_linked_list::CircularLinkedList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list: CircularLinkedList<i32> = CircularLinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_from() {
        let list = CircularLinkedList::from(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.head.as_ref().unwrap().data, 1);
        assert_eq!(list.tail.as_ref().unwrap().data, 3);
    }

    #[test]
    fn test_push_front() {
        let mut list = CircularLinkedList::new();
        list.push_front(1);
        assert_eq!(list.head.as_ref().unwrap().data, 1);
        assert_eq!(list.tail.as_ref().unwrap().data, 1);

        list.push_front(2);
        assert_eq!(list.head.as_ref().unwrap().data, 2);
        assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().data, 1);
        assert_eq!(list.tail.as_ref().unwrap().data, 1);
        assert_eq!(list.tail.as_ref().unwrap().next.as_ref().unwrap().data, 2);
    }

    #[test]
    fn test_pop_front() {
        let mut list = CircularLinkedList::from(&[1, 2, 3]);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_len() {
        let list = CircularLinkedList::from(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_is_empty() {
        let mut list = CircularLinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_print() {
        let list = CircularLinkedList::from(&[1, 2, 3, 1]);
        list.print(); // This will print: head -> 1 -> 2 -> 3 -> head
    }

}