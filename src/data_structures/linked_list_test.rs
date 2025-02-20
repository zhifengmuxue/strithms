#[cfg(test)]
mod tests {
    use super::super::linked_list::LinkedList;

    #[test]
    fn test_new() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_from() {
        let mut list = LinkedList::from(&[1, 2, 3]);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
    }

    #[test]
    fn test_push_front_and_len() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_is_empty() {
        let mut list = LinkedList::new();
        assert!(list.is_empty());
        list.push_front(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_print() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.print(); // This will print the list to the console
    }

    #[test]
    fn test_reverse() {
        let mut list = LinkedList::from(&[1, 2, 3]);
        list.reverse();
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn test_merge() {
        let list1 = LinkedList::from(&[1, 2, 3]);
        let list2 = LinkedList::from(&[4, 5, 6]);
        let mut merged_list = list1.merge(list2);
        assert_eq!(merged_list.len(), 6);
        assert_eq!(merged_list.pop_front(), Some(1));
        assert_eq!(merged_list.pop_front(), Some(2));
        assert_eq!(merged_list.pop_front(), Some(3));
        assert_eq!(merged_list.pop_front(), Some(4));
        assert_eq!(merged_list.pop_front(), Some(5));
        assert_eq!(merged_list.pop_front(), Some(6));
    }
}