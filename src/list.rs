struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{head: None, len: 0}
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, val: T) {
        self.len += 1;
        let old_head = std::mem::take(&mut self.head);
        self.head = Some(Box::new(Node{data: val, next: old_head}));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let Some(head) = std::mem::take(&mut self.head) else {
            return None;
        };
        self.len -= 1;
        self.head = head.next;
        Some(head.data)
    }

    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    pub fn clear(&mut self) {
        self.head = None;
        self.len = 0;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { curr: self.head.as_deref() }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { curr: self.head.as_deref_mut() }
    }
}

pub struct Iter<'a, T> {
    curr: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = self.curr else {
            return None;
        };
        self.curr = node.next.as_deref();
        Some(&node.data)
    }
}

pub struct IterMut<'a, T> {
    curr: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(node) = std::mem::take(&mut self.curr) else {
            return None;
        };
        self.curr = node.next.as_deref_mut();
        Some(&mut node.data)
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_new() {
        let list: List<i32> = List::new();
        assert_eq!(list.len(), 0);
        assert!(list.peek_front().is_none());
    }

    #[test]
    fn test_push_pop() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());

        list.push_front(1);
        assert_eq!(list.peek_front(), Some(&1));

        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));

        list.pop_front();
        assert_eq!(list.peek_front(), Some(&1));

        list.pop_front();
        assert!(list.peek_front().is_none());
    }

    #[test]
    fn test_len() {
        let mut list = List::new();
        assert_eq!(list.len(), 0);

        list.push_front(1);
        assert_eq!(list.len(), 1);

        list.push_front(2);
        assert_eq!(list.len(), 2);

        list.pop_front();
        assert_eq!(list.len(), 1);

        list.pop_front();
        assert_eq!(list.len(), 0);

        list.pop_front(); // Pop from empty list
        assert_eq!(list.len(), 0);
    }
    #[test]
    fn test_peek_front_mut() {
        let mut list = List::new();
        assert!(list.peek_front_mut().is_none());

        list.push_front(1);
        *list.peek_front_mut().unwrap() = 10;
        assert_eq!(list.peek_front(), Some(&10));

        list.push_front(2);
        *list.peek_front_mut().unwrap() = 20;
        assert_eq!(list.peek_front(), Some(&20));

        list.pop_front();
        *list.peek_front_mut().unwrap() = 100;
        assert_eq!(list.peek_front(), Some(&100));
    }
    #[test]
    fn test_clear() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);

        list.clear();
        assert_eq!(list.len(), 0);
        assert!(list.peek_front().is_none());
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);

        // Test with an empty list
        let empty_list: List<i32> = List::new();
        let mut empty_iter = empty_list.iter();
        assert_eq!(empty_iter.next(), None);

        // Test with a single element
        let mut single_list = List::new();
        single_list.push_front(42);
        let mut single_iter = single_list.iter();
        assert_eq!(single_iter.next(), Some(&42));
        assert_eq!(single_iter.next(), None);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        for item in list.iter_mut() {
            *item *= 2;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), None);

        // Test with an empty list
        let mut empty_list: List<i32> = List::new();
        for item in empty_list.iter_mut() {
            *item *= 2; // Should not panic or do anything
        }
        assert_eq!(empty_list.len(), 0);

        // Test with a single element
        let mut single_list = List::new();
        single_list.push_front(42);
        for item in single_list.iter_mut() {
            *item += 1;
        }
        let mut single_iter = single_list.iter();
        assert_eq!(single_iter.next(), Some(&43));
        assert_eq!(single_iter.next(), None);
    }
}