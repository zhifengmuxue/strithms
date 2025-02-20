#[cfg(test)]
mod tests {
    use super::super::array_list::ArrayList;

    #[test]
    fn test_new() {
        let list: ArrayList<i32> = ArrayList::new();
        assert!(list.is_empty());
    }

    #[test]
    fn test_from_slice() {
        let list = ArrayList::from(&[1, 2, 3, 4]);
        assert_eq!(list.len(), 4);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
        assert_eq!(list.get(3), Some(&4));
    }

    #[test]
    fn test_push_and_len() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_pop() {
        let mut list = ArrayList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_remove() {
        let mut list = ArrayList::from(&[1,2,3]);
        assert_eq!(list.remove(1), Some(2));
        assert_eq!(list.len(), 2);
        assert_eq!(list.get(1), Some(&3));
    }

    #[test]
    fn test_update() {
        let mut list = ArrayList::from(&[1,2,3]);
        assert_eq!(list.update(1, 4), Some(2));
        assert_eq!(list.get(1), Some(&4));
    }

    #[test]
    fn test_get() {
        let list = ArrayList::from(&[1,2,3]);
        assert_eq!(list.get(0), Some(&1));
        assert_eq!(list.get(1), Some(&2));
        assert_eq!(list.get(2), Some(&3));
    }

    #[test]
    fn test_is_empty() {
        let mut list = ArrayList::new();
        assert!(list.is_empty());
        list.push(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_locate_elem() {
        let list = ArrayList::from(&[1,2,3]);
        assert_eq!(list.locate_elem(&2), Some(1));
        assert_eq!(list.locate_elem(&4), None);
    }

    #[test]
    fn test_clear() {
        let mut list = ArrayList::from(&[1,2,3]);
        list.clear();
        assert!(list.is_empty());
    }

    #[test]
    fn test_print() {
        let list = ArrayList::from(&[1,2,3]);
        list.print(); 
    }

    #[test]
    fn test_margin() {
        let list = ArrayList::from(&[1,2,3]);
        let another_list = ArrayList::from(&[4,5]);
        let merged_list = list.margin(another_list);
        assert_eq!(merged_list.len(), 5);
        assert_eq!(merged_list.get(0), Some(&1));
        assert_eq!(merged_list.get(1), Some(&2));
        assert_eq!(merged_list.get(2), Some(&3));
        assert_eq!(merged_list.get(3), Some(&4));
        assert_eq!(merged_list.get(4), Some(&5));
    }
}