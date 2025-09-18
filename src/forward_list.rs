pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct ForwardList<T> {
    next: Option<Box<Node<T>>>,
}

pub struct ForwardListIter<'a, T> {
    current: Option<&'a Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn insert_after(&mut self, value: T) -> &mut Box<Node<T>> {
        let new_node = Box::new(Node {
            value: value,
            next: None,
        });
        self.next = Some(new_node);
        // debug_assert!(!self.next.is_none());
        return self.next.as_mut().unwrap();
    }
}

impl<T> ForwardList<T> {
    pub fn new() -> ForwardList<T> {
        ForwardList { next: None }
    }

    pub fn swap(&mut self, other: &mut ForwardList<T>) {
        let mut temp = self.next.take();
        self.next = other.next.take();
        other.next = temp.take();
    }

    pub fn empty(&self) -> bool {
        return self.next.is_none();
    }

    pub fn front(&self) -> &Box<Node<T>> {
        self.next.as_ref().unwrap()
    }

    pub fn front_mut(&mut self) -> &mut Box<Node<T>> {
        self.next.as_mut().unwrap()
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value: value,
            next: self.next.take(),
        });

        self.next = Some(new_node);
    }

    pub fn pop_front(&mut self) {
        self.next = self.next.take().unwrap().next.take();
    }

    pub fn iter(&self) -> ForwardListIter<T> {
        ForwardListIter {
            current: self.next.as_ref(),
        }
    }
}

impl<'a, T> Iterator for ForwardListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node) => {
                let result = &node.value;
                self.current = node.next.as_ref();
                Some(result)
            }
            None => None,
        }
    }
}

#[macro_export]
macro_rules! make_forward_list {
    () => {{
        let mut list = ForwardList::new();
        list
    }};

    ( $x:expr ) => {{
        let mut list = ForwardList::new();
        list.push_front($x);
        list
    }};

    ( $x:expr, $($y:expr),+ ) => {{
        let mut list = make_forward_list!($x);
        let mut node = list.front_mut();
        $(
            node = node.insert_after($y);
        )+
        list
    }};
}
