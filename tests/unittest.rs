#[cfg(test)]
mod unittest {
    use forward_list::ForwardList;
    use forward_list::make_forward_list;

    #[test]
    fn test_constructor1() {
        let list: ForwardList<i32> = make_forward_list![];
        assert!(list.empty());
    }

    #[test]
    fn test_constructor2() {
        let list = make_forward_list![1, 2, 3];
        assert!(!list.empty());
    }

    #[test]
    fn test_iter() {
        let list = make_forward_list![1, 2, 3];
        for (i, &item) in list.iter().enumerate() {
            assert_eq!(i + 1, item);
        }
    }

    #[test]
    fn test_pop_front() {
        let mut list = make_forward_list![1, 2, 3];
        list.pop_front();
        assert_eq!(*list.front().value(), 2);
    }

    #[test]
    fn test_swap() {
        let mut list1 = make_forward_list![1, 2, 3];
        let mut list2 = make_forward_list![4, 5, 6];
        list1.swap(&mut list2);
        assert_eq!(*list1.front().value(), 4);
        assert_eq!(*list2.front().value(), 1);
    }
}
